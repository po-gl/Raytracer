/// # Patterns
/// `pattern` is a module to represent a color pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error, Debug};

pub mod test_pattern;
pub mod stripe_pattern;
pub mod gradient_pattern;
pub mod ring_pattern;


pub trait Pattern: Any {
    fn as_any(&self) -> &dyn Any;

    fn box_eq(&self, other: &dyn Any) -> bool;

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;

    fn pattern_clone(&self) -> Box<dyn Pattern>;

    fn transform(&self) -> Matrix4;

    fn set_transform(&mut self, transform: Matrix4);

    fn pattern_at(&self, point: &Tuple) -> Color;

    fn pattern_at_object(&self, object: Box<dyn Shape>, world_point: &Tuple) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * object_point;
        self.pattern_at(&pattern_point)
    }
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Box<dyn Pattern>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Debug for Box<dyn Pattern> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.debug_fmt(f)
    }
}

impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Self {
        self.pattern_clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::pattern::test_pattern::TestPattern;
    use crate::matrix::Matrix4;
    use crate::pattern::Pattern;
    use crate::transformation::{translation, scaling};
    use crate::shape::sphere::Sphere;
    use crate::color::Color;
    use crate::tuple::point;
    use crate::shape::Shape;

    #[test]
    fn pattern_creation() {
        let pattern = TestPattern::new();
        assert_eq!(pattern.transform, Matrix4::identity());
    }

    #[test]
    fn pattern_transformations() {
        let mut pattern = TestPattern::new();
        pattern.set_transform(translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.transform, translation(1.0, 2.0, 3.0));
    }

    #[test]
    fn pattern_at_object() {
        // Pattern with an object transformation
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::new();
        let c = pattern.pattern_at_object(Box::new(object), &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));

        // Pattern with a transformation
        let object = Sphere::new();
        let mut pattern = TestPattern::new();
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_object(Box::new(object), &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));

        // Pattern and object with a transformation
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = TestPattern::new();
        pattern.set_transform(translation(0.5, 1.0, 1.5));
        let c = pattern.pattern_at_object(Box::new(object), &point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
