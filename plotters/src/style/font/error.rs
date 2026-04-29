use plotters_backend::MathError;
use std::sync::Arc;
use font_kit::error::FontLoadingError;
use font_kit::error::GlyphLoadingError;

/// Unified error type for font operations.
#[derive(Debug, Clone)]
pub enum FontError {
    /// Failed to lock shared font state.
    LockError,
    /// Requested font family/style was not found.
    NoSuchFont(String, String),
    /// Failed to load font data.
    FontLoadError(Arc<FontLoadingError>),
    /// Failed to load or render a glyph.
    GlyphError(Arc<GlyphLoadingError>),
    /// Font handle was unavailable.
    FontHandleUnavailable,
    /// Failed to parse font face data.
    FaceParseError(String),
    /// Unknown font error.
    Unknown,
    /// Requested font is unavailable.
    FontUnavailable,
    /// Arithmetic failed during font layout.
    MathError(MathError),
}

impl std::fmt::Display for FontError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontError::LockError => write!(fmt, "Could not lock mutex"),
            FontError::NoSuchFont(family, style) => {
                write!(fmt, "No such font: {} {}", family, style)
            }
            FontError::FontLoadError(e) => write!(fmt, "Font loading error {}", e),
            FontError::GlyphError(e) => write!(fmt, "Glyph error {}", e),
            FontError::FontHandleUnavailable => write!(fmt, "Font handle is not available"),
            FontError::FaceParseError(e) => write!(fmt, "Font face parse error {}", e),
            FontError::Unknown => write!(fmt, "Unknown font error"),
            FontError::FontUnavailable => write!(fmt, "Font unavailable"),
            FontError::MathError(e) => write!(fmt, "Math error: {}", e),
        }
    }
}

impl From<MathError> for FontError {
    fn from(err: MathError) -> Self {
        FontError::MathError(err)
    }
}

impl std::error::Error for FontError {}