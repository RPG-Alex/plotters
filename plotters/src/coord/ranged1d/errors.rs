
use core::fmt;

#[derive(Debug)]
pub enum Ranged1DError {
    RangeOverflow,
    RangeUnderFlow,
    NonFiniteProjection,
    CoordOutOfRange,
}

impl fmt::Display for Ranged1DError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ranged1DError::RangeOverflow => write!(f, "value range overflow"),
            Ranged1DError::RangeUnderFlow => write!(f, "value range underflow"),
            Ranged1DError::NonFiniteProjection => {
                write!(f, "projection produced a non-finite value")
            }
            Ranged1DError::CoordOutOfRange => write!(f, "projected coordinate is out of range"),
        }
    }
}

impl std::error::Error for Ranged1DError {}