use crate::MathError;
use std::error::Error;
/// The error produced by a drawing backend.
#[derive(Debug)]
pub enum DrawingErrorKind<E: Error + Send + Sync> {
    /// A drawing backend error
    DrawingError(E),
    /// A font rendering error
    FontError(Box<dyn Error + Send + Sync + 'static>),
    /// A mathematical operation has failed
    MathError(MathError),
}

impl<E: Error + Send + Sync> From<MathError> for DrawingErrorKind<E> {
    fn from(err: MathError) -> Self {
        DrawingErrorKind::MathError(err)
    }
}

impl<E: Error + Send + Sync> std::fmt::Display for DrawingErrorKind<E> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            DrawingErrorKind::DrawingError(e) => write!(fmt, "Drawing backend error: {}", e),
            DrawingErrorKind::FontError(e) => write!(fmt, "Font loading error: {}", e),
            DrawingErrorKind::MathError(e) => write!(fmt, "Math error: {}", e),
        }
    }
}

impl<E: Error + Send + Sync> Error for DrawingErrorKind<E> {}
