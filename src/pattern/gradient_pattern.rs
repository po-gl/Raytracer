/// # Gradient Patterns
/// `gradient_pattern` is a module to represent gradient patterns

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GradientPattern {
    pub a: Color, // First color used in the pattern
    pub b: Color, // Second color used in the pattern
    pub transform: Matrix4,
}

impl GradientPattern {
    pub fn new(color_a: Color, color_b: Color) -> GradientPattern {
        GradientPattern { a: color_a, b: color_b, transform: Matrix4::identity() }
    }
}

impl Pattern for GradientPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Box {:?}", self)
    }

    fn pattern_clone(&self) -> Box<dyn Pattern + Send> {
        Box::new(*self)
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: &Tuple) -> Color {
        // Interpolate color
        let distance = self.b - self.a;
        let fraction = point.x - point.x.value().floor();

        self.a + distance * fraction.value()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::point;

    #[test]
    fn gradient_pattern() {
        let pattern = GradientPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.pattern_at(&point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.pattern_at(&point(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
    }
}
