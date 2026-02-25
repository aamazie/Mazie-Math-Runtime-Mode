use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use crate::{
    error::{MazieError, MazieResult},
    mode::{MazieMode, Div0Policy, NonFinitePolicy},
    num::MazieNum,
    hook::MazieHook,
};

static NEXT_RUNTIME_ID: AtomicU64 = AtomicU64::new(1);

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

    pub fn mazie() -> Arc<Self> {
        Self::new("MazieRuntime::mazie", MazieMode::default())
    }

    pub fn strict() -> Arc<Self> {
        Self::new(
            "MazieRuntime::strict",
            MazieMode {
                div0: Div0Policy::Error,
                non_finite: NonFinitePolicy::Error,
            },
        )
    }

    pub fn ieee() -> Arc<Self> {
        Self::new(
            "MazieRuntime::ieee",
            MazieMode {
                div0: Div0Policy::Ieee,
                non_finite: NonFinitePolicy::Allow,
            },
        )
    }

    pub fn with_hook(self: &Arc<Self>, hook: Arc<dyn MazieHook>) -> Arc<Self> {
        Arc::new(Self {
            id: NEXT_RUNTIME_ID.fetch_add(1, Ordering::Relaxed),
            name: self.name,
            mode: self.mode,
            hook: Some(hook),
        })
    }

    pub fn n(self: &Arc<Self>, x: f64) -> MazieNum {
        MazieNum { value: x, runtime_id: self.id }
    }

    fn ensure(&self, a: MazieNum, b: MazieNum, op: &'static str) -> MazieResult<()> {
        if a.runtime_id != self.id || b.runtime_id != self.id {
            return Err(MazieError::RuntimeMismatch {
                lhs_runtime: a.runtime_id,
                rhs_runtime: b.runtime_id,
                op,
            });
        }
        Ok(())
    }

    pub fn add(&self, a: MazieNum, b: MazieNum) -> MazieResult<MazieNum> {
        self.ensure(a, b, "add")?;
        Ok(MazieNum { value: a.value + b.value, runtime_id: self.id })
    }

    pub fn div(&self, a: MazieNum, b: MazieNum) -> MazieResult<MazieNum> {
        self.ensure(a, b, "div")?;

        if b.value == 0.0 {
            if let Some(h) = &self.hook {
                h.on_div0(self.name, a.value, b.value);
            }

            return match self.mode.div0 {
                Div0Policy::Identity => Ok(MazieNum { value: a.value, runtime_id: self.id }),
                Div0Policy::Error => Err(MazieError::DivisionByZero {
                    runtime: self.name,
                    dividend: a.value,
                    divisor: b.value,
                }),
                Div0Policy::Ieee => Ok(MazieNum {
                    value: a.value / b.value,
                    runtime_id: self.id,
                }),
            };
        }

        Ok(MazieNum { value: a.value / b.value, runtime_id: self.id })
    }
}
