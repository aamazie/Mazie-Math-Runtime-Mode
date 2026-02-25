use std::fmt;

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
