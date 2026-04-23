use std::convert::TryFrom;

use num_traits::{CheckedAdd, Float, NumCast, PrimInt};

pub(crate) fn float_to_integer_checked<F, I, E: Copy>(v: F, err: E) -> Result<I, E>
where
    F: Float + NumCast,
    I: PrimInt + NumCast,
{
    if !v.is_finite() {
        return Err(err);
    }

    let min: F = NumCast::from(I::min_value()).ok_or(err)?;
    let max: F = NumCast::from(I::max_value()).ok_or(err)?;

    if v < min || v > max {
        return Err(err);
    }

    NumCast::from(v).ok_or(err)
}

pub(crate) fn add_integer_checked<N, E>(a: N, b: N, err: E) -> Result<N, E>
where
    N: CheckedAdd<Output = N>,
{
    a.checked_add(&b).ok_or(err)
}

pub(crate) fn try_convert_checked<T, U, E>(v: T, err: E) -> Result<U, E>
where
    U: TryFrom<T>,
{
    U::try_from(v).map_err(|_| err)
}

pub(crate) fn try_convert_float<FB, FS, E>(v: FB, err: E) -> Result<FS, E> where FB: Float + NumCast, FS: Float + NumCast {
    if !v.is_finite() {
        return Err(err);
    }
    let out: FS = NumCast::from(v).ok_or(err)?;
    Ok(out)
}