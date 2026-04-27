use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlotError {
    ValueOverflow,
    ValueUnderflow,
    NonFiniteCalculation,
    ValueOutOfRange,
    ZeroDivision,
}

impl fmt::Display for PlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlotError::ValueOverflow => {
                write!(f, "value exceeds the target type's maximum")
            }
            PlotError::ValueUnderflow => {
                write!(f, "value is below the target type's minimum")
            }
            PlotError::NonFiniteCalculation => {
                write!(f, "calculation produced a non-finite value")
            }
            PlotError::ValueOutOfRange => {
                write!(f, "value is out of range for the target type")
            }
            PlotError::ZeroDivision => {
                write!(f, "attempted to divide by zero")
            }
        }
    }
}

impl std::error::Error for PlotError {}
