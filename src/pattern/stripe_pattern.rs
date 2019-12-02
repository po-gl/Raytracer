/// # Stripe Patterns
/// `stripe_pattern` is a module to represent a stripe pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::float::Float;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use std::fmt::{Formatter, Error};
use std::any::Any;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripePattern {
    pub a: Color, // First color used in the pattern
    pub b: Color, // Second color used in the pattern
    pub transform: Matrix4,
}

impl StripePattern {
    pub fn new(color_a: Color, color_b: Color) -> StripePattern {
        StripePattern { a: color_a, b: color_b, transform: Matrix4::identity() }
    }
}

impl Pattern for StripePattern {
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
        if Float(point.x.value().floor() % 2.0) == Float(0.0) {
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
    use crate::shape::sphere::Sphere;
    use crate::shape::Shape;
    use crate::transformation::{scaling, translation};

    #[test]
    fn pattern_creation() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn pattern_stripe_at() {
        // A stripe pattern is constant in y
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 2.0, 0.0)), Color::white());

        // A stripe pattern is constant in z
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 2.0)), Color::white());

        // A stripe pattern alternates in x
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(&point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn pattern_transformations() {
        // Transform object
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = StripePattern::new(Color::white(), Color::black());
        let c = pattern.pattern_at_object(Box::new(object), &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());

        // Transform pattern
        let object = Sphere::new();
        let mut pattern = StripePattern::new(Color::white(), Color::black());
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_object(Box::new(object), &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());

        // Both object and pattern transforms
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = StripePattern::new(Color::white(), Color::black());
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = pattern.pattern_at_object(Box::new(object), &point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }
}

