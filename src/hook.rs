use std::sync::atomic::{AtomicU64, Ordering};

pub trait MazieHook: Send + Sync {
    fn on_div0(&self, runtime: &'static str, dividend: f64, divisor: f64);
}

pub struct Div0Counter {
    hits: AtomicU64,
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
