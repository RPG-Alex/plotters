
use core::fmt;

#[derive(Debug)]
pub enum PlotError {
    RangeOverflow,
    RangeUnderFlow,
    NonFiniteProjection,
    CoordOutOfRange,
}

impl fmt::Display for PlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlotError::RangeOverflow => write!(f, "value range overflow"),
            PlotError::RangeUnderFlow => write!(f, "value range underflow"),
            PlotError::NonFiniteProjection => {
                write!(f, "projection produced a non-finite value")
            }
            PlotError::CoordOutOfRange => write!(f, "projected coordinate is out of range"),
        }
    }
}

impl std::error::Error for PlotError {}