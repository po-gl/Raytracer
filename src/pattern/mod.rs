/// # Patterns
/// `pattern` is a module to represent a color pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error, Debug};

pub mod stripe_pattern;


pub trait Pattern: Any {
    fn as_any(&self) -> &dyn Any;

    fn box_eq(&self, other: &dyn Any) -> bool;

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;

    fn pattern_clone(&self) -> Box<dyn Pattern>;

    fn transform(&self) -> Matrix4;

    fn set_transform(&mut self, transform: Matrix4);

    fn stripe_at(&self, point: &Tuple) -> Color;

    fn stripe_at_object(&self, object: Box<dyn Shape>, world_point: &Tuple) -> Color;
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

    #[test]
    fn pattern_creation() {

    }

    #[test]
    fn pattern_transformations() {

    }
}
