// mazie_runtime_mode_v2.rs
//
// "Real" (production-shaped) version of your idea:
//
// ✅ No panics in library path (uses Result)
// ✅ Runtime identity is enforced (no silent rebinding)
// ✅ Explicit policy for div-by-zero (+0.0 and -0.0)
// ✅ Explicit behavior for NaN/Infinity (policy-controlled)
// ✅ Optional instrumentation hook (counts/logs div0 events)
//
// This is still single-file, but the structure is what reviewers expect.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

// ----------------------------- Errors -----------------------------

#[derive(Debug, Clone)]
pub enum MazieError {
    DivisionByZero {
        runtime: &'static str,
        dividend: f64,
        divisor: f64,
    },
    RuntimeMismatch {
        lhs_runtime: u64,
        rhs_runtime: u64,
        op: &'static str,
    },
    NonFinite {
        runtime: &'static str,
        op: &'static str,
        value: f64,
    },
}

impl fmt::Display for MazieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MazieError::DivisionByZero { runtime, dividend, divisor } => {
                write!(f, "division by zero in {runtime}: {dividend} / {divisor}")
            }
            MazieError::RuntimeMismatch { lhs_runtime, rhs_runtime, op } => {
                write!(f, "runtime mismatch during {op}: lhs={lhs_runtime} rhs={rhs_runtime}")
            }
            MazieError::NonFinite { runtime, op, value } => {
                write!(f, "non-finite value in {runtime} during {op}: {value}")
            }
        }
    }
}

pub type MazieResult<T> = Result<T, MazieError>;

