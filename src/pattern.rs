/// # Patterns
/// `pattern` is a module to represent a color pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pattern {
    pub a: Color, // First color used in the pattern
    pub b: Color, // Second color used in the pattern
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {a: Color::white(), b: Color::white()}
    }

    pub fn stripe_pattern(color_a: Color, color_b: Color) -> Pattern {
        Pattern {a: color_a, b: color_b}
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        // Only x effects the stripe pattern
        if Float(point.x.value().floor() % 2.0) == Float(0.0) {
            Color::white()
        } else {
            Color::black()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::point;

    #[test]
    fn pattern_creation() {
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn pattern_stripe_at() {
        // A stripe pattern is constant in y
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 2.0, 0.0)), Color::white());

        // A stripe pattern is constant in z
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 2.0)), Color::white());

        // A stripe pattern alternates in x
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(point(-1.1, 0.0, 0.0)), Color::white());
    }
}
