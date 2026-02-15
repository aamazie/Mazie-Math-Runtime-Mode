 // mazie_runtime_mode.rs
// Mazie "runtime mode" context: the semantics live in MazieRuntime,
// not scattered across operator overloads.

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieMode {
    pub div0_identity: bool,
}

impl Default for MazieMode {
    fn default() -> Self {
        Self { div0_identity: true }
    }
}

/// A number tagged with a specific MazieRuntime instance via its mode copy.
/// (In a larger system you could store an Arc<MazieRuntime> instead,
/// but this stays single-file and lightweight.)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieNum {
    value: f64,
    mode: MazieMode,
}

impl MazieNum {
    pub fn unwrap(self) -> f64 {
        self.value
    }
}

impl fmt::Display for MazieNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// The "runtime mode" abstraction.
/// This is the API you use to perform arithmetic under chosen semantics.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieRuntime {
    pub mode: MazieMode,
    pub name: &'static str,
}

impl MazieRuntime {
    /// Default Mazie runtime: identity-preserving division by zero enabled.
    pub fn mazie() -> Self {
        Self {
            mode: MazieMode { div0_identity: true },
            name: "MazieRuntime::mazie",
        }
    }

    /// Strict runtime: division by zero is a hard error (panic).
    pub fn strict() -> Self {
        Self {
            mode: MazieMode { div0_identity: false },
            name: "MazieRuntime::strict",
        }
    }

    /// Construct a MazieNum bound to this runtime's mode.
    pub fn n(&self, x: f64) -> MazieNum {
        MazieNum { value: x, mode: self.mode }
    }

    /// Ensure the number is under this runtime (rebind semantics if needed).
    pub fn bind(&self, x: MazieNum) -> MazieNum {
        MazieNum { value: x.value, mode: self.mode }
    }

    // ---- Runtime arithmetic functions ----

    pub fn add(&self, a: MazieNum, b: MazieNum) -> MazieNum {
        let a = self.bind(a);
        let b = self.bind(b);
        MazieNum { value: a.value + b.value, mode: self.mode }
    }

    pub fn sub(&self, a: MazieNum, b: MazieNum) -> MazieNum {
        let a = self.bind(a);
        let b = self.bind(b);
        MazieNum { value: a.value - b.value, mode: self.mode }
    }

    pub fn mul(&self, a: MazieNum, b: MazieNum) -> MazieNum {
        let a = self.bind(a);
        let b = self.bind(b);
        MazieNum { value: a.value * b.value, mode: self.mode }
    }

    pub fn neg(&self, a: MazieNum) -> MazieNum {
        let a = self.bind(a);
        MazieNum { value: -a.value, mode: self.mode }
    }

    /// Division under runtime semantics:
    /// - If divisor == 0 and div0_identity == true: a / 0 => a
    /// - If divisor == 0 and div0_identity == false: panic
    pub fn div(&self, a: MazieNum, b: MazieNum) -> MazieNum {
        let a = self.bind(a);
        let b = self.bind(b);

        if b.value == 0.0 {
            if self.mode.div0_identity {
                return MazieNum { value: a.value, mode: self.mode };
            }
            panic!("division by zero ({}; div0_identity=false)", self.name);
        }

        MazieNum { value: a.value / b.value, mode: self.mode }
    }

    /// Convenience overloads (so you can pass raw f64 too)
    pub fn addf(&self, a: MazieNum, b: f64) -> MazieNum { self.add(a, self.n(b)) }
    pub fn subf(&self, a: MazieNum, b: f64) -> MazieNum { self.sub(a, self.n(b)) }
    pub fn mulf(&self, a: MazieNum, b: f64) -> MazieNum { self.mul(a, self.n(b)) }
    pub fn divf(&self, a: MazieNum, b: f64) -> MazieNum { self.div(a, self.n(b)) }
}

fn main() {
    // Two runtimes
    let rt = MazieRuntime::mazie();   // identity div0
    let strict = MazieRuntime::strict();

    let x = rt.n(5.0);
    let zero = rt.n(0.0);

    println!("Runtime: {}", rt.name);
    println!("rt.div(x, 0) => {}", rt.div(x, zero).unwrap()); // 5.0

    // Compose operations under runtime
    let y = rt.n(10.0);
    let out = rt.add(rt.divf(y, 2.0), rt.n(7.0));
    println!("rt.add(rt.divf(10,2), 7) => {}", out.unwrap()); // 12.0

    // Strict mode example (will panic if uncommented)
    let xs = strict.n(5.0);
    // let _ = strict.divf(xs, 0.0);
}
