/// # Checker Patterns
/// `checker_pattern` is a module to represent a checker board pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::float::Float;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CheckerPattern {
    pub a: Color, // First color used in the pattern
    pub b: Color, // Second color used in the pattern
    pub transform: Matrix4,
}

impl CheckerPattern {
    pub fn new(color_a: Color, color_b: Color) -> CheckerPattern {
        CheckerPattern { a: color_a, b: color_b, transform: Matrix4::identity() }
    }
}

impl Pattern for CheckerPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Box {:?}", self)
    }

    fn pattern_clone(&self) -> Box<dyn Pattern> {
        Box::new(*self)
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        // Only x effects the stripe pattern
        if Float((point.x.value().floor() + point.y.value().floor() + point.z.value().floor()) % 2.0) == Float(0.0) {
            self.a
        } else {
            self.b
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::point;

    #[test]
    fn checker_pattern() {
        // Checkers should repeat in x
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(1.01, 0.0, 0.0)), Color::black());

        // Checkers should repeat in y
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 1.01, 0.0)), Color::black());

        // Checkers should repeat in z

        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.01)), Color::black());
    }
}

