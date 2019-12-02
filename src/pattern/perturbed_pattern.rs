/// # Perturbed Patterns
/// `perturbed` is a module to represent a "perturbed" patterns
/// or a more organic looking pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;
use noise::{Perlin, NoiseFn};
use crate::tuple;

#[derive(Debug, Clone)]
pub struct PerturbedPattern {
    pub pattern: Option<Box<dyn Pattern>>, // Pattern to perturb
    pub transform: Matrix4,
    pub perlin: Perlin,
    /// Typically a positive number below 0.2
    pub perlin_factor: f64,
}

impl PerturbedPattern {
    pub fn new(pattern: Box<dyn Pattern>, perlin_factor: f64) -> PerturbedPattern {
        PerturbedPattern { pattern: Some(pattern), transform: Matrix4::identity(), perlin: Perlin::new(), perlin_factor }
    }
}

impl Pattern for PerturbedPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, _other: &dyn Any) -> bool {
//        other.downcast_ref::<Self>().map_or(false, |a| self == a)
        false
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
        let perlin_x = self.perlin.get([point.x.value(), point.y.value(), point.z.value()]) * self.perlin_factor;
        let perlin_y = self.perlin.get([point.x.value(), point.y.value(), point.z.value()]) * self.perlin_factor;
        let perlin_z = self.perlin.get([point.x.value(), point.y.value(), point.z.value()]) * self.perlin_factor;

        let perlin_point = point + tuple::point(perlin_x, perlin_y, perlin_z);

        self.pattern.clone().unwrap().pattern_at(&perlin_point)
    }
}


#[cfg(test)]
mod tests {
//    use super::*;
//    use crate::pattern::ring_pattern::RingPattern;
//    use crate::tuple::point;

    #[test]
    fn perturbed_patterns() {
//        // Rings that should add up to purple
//        let pattern_a = RingPattern::new(Color::from_hex("FF0000"), Color::black()); // Red
//        let pattern_b = RingPattern::new(Color::from_hex("0000FF"), Color::black()); // Blue
//        let pattern = PerturbedPattern::new(Box::new(pattern_a), Box::new(pattern_b));
//        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::new(0.5, 0.0, 0.5)); // Purple
//        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), Color::black());
    }
}

