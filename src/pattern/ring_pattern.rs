/// # Ring Patterns
/// `ring_pattern` is a module to represent ring patterns (bull's eye)

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;
use crate::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RingPattern {
    pub a: Color, // First color used in the pattern
    pub b: Color, // Second color used in the pattern
    pub transform: Matrix4,
}

impl RingPattern {
    pub fn new(color_a: Color, color_b: Color) -> RingPattern {
        RingPattern { a: color_a, b: color_b, transform: Matrix4::identity() }
    }
}

impl Pattern for RingPattern {
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
        // floor of the magnitude (x, z) mod 2 == 0
        if Float((point.x * point.x + point.z * point.z).value().sqrt().floor() % 2.0) == Float(0.0) {
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
    fn ring_pattern() {
        let pattern = RingPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.0)), Color::black());
        // 0.708 is slightly more than 2.0.sqrt()/2
        assert_eq!(pattern.pattern_at(&point(0.708, 0.0, 0.708)), Color::black());
    }
}