// ----------------------------- Policy -----------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Div0Policy {
    /// a / 0 => a (your Mazie identity semantics)
    Identity,
    /// a / 0 => error
    Error,
    /// a / 0 => IEEE-754 (inf, -inf, NaN)
    Ieee,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NonFinitePolicy {
    /// Allow NaN/Inf to flow through (typical numeric code)
    Allow,
    /// Reject any NaN/Inf inputs/outputs for ops (safety-critical)
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieMode {
    pub div0: Div0Policy,
    pub non_finite: NonFinitePolicy,
}

impl Default for MazieMode {
    fn default() -> Self {
        Self {
            div0: Div0Policy::Identity,
            non_finite: NonFinitePolicy::Allow,
        }
    }
}

// ----------------------------- Telemetry -----------------------------

pub trait MazieHook: Send + Sync {
    fn on_div0(&self, runtime: &'static str, dividend: f64, divisor: f64);
}

/// Simple built-in hook you can use in demos/tests.
pub struct Div0Counter {
    pub hits: AtomicU64,
}
impl Div0Counter {
    pub fn new() -> Self {
        Self { hits: AtomicU64::new(0) }
    }
    pub fn load(&self) -> u64 {
        self.hits.load(Ordering::Relaxed)
    }
}
impl MazieHook for Div0Counter {
    fn on_div0(&self, _runtime: &'static str, _dividend: f64, _divisor: f64) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }
}

// ----------------------------- Runtime + Number -----------------------------
//
// Important: MazieNum carries a runtime id.
// Mixed-runtime operations are rejected unless you explicitly convert.
//

static NEXT_RUNTIME_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct MazieRuntime {
    id: u64,
    pub name: &'static str,
    pub mode: MazieMode,
    hook: Option<Arc<dyn MazieHook>>,
}

impl MazieRuntime {
    pub fn new(name: &'static str, mode: MazieMode) -> Arc<Self> {
        Arc::new(Self {
            id: NEXT_RUNTIME_ID.fetch_add(1, Ordering::Relaxed),
            name,
            mode,
            hook: None,
        })
    }

    pub fn with_hook(mut self: Arc<Self>, hook: Arc<dyn MazieHook>) -> Arc<Self> {
        // This is a convenience: clone into a new runtime instance with same id is not ideal.
        // So instead we build a new runtime with a new id.
        // In a multi-file crate, you'd make runtime mutable behind Arc<Mutex<_>> or builder pattern.
        let mode = self.mode;
        let name = self.name;
        let rt = MazieRuntime::new(name, mode);
        // SAFETY: we just created it
        unsafe {
            let ptr = Arc::as_ptr(&rt) as *mut MazieRuntime;
            (*ptr).hook = Some(hook);
        }
        rt
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    // Constructors
    pub fn mazie() -> Arc<Self> {
        Self::new(
            "MazieRuntime::mazie",
            MazieMode { div0: Div0Policy::Identity, non_finite: NonFinitePolicy::Allow },
        )
    }

    pub fn strict() -> Arc<Self> {
        Self::new(
            "MazieRuntime::strict",
            MazieMode { div0: Div0Policy::Error, non_finite: NonFinitePolicy::Error },
        )
    }

    pub fn ieee() -> Arc<Self> {
        Self::new(
            "MazieRuntime::ieee",
            MazieMode { div0: Div0Policy::Ieee, non_finite: NonFinitePolicy::Allow },
        )
    }

    // Bind / convert explicitly
    pub fn n(self: &Arc<Self>, x: f64) -> MazieNum {
        MazieNum { value: x, runtime_id: self.id }
    }

    /// Explicit conversion: take the numeric value and retag it to this runtime.
    /// This is the only way to "rebind" across semantics.
    pub fn convert(self: &Arc<Self>, x: MazieNum) -> MazieNum {
        MazieNum { value: x.value, runtime_id: self.id }
    }

    fn require_same_runtime(&self, a: MazieNum, b: MazieNum, op: &'static str) -> MazieResult<()> {
        if a.runtime_id != self.id || b.runtime_id != self.id {
            return Err(MazieError::RuntimeMismatch {
                lhs_runtime: a.runtime_id,
                rhs_runtime: b.runtime_id,
                op,
            });
        }
        Ok(())
    }

    fn check_finite_in(&self, op: &'static str, v: f64) -> MazieResult<()> {
        if self.mode.non_finite == NonFinitePolicy::Error && !v.is_finite() {
            return Err(MazieError::NonFinite { runtime: self.name, op, value: v });
        }
        Ok(())
    }

    fn check_finite_out(&self, op: &'static str, v: f64) -> MazieResult<()> {
        self.check_finite_in(op, v)
    }

    // ---- Arithmetic (Result-returning) ----

    pub fn add(&self, a: MazieNum, b: MazieNum) -> MazieResult<MazieNum> {
        self.require_same_runtime(a, b, "add")?;
        self.check_finite_in("add(a)", a.value)?;
        self.check_finite_in("add(b)", b.value)?;
        let out = a.value + b.value;
        self.check_finite_out("add(out)", out)?;
        Ok(MazieNum { value: out, runtime_id: self.id })
    }

    pub fn sub(&self, a: MazieNum, b: MazieNum) -> MazieResult<MazieNum> {
        self.require_same_runtime(a, b, "sub")?;
        self.check_finite_in("sub(a)", a.value)?;
        self.check_finite_in("sub(b)", b.value)?;
        let out = a.value - b.value;
        self.check_finite_out("sub(out)", out)?;
        Ok(MazieNum { value: out, runtime_id: self.id })
    }

    pub fn mul(&self, a: MazieNum, b: MazieNum) -> MazieResult<MazieNum> {
        self.require_same_runtime(a, b, "mul")?;
        self.check_finite_in("mul(a)", a.value)?;
        self.check_finite_in("mul(b)", b.value)?;
        let out = a.value * b.value;
        self.check_finite_out("mul(out)", out)?;
        Ok(MazieNum { value: out, runtime_id: self.id })
    }

    pub fn neg(&self, a: MazieNum) -> MazieResult<MazieNum> {
        if a.runtime_id != self.id {
            return Err(MazieError::RuntimeMismatch {
                lhs_runtime: a.runtime_id,
                rhs_runtime: self.id,
                op: "neg",
            });
        }
        self.check_finite_in("neg(a)", a.value)?;
        let out = -a.value;
        self.check_finite_out("neg(out)", out)?;
        Ok(MazieNum { value: out, runtime_id: self.id })
    }

    /// Division with explicit policies:
    /// - Div0Policy::Identity => a / ±0.0 => a  (and records hook)
    /// - Div0Policy::Error    => error
    /// - Div0Policy::Ieee     => IEEE-754 a / ±0.0 (inf/-inf/NaN)
    ///
    /// Note: We treat both +0.0 and -0.0 as "zero" for Identity/Error, but we still pass
    /// the actual divisor to hooks/errors so sign is visible.
    pub fn div(&self, a: MazieNum, b: MazieNum) -> MazieResult<MazieNum> {
        self.require_same_runtime(a, b, "div")?;
        self.check_finite_in("div(a)", a.value)?;
        self.check_finite_in("div(b)", b.value)?;

        if b.value == 0.0 {
            if let Some(h) = &self.hook {
                h.on_div0(self.name, a.value, b.value);
            }
            match self.mode.div0 {
                Div0Policy::Identity => {
                    // a / 0 => a
                    return Ok(MazieNum { value: a.value, runtime_id: self.id });
                }
                Div0Policy::Error => {
                    return Err(MazieError::DivisionByZero {
                        runtime: self.name,
                        dividend: a.value,
                        divisor: b.value,
                    });
                }
                Div0Policy::Ieee => {
                    let out = a.value / b.value; // IEEE behavior
                    self.check_finite_out("div(out)", out)?;
                    return Ok(MazieNum { value: out, runtime_id: self.id });
                }
            }
        }

        let out = a.value / b.value;
        self.check_finite_out("div(out)", out)?;
        Ok(MazieNum { value: out, runtime_id: self.id })
    }

    // Convenience with raw f64 (still tags it to *this* runtime)
    pub fn addf(&self, a: MazieNum, b: f64) -> MazieResult<MazieNum> {
        self.add(a, MazieNum { value: b, runtime_id: self.id })
    }
    pub fn subf(&self, a: MazieNum, b: f64) -> MazieResult<MazieNum> {
        self.sub(a, MazieNum { value: b, runtime_id: self.id })
    }
    pub fn mulf(&self, a: MazieNum, b: f64) -> MazieResult<MazieNum> {
        self.mul(a, MazieNum { value: b, runtime_id: self.id })
    }
    pub fn divf(&self, a: MazieNum, b: f64) -> MazieResult<MazieNum> {
        self.div(a, MazieNum { value: b, runtime_id: self.id })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieNum {
    value: f64,
    runtime_id: u64,
}

impl MazieNum {
    pub fn unwrap(self) -> f64 {
        self.value
    }
    pub fn runtime_id(self) -> u64 {
        self.runtime_id
    }
}

impl fmt::Display for MazieNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ----------------------------- Demo -----------------------------

fn main() -> MazieResult<()> {
    let counter = Arc::new(Div0Counter::new());

    let rt = MazieRuntime::mazie();
    let rt = rt.with_hook(counter.clone()); // counts div0 events

    let strict = MazieRuntime::strict();
    let ieee = MazieRuntime::ieee();

    // Mazie identity div0
    let x = rt.n(5.0);
    let z = rt.n(0.0);
    let out = rt.div(x, z)?;
    println!("{} => rt.div(5, 0) = {}", rt.name, out.unwrap());

    // Composition stays clean
    let y = rt.n(10.0);
    let out2 = rt.add(rt.divf(y, 2.0)?, rt.n(7.0))?;
    println!("{} => rt.add(rt.divf(10,2), 7) = {}", rt.name, out2.unwrap());

    println!("div0 hits recorded = {}", counter.load());

    // Strict runtime returns error (no panic)
    let xs = strict.n(5.0);
    match strict.divf(xs, 0.0) {
        Ok(v) => println!("unexpected strict result: {}", v.unwrap()),
        Err(e) => println!("{} => strict.divf(5,0) error: {}", strict.name, e),
    }

    // IEEE runtime does IEEE-754
    let xi = ieee.n(5.0);
    let res = ieee.divf(xi, 0.0)?;
    println!("{} => ieee.divf(5,0) = {}", ieee.name, res.unwrap());

    // Runtime mismatch is rejected (this is the big “corporate-grade” piece)
    let a = rt.n(1.0);
    let b = strict.n(2.0);
    match rt.add(a, b) {
        Ok(_) => println!("unexpected: mixed-runtime add succeeded"),
        Err(e) => println!("mixed-runtime op rejected: {}", e),
    }

    // Explicit conversion is allowed
    let b2 = rt.convert(b);
    let ok = rt.add(a, b2)?;
    println!("explicit convert => {}", ok.unwrap());

    Ok(())
}
