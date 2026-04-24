
use core::fmt;

#[derive(Debug)]
pub enum Ranged1DError {
    PixelRangeOverflow,
    ZeroSpan,
    NonFiniteProjection,
    CoordOutOfRange,
}

impl fmt::Display for Ranged1DError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ranged1DError::PixelRangeOverflow => write!(f, "pixel range overflow"),
            Ranged1DError::ZeroSpan => write!(f, "time span is zero"),
            Ranged1DError::NonFiniteProjection => {
                write!(f, "projection produced a non-finite value")
            }
            Ranged1DError::CoordOutOfRange => write!(f, "projected coordinate is out of range"),
        }
    }
}

impl std::error::Error for Ranged1DError {}