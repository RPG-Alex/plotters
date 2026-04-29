use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedSub, Float, NumCast, PrimInt, Zero,
};

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

pub(crate) fn try_convert_float<FB, FS, E>(v: FB, err: E) -> Result<FS, E>
where
    FB: Float + NumCast,
    FS: Float + NumCast,
{
    if !v.is_finite() {
        return Err(err);
    }
    let out: FS = NumCast::from(v).ok_or(err)?;
    Ok(out)
}

pub(crate) fn non_zero_checked<T, E>(v: T, err: E) -> Result<T, E>
where
    T: Zero,
{
    if v.is_zero() {
        Err(err)
    } else {
        Ok(v)
    }
}

pub(crate) fn checked_add<T, E>(lhs: T, rhs: T, err: E) -> Result<T, E>
where
    T: CheckedAdd,
{
    lhs.checked_add(&rhs).ok_or(err)
}

pub(crate) fn checked_mul<T, E>(lhs: T, rhs: T, err: E) -> Result<T, E>
where
    T: CheckedMul,
{
    lhs.checked_mul(&rhs).ok_or(err)
}

pub(crate) fn checked_div<T, E>(lhs: T, rhs: T, err: E) -> Result<T, E>
where
    T: CheckedDiv,
{
    lhs.checked_div(&rhs).ok_or(err)
}

pub(crate) fn checked_sub<T, E>(lhs: T, rhs: T, err: E) -> Result<T, E>
where
    T: CheckedSub,
{
    lhs.checked_sub(&rhs).ok_or(err)
}

pub(crate) fn checked_neg<T, E>(v: T, err: E) -> Result<T, E>
where
    T: CheckedNeg,
{
    v.checked_neg().ok_or(err)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR: &str = "math error";

    #[test]
    fn float_to_integer_checked_accepts_valid_value() {
        assert_eq!(float_to_integer_checked::<f64, i32, _>(42.0, ERR), Ok(42));
    }

    #[test]
    fn float_to_integer_checked_rejects_non_finite_values() {
        assert_eq!(
            float_to_integer_checked::<f64, i32, _>(f64::NAN, ERR),
            Err(ERR)
        );
        assert_eq!(
            float_to_integer_checked::<f64, i32, _>(f64::INFINITY, ERR),
            Err(ERR)
        );
        assert_eq!(
            float_to_integer_checked::<f64, i32, _>(f64::NEG_INFINITY, ERR),
            Err(ERR)
        );
    }

    #[test]
    fn float_to_integer_checked_rejects_out_of_range_values() {
        assert_eq!(float_to_integer_checked::<f64, i8, _>(128.0, ERR), Err(ERR));

        assert_eq!(
            float_to_integer_checked::<f64, i8, _>(-129.0, ERR),
            Err(ERR)
        );
    }

    #[test]
    fn try_convert_float_accepts_finite_value() {
        assert_eq!(try_convert_float::<f64, f32, _>(1.5, ERR), Ok(1.5_f32));
    }

    #[test]
    fn try_convert_float_rejects_non_finite_values() {
        assert_eq!(try_convert_float::<f64, f32, _>(f64::NAN, ERR), Err(ERR));
        assert_eq!(
            try_convert_float::<f64, f32, _>(f64::INFINITY, ERR),
            Err(ERR)
        );
        assert_eq!(
            try_convert_float::<f64, f32, _>(f64::NEG_INFINITY, ERR),
            Err(ERR)
        );
    }

    #[test]
    fn non_zero_checked_accepts_non_zero_value() {
        assert_eq!(non_zero_checked(7_i32, ERR), Ok(7));
    }

    #[test]
    fn non_zero_checked_rejects_zero() {
        assert_eq!(non_zero_checked(0_i32, ERR), Err(ERR));
    }

    #[test]
    fn checked_add_accepts_valid_sum() {
        assert_eq!(checked_add(2_i32, 3_i32, ERR), Ok(5));
    }

    #[test]
    fn checked_add_rejects_overflow() {
        assert_eq!(checked_add(i32::MAX, 1, ERR), Err(ERR));
    }

    #[test]
    fn checked_sub_accepts_valid_difference() {
        assert_eq!(checked_sub(5_i32, 3_i32, ERR), Ok(2));
    }

    #[test]
    fn checked_sub_rejects_overflow() {
        assert_eq!(checked_sub(i32::MIN, 1, ERR), Err(ERR));
    }

    #[test]
    fn checked_mul_accepts_valid_product() {
        assert_eq!(checked_mul(6_i32, 7_i32, ERR), Ok(42));
    }

    #[test]
    fn checked_mul_rejects_overflow() {
        assert_eq!(checked_mul(i32::MAX, 2, ERR), Err(ERR));
    }

    #[test]
    fn checked_div_accepts_valid_quotient() {
        assert_eq!(checked_div(8_i32, 2_i32, ERR), Ok(4));
    }

    #[test]
    fn checked_div_rejects_division_by_zero() {
        assert_eq!(checked_div(8_i32, 0_i32, ERR), Err(ERR));
    }

    #[test]
    fn checked_div_rejects_min_divided_by_negative_one() {
        assert_eq!(checked_div(i32::MIN, -1, ERR), Err(ERR));
    }

    #[test]
    fn checked_neg_accepts_valid_negation() {
        assert_eq!(checked_neg(7_i32, ERR), Ok(-7));
    }

    #[test]
    fn checked_neg_rejects_min_value() {
        assert_eq!(checked_neg(i32::MIN, ERR), Err(ERR));
    }
}
