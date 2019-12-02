/// # Test Patterns
/// `test_pattern` is a module to test patterns

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TestPattern {
    pub transform: Matrix4,
}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern { transform: Matrix4::identity() }
    }
}

impl Pattern for TestPattern {
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
        // Simply returns a color of the point
        Color::new(point.x.value(), point.y.value(), point.z.value())
    }
}
