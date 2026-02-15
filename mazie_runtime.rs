// mazie_runtime.rs
// A tiny runtime-mode numeric wrapper:
// if div0_identity = true, then x / 0 => x

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieMode {
    pub div0_identity: bool,
}

impl Default for MazieMode {
    fn default() -> Self {
        Self { div0_identity: true }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MazieNum {
    pub value: f64,
    pub mode: MazieMode,
}

impl MazieNum {
    pub fn new(value: f64) -> Self {
        Self { value, mode: MazieMode::default() }
    }

    pub fn with_mode(value: f64, mode: MazieMode) -> Self {
        Self { value, mode }
    }

    pub fn unwrap(self) -> f64 {
        self.value
    }
}

// Convenience constructor
pub fn m(x: f64) -> MazieNum {
    MazieNum::new(x)
}

pub fn m_mode(x: f64, mode: MazieMode) -> MazieNum {
    MazieNum::with_mode(x, mode)
}

// Display as a plain number for easy printing
impl fmt::Display for MazieNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- Operators (MazieNum op MazieNum) ---
impl Add for MazieNum {
    type Output = MazieNum;
    fn add(self, rhs: MazieNum) -> MazieNum {
        MazieNum::with_mode(self.value + rhs.value, self.mode)
    }
}

impl Sub for MazieNum {
    type Output = MazieNum;
    fn sub(self, rhs: MazieNum) -> MazieNum {
        MazieNum::with_mode(self.value - rhs.value, self.mode)
    }
}

impl Mul for MazieNum {
    type Output = MazieNum;
    fn mul(self, rhs: MazieNum) -> MazieNum {
        MazieNum::with_mode(self.value * rhs.value, self.mode)
    }
}

impl Div for MazieNum {
    type Output = MazieNum;
    fn div(self, rhs: MazieNum) -> MazieNum {
        if rhs.value == 0.0 {
            if self.mode.div0_identity {
                return MazieNum::with_mode(self.value, self.mode); // x / 0 => x
            }
            panic!("division by zero (MazieMode.div0_identity=false)");
        }
        MazieNum::with_mode(self.value / rhs.value, self.mode)
    }
}

impl Neg for MazieNum {
    type Output = MazieNum;
    fn neg(self) -> MazieNum {
        MazieNum::with_mode(-self.value, self.mode)
    }
}

// --- Optional: allow MazieNum op f64 (ergonomics) ---
impl Add<f64> for MazieNum {
    type Output = MazieNum;
    fn add(self, rhs: f64) -> MazieNum {
        MazieNum::with_mode(self.value + rhs, self.mode)
    }
}

impl Sub<f64> for MazieNum {
    type Output = MazieNum;
    fn sub(self, rhs: f64) -> MazieNum {
        MazieNum::with_mode(self.value - rhs, self.mode)
    }
}

impl Mul<f64> for MazieNum {
    type Output = MazieNum;
    fn mul(self, rhs: f64) -> MazieNum {
        MazieNum::with_mode(self.value * rhs, self.mode)
    }
}

impl Div<f64> for MazieNum {
    type Output = MazieNum;
    fn div(self, rhs: f64) -> MazieNum {
        if rhs == 0.0 {
            if self.mode.div0_identity {
                return MazieNum::with_mode(self.value, self.mode);
            }
            panic!("division by zero (MazieMode.div0_identity=false)");
        }
        MazieNum::with_mode(self.value / rhs, self.mode)
    }
}

// Demo
fn main() {
    let x = m(5.0);
    let y = m(10.0);

    println!("m(5)/0 => {}", (x / 0.0).unwrap());     // 5.0
    println!("m(10)/2 => {}", (y / 2.0).unwrap());    // 5.0

    let strict = MazieMode { div0_identity: false };
    let xs = m_mode(5.0, strict);
    // Uncomment to see panic:
    // let _ = xs / 0.0;
}
