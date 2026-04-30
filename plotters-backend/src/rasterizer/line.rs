use crate::{
    math_guard::{
        checked_add, checked_mul, checked_sub, float_to_integer_checked, non_zero_checked,
    },
    BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind, MathError,
};
use std::convert::TryFrom;

pub fn draw_line<DB: DrawingBackend, S: BackendStyle>(
    back: &mut DB,
    mut from: BackendCoord,
    mut to: BackendCoord,
    style: &S,
) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
    if style.color().alpha == 0.0 || style.stroke_width() == 0 {
        return Ok(());
    }

    if style.stroke_width() != 1 {
        // If the line is wider than 1px, then we need to make it a polygon
        let dx = i64::from(checked_sub(to.0, from.0, MathError::ValueOverflow)?);
        let dy = i64::from(checked_sub(to.1, from.1, MathError::ValueOverflow)?);

        let x2 = checked_mul(dx, dx, MathError::ValueOverflow)?;
        let y2 = checked_mul(dy, dy, MathError::ValueOverflow)?;
        let sum = checked_add(x2, y2, MathError::ValueOverflow)? as f64;

        let len = sum.sqrt();

        if len < 1e-5 {
            return Ok(());
        }

        let len = non_zero_checked(len, MathError::ZeroDivision)?;
        let v = (dx as f64 / len, dy as f64 / len);

        let r = f64::from(style.stroke_width()) / 2.0;
        let mut trans = [(v.1 * r, -v.0 * r), (-v.1 * r, v.0 * r)];
        let mut vertices = vec![];

        for point in [from, to].iter() {
            for t in trans.iter() {
                vertices.push((
                    (f64::from(point.0) + t.0) as i32,
                    (f64::from(point.1) + t.1) as i32,
                ))
            }

            trans.swap(0, 1);
        }

        return back.fill_polygon(vertices, style);
    }

    if from.0 == to.0 {
        if from.1 > to.1 {
            std::mem::swap(&mut from, &mut to);
        }
        for y in from.1..=to.1 {
            check_result!(back.draw_pixel((from.0, y), style.color()));
        }
        return Ok(());
    }

    if from.1 == to.1 {
        if from.0 > to.0 {
            std::mem::swap(&mut from, &mut to);
        }
        for x in from.0..=to.0 {
            check_result!(back.draw_pixel((x, from.1), style.color()));
        }
        return Ok(());
    }
    let dx = checked_sub::<i64, MathError>(
        i64::from(to.0),
        i64::from(from.0),
        MathError::ValueUnderflow,
    )?;
    let dy = checked_sub::<i64, MathError>(
        i64::from(to.1),
        i64::from(from.1),
        MathError::ValueUnderflow,
    )?;
    let steep = dx.abs() < dy.abs();

    if steep {
        from = (from.1, from.0);
        to = (to.1, to.0);
    }

    let (from, to) = if from.0 > to.0 {
        (to, from)
    } else {
        (from, to)
    };

    let mut size_limit = back.get_size();

    if steep {
        size_limit = (size_limit.1, size_limit.0);
    }
    let grad = f64::from(checked_sub(to.1, from.1, MathError::ValueOverflow)?)
        / f64::from(non_zero_checked(
            checked_sub(to.0, from.0, MathError::ValueOverflow)?,
            MathError::ZeroDivision,
        )?);

    let mut put_pixel = |(x, y): BackendCoord, b: f64| {
        if steep {
            back.draw_pixel((y, x), style.color().mix(b))
        } else {
            back.draw_pixel((x, y), style.color().mix(b))
        }
    };
    let y_max = checked_sub(
        i32::try_from(size_limit.1).map_err(|_| MathError::ValueOutOfRange)?,
        1,
        MathError::ValueUnderflow,
    )?;

    let y_clamped = to.1.min(y_max).max(0);

    let y_delta = checked_sub(y_clamped, from.1, MathError::ValueOverflow)?;

    let y_step_limit = (f64::from(y_delta) / grad).floor() as i32;

    let y_max = checked_sub(
        i32::try_from(size_limit.1).map_err(|_| MathError::ValueOutOfRange)?,
        2,
        MathError::ValueUnderflow,
    )?;

    let y_clamped = from.1.min(y_max).max(0);

    let y_delta = checked_sub(y_clamped, from.1, MathError::ValueOverflow)?;

    let x_offset = (f64::from(y_delta) / grad).abs().ceil() as i32;

    let batch_start = checked_add(x_offset, from.0, MathError::ValueOverflow)?;

    let x_max = checked_sub(
        i32::try_from(size_limit.0).map_err(|_| MathError::ValueOutOfRange)?,
        2,
        MathError::ValueUnderflow,
    )?;

    let stepped_x = checked_sub(
        checked_add(from.0, y_step_limit, MathError::ValueOverflow)?,
        1,
        MathError::ValueUnderflow,
    )?;

    let batch_limit = to.0.min(x_max).min(stepped_x);

    let batch_delta = checked_sub(batch_start, from.0, MathError::ValueOverflow)?;

    let mut y = f64::from(from.1) + f64::from(batch_delta) * grad;

    for x in batch_start..=batch_limit {
        let y_i = float_to_integer_checked::<f64, i32, MathError>(y, MathError::ValueOutOfRange)?;

        let y_next = checked_add(y_i, 1, MathError::ValueOverflow)?;

        let y_floor = y.floor();

        check_result!(put_pixel((x, y_i), 1.0 + y_floor - y));
        check_result!(put_pixel((x, y_next), y - y_floor));

        y += grad;
    }

    if to.0 > batch_limit && y < f64::from(to.1) {
        let x = checked_add(batch_limit, 1, MathError::ValueOverflow)?;
        let y_floor = y.floor();

        let y_i = float_to_integer_checked::<f64, i32, MathError>(y, MathError::ValueOutOfRange)?;

        let lower_alpha = 1.0 + y_floor - y;
        if lower_alpha > 1e-5 {
            check_result!(put_pixel((x, y_i), lower_alpha));
        }

        let upper_alpha = y - y_floor;
        let y_next = checked_add(y_i, 1, MathError::ValueOverflow)?;

        if upper_alpha > 1e-5 && y + 1.0 < f64::from(to.1) {
            check_result!(put_pixel((x, y_next), upper_alpha));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // tried keep this unit test inside this file as much as possible. 
    use super::*;
    use crate::{BackendColor, BackendStyle};

    // a simple backend error for testing in this module
    #[derive(Debug)]
    struct TestBackendError;

    impl std::fmt::Display for TestBackendError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "test backend error")
        }
    }

    impl std::error::Error for TestBackendError {}

    #[derive(Clone, Copy)]
    struct TestStyle {
        color: BackendColor,
        stroke_width: u32,
    }

    impl BackendStyle for TestStyle {
        fn color(&self) -> BackendColor {
            self.color
        }

        fn stroke_width(&self) -> u32 {
            self.stroke_width
        }
    }

    #[derive(Default)]
    struct TestBackend {
        size: (u32, u32),
        pixels: Vec<(BackendCoord, BackendColor)>,
        polygons: Vec<Vec<BackendCoord>>,
    }

    impl DrawingBackend for TestBackend {
        type ErrorType = TestBackendError;

        fn get_size(&self) -> (u32, u32) {
            self.size
        }

        fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
            Ok(())
        }

        fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
            Ok(())
        }

        fn draw_pixel(
            &mut self,
            point: BackendCoord,
            color: BackendColor,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
            self.pixels.push((point, color));
            Ok(())
        }

        fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
            &mut self,
            vertices: I,
            _style: &S,
        ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
            self.polygons.push(vertices.into_iter().collect());
            Ok(())
        }
    }

    fn style(stroke_width: u32, alpha: f64) -> TestStyle {
        TestStyle {
            color: BackendColor {
                rgb: (0, 0, 0),
                alpha,
            },
            stroke_width,
        }
    }

    fn backend() -> TestBackend {
        TestBackend {
            size: (100, 100),
            ..Default::default()
        }
    }

    #[test]
    fn transparent_line_draws_nothing() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (0, 0), (10, 10), &style(1, 0.0)).is_ok());

        assert!(backend.pixels.is_empty());
        assert!(backend.polygons.is_empty());
    }

    #[test]
    fn zero_width_line_draws_nothing() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (0, 0), (10, 10), &style(0, 1.0)).is_ok());

        assert!(backend.pixels.is_empty());
        assert!(backend.polygons.is_empty());
    }

    #[test]
    fn vertical_line_draws_pixels_in_order() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (2, 1), (2, 3), &style(1, 1.0)).is_ok());

        let points: Vec<_> = backend.pixels.iter().map(|(point, _)| *point).collect();

        assert_eq!(points, vec![(2, 1), (2, 2), (2, 3)]);
    }

    #[test]
    fn reversed_vertical_line_draws_pixels_in_order() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (2, 3), (2, 1), &style(1, 1.0)).is_ok());

        let points: Vec<_> = backend.pixels.iter().map(|(point, _)| *point).collect();

        assert_eq!(points, vec![(2, 1), (2, 2), (2, 3)]);
    }

    #[test]
    fn horizontal_line_draws_pixels_in_order() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (1, 2), (3, 2), &style(1, 1.0)).is_ok());

        let points: Vec<_> = backend.pixels.iter().map(|(point, _)| *point).collect();

        assert_eq!(points, vec![(1, 2), (2, 2), (3, 2)]);
    }

    #[test]
    fn reversed_horizontal_line_draws_pixels_in_order() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (3, 2), (1, 2), &style(1, 1.0)).is_ok());

        let points: Vec<_> = backend.pixels.iter().map(|(point, _)| *point).collect();

        assert_eq!(points, vec![(1, 2), (2, 2), (3, 2)]);
    }

    #[test]
    fn diagonal_line_draws_some_pixels() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (1, 1), (5, 3), &style(1, 1.0)).is_ok());

        assert!(!backend.pixels.is_empty());
    }

    #[test]
    fn wide_zero_length_line_is_noop() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (5, 5), (5, 5), &style(3, 1.0)).is_ok());

        assert!(backend.pixels.is_empty());
        assert!(backend.polygons.is_empty());
    }

    #[test]
    fn wide_line_is_converted_to_polygon() {
        let mut backend = backend();

        assert!(draw_line(&mut backend, (10, 10), (20, 10), &style(4, 1.0)).is_ok());

        assert_eq!(backend.polygons.len(), 1);
        assert_eq!(backend.polygons[0].len(), 4);
    }

    #[test]
    fn wide_line_reports_math_error_for_extreme_delta() {
        let mut backend = backend();

        let err =
            draw_line(&mut backend, (i32::MIN, 0), (i32::MAX, 0), &style(4, 1.0)).unwrap_err();

        assert!(matches!(
            err,
            DrawingErrorKind::MathError(MathError::ValueOverflow)
        ));
    }

    #[test]
    fn diagonal_line_reports_math_error_for_extreme_x_span() {
        let mut backend = backend();

        let err =
            draw_line(&mut backend, (i32::MIN, 0), (i32::MAX, 1), &style(1, 1.0)).unwrap_err();

        assert!(matches!(
            err,
            DrawingErrorKind::MathError(MathError::ValueOverflow)
        ));
    }

    #[test]
    fn diagonal_line_with_tiny_backend_does_not_panic() {
        let mut backend = TestBackend {
            size: (1, 1),
            ..Default::default()
        };

        assert!(draw_line(&mut backend, (0, 0), (2, 1), &style(1, 1.0)).is_ok());
    }
}
