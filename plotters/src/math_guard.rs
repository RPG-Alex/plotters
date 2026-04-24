use num_traits::{Float, NumCast, PrimInt};

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


pub(crate) fn try_convert_float<FB, FS, E>(v: FB, err: E) -> Result<FS, E> where FB: Float + NumCast, FS: Float + NumCast {
    if !v.is_finite() {
        return Err(err);
    }
    let out: FS = NumCast::from(v).ok_or(err)?;
    Ok(out)
}