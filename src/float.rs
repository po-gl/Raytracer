/// # float
/// `float` is a module to represent a modified f64

use std::ops;
use crate::FLOAT_THRESHOLD;
use std::cmp::Ordering;
use std::f64::MAX;

#[derive(Debug, Copy, Clone)]
pub struct Float(pub f64);

impl Float {
    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn sqrt(&self) -> f64 {
        self.0.sqrt()
    }

    pub fn clamp(&self, min: f64, max: f64) -> f64 {
        if self.value() < min {
            min
        } else if self.value() > max {
            max
        } else {
            self.value()
        }
    }

    pub fn max() -> Float {
        Float(MAX)
    }
}

// Addition
impl_op_ex!(+ |a: &Float, b: f64| -> Float { Float(a.0 + b) });
impl_op_ex!(+ |a: f64, b: &Float| -> Float { Float(a + b.0) });
impl_op_ex!(+ |a: &Float, b: &Float| -> Float { Float(a.0 + b.0) });

// Subtraction
impl_op_ex!(- |a: &Float, b: f64| -> Float { Float(a.0 - b) });
impl_op_ex!(- |a: f64, b: &Float| -> Float { Float(a - b.0) });
impl_op_ex!(- |a: &Float, b: &Float| -> Float { Float(a.0 - b.0) });

// Multiplication
impl_op_ex!(* |a: &Float, b: f64| -> Float { Float(a.0 * b) });
impl_op_ex!(* |a: f64, b: &Float| -> Float { Float(a * b.0) });
impl_op_ex!(* |a: &Float, b: &Float| -> Float { Float(a.0 * b.0) });

// Division
impl_op_ex!(/ |a: &Float, b: f64| -> Float { Float(a.0 / b) });
impl_op_ex!(/ |a: f64, b: &Float| -> Float { Float(a / b.0) });
impl_op_ex!(/ |a: &Float, b: &Float| -> Float { Float(a.0 / b.0) });

// '==' comparator
// Allow for non-precise floats to be equal within a threshold
impl PartialEq<Float> for Float {
    fn eq(&self, other: &Self) -> bool {
        // This shaves off a few ms compared to using .abs()
        ((self.0 - other.0) < FLOAT_THRESHOLD && (self.0 - other.0) >= 0.0) || ((other.0 - self.0) < FLOAT_THRESHOLD && (other.0 - self.0) >= 0.0)
    }
}
impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        // This shaves off a few ms compared to using .abs()
        ((self.0 - *other) < FLOAT_THRESHOLD && (self.0 - *other) >= 0.0) || ((*other - self.0) < FLOAT_THRESHOLD && (*other - self.0) >= 0.0)
    }
}
impl PartialEq<Float> for f64 {
    fn eq(&self, other: &Float) -> bool {
        // This shaves off a few ms compared to using .abs()
        ((self - other.0) < FLOAT_THRESHOLD && (self - other.0) >= 0.0) || ((other.0 - self) < FLOAT_THRESHOLD && (other.0 - self) >= 0.0)
    }
}

impl PartialOrd<Float> for Float {
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_eq() {
        assert_eq!(Float(3.0), Float(3.0000000000000001));
        assert_eq!(Float(3.0), 3.0000000000000001);
        assert_eq!(3.0, Float(3.0000000000000001));
    }

    #[test]
    fn float_operations() {
        let a = Float(-2.0);
        let b = Float(31.5);

        assert_eq!(&a + &b, Float(29.5));
        assert_eq!(&a - &b, Float(-33.5));
        assert_eq!(&a * &b, Float(-63.0));
        assert_eq!(&a / &b, Float(-0.06349206349206349));

        assert_eq!((&a + 6.0).value(), 4.0);
        assert_eq!((&a - 6.0).value(), -8.0);
        assert_eq!((&a * 6.0).value(), -12.0);
        assert_eq!((&a / 6.0).value(), -0.33333333333333333);

        assert_eq!((6.0 + &a).value(), 4.0);
        assert_eq!((6.0 - &a).value(), 8.0);
        assert_eq!((6.0 * &a).value(), -12.0);
        assert_eq!((6.0 / &a).value(), -3.0);
    }

    #[test]
    fn float_performance() {

    }
}
