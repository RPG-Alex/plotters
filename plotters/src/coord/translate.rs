use plotters_backend::BackendCoord;
use std::ops::Deref;

/// The trait that translates some customized object to the backend coordinate
pub trait CoordTranslate {
    /// Specifies the object to be translated from
    type From;
    /// Specifies the error type to use
    type ErrorType;

    /// Translate the guest coordinate to the guest coordinate
    fn translate(&self, from: &Self::From) -> Result<BackendCoord, Self::ErrorType>;

    /// Get the Z-value of current coordinate
    fn depth(&self, _from: &Self::From) -> i32 {
        0
    }
}

impl<C, T> CoordTranslate for T
where
    C: CoordTranslate,
    T: Deref<Target = C>,
{
    type From = C::From;
    type ErrorType = C::ErrorType;
    fn translate(&self, from: &Self::From) -> Result<BackendCoord, Self::ErrorType> {
        self.deref().translate(from)
    }
}

/// The trait indicates that the coordinate system supports reverse transform
/// This is useful when we need an interactive plot, thus we need to map the event
/// from the backend coordinate to the logical coordinate
pub trait ReverseCoordTranslate: CoordTranslate {
    /// Reverse translate the coordinate from the drawing coordinate to the
    /// logic coordinate.
    /// Note: the return value is an option, because it's possible that the drawing
    /// coordinate isn't able to be represented in te guest coordinate system
    fn reverse_translate(&self, input: BackendCoord)
        -> Result<Option<Self::From>, Self::ErrorType>;
}
