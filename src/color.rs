/// # color
/// `color` is a module to represent color tuples

use std::ops;
use super::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub red: Float,
    pub green: Float,
    pub blue: Float,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {red: Float(r), green: Float(g), blue: Float(b)}
    }
}


// Addition
impl_op_ex!(+ |a: &Color, b: &Color| -> Color { Color {red: &a.red + &b.red, green: &a.green + &b.green, blue: &a.blue + &b.blue} });

// Subtraction
impl_op_ex!(- |a: &Color, b: &Color| -> Color { Color {red: &a.red - &b.red, green: &a.green - &b.green, blue: &a.blue - &b.blue} });

// Multiplication
impl_op_ex!(* |a: &Color, s: f64| -> Color { Color {red: &a.red * s, green: &a.green * s, blue: &a.blue * s} });
impl_op_ex!(* |a: &Color, b: &Color| -> Color { Color {red: &a.red * &b.red, green: &a.green * &b.green, blue: &a.blue * &b.blue} }); // Hadarmard product


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_creation() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn color_operations() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);

        // Addition
        assert_eq!(&a + &b, Color::new(1.6, 0.7, 1.0));

        // Subtraction
        assert_eq!(&a - &b, Color::new(0.2, 0.5, 0.5));

        // Scalar Multiplication
        let a = Color::new(0.2, 0.3, 0.4);
        assert_eq!(&a * 2.0, Color::new(0.4, 0.6, 0.8));

        // Multiplication
        let a  = Color::new(1.0, 0.2, 0.4);
        let b  = Color::new(0.9, 1.0, 0.1);
        assert_eq!(&a * &b, Color::new(0.9, 0.2, 0.04));
    }
}
