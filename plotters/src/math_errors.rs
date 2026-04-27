use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MathError {
    ValueOverflow,
    ValueUnderflow,
    NonFiniteCalculation,
    ValueOutOfRange,
    ZeroDivision,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::ValueOverflow => {
                write!(f, "value exceeds the target type's maximum")
            }
            MathError::ValueUnderflow => {
                write!(f, "value is below the target type's minimum")
            }
            MathError::NonFiniteCalculation => {
                write!(f, "calculation produced a non-finite value")
            }
            MathError::ValueOutOfRange => {
                write!(f, "value is out of range for the target type")
            }
            MathError::ZeroDivision => {
                write!(f, "attempted to divide by zero")
            }
        }
    }
}

impl std::error::Error for MathError {}
