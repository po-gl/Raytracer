/// # Blended Patterns
/// `blended_patterns` is a module to represent a blending 2 patterns

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct BlendedPattern {
    pub a: Option<Box<dyn Pattern>>, // First pattern to blend
    pub b: Option<Box<dyn Pattern>>, // Second pattern to blend
    pub transform: Matrix4,
}

impl BlendedPattern {
    pub fn new(pattern_a: Box<dyn Pattern>, pattern_b: Box<dyn Pattern>) -> BlendedPattern {
        BlendedPattern { a: Some(pattern_a), b: Some(pattern_b), transform: Matrix4::identity() }
    }
}

impl Pattern for BlendedPattern {
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
        Box::new(self.clone())
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        let color_a = self.a.clone().unwrap().pattern_at(point);
        let color_b = self.b.clone().unwrap().pattern_at(point);

        (color_a + color_b) * 0.5 // average the 2 colors
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::ring_pattern::RingPattern;
    use crate::tuple::point;

    #[test]
    fn blended_patterns() {
        // Rings that should add up to purple
        let pattern_a = RingPattern::new(Color::from_hex("FF0000"), Color::black()); // Red
        let pattern_b = RingPattern::new(Color::from_hex("0000FF"), Color::black()); // Blue
        let pattern = BlendedPattern::new(Box::new(pattern_a), Box::new(pattern_b));
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::new(0.5, 0.0, 0.5)); // Purple
        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), Color::black());
    }
}

