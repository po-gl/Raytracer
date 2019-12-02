/// # Patterns
/// `pattern` is a module to represent a color pattern

use crate::color::Color;
use crate::tuple::Tuple;
use crate::float::Float;
use crate::matrix::Matrix4;
use crate::shape::Shape;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pattern {
    pub a: Color, // First color used in the pattern
    pub b: Color, // Second color used in the pattern
    pub transform: Matrix4,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern {a: Color::white(), b: Color::white(), transform: Matrix4::identity()}
    }

    pub fn stripe_pattern(color_a: Color, color_b: Color) -> Pattern {
        Pattern {a: color_a, b: color_b, transform: Matrix4::identity()}
    }

    pub fn stripe_at(&self, point: &Tuple) -> Color {
        // Only x effects the stripe pattern
        if Float(point.x.value().floor() % 2.0) == Float(0.0) {
            Color::white()
        } else {
            Color::black()
        }
    }

    pub fn stripe_at_object(&self, object: Box<dyn Shape>, world_point: &Tuple) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.stripe_at(&pattern_point)
    }

    pub fn set_transform(&mut self, matrix: Matrix4) {
        self.transform = matrix;
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
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn pattern_stripe_at() {
        // A stripe pattern is constant in y
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(&point(0.0, 2.0, 0.0)), Color::white());

        // A stripe pattern is constant in z
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 2.0)), Color::white());

        // A stripe pattern alternates in x
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.stripe_at(&point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn pattern_transformations() {
        // Transform object
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        let c = pattern.stripe_at_object(Box::new(object), &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());

        // Transform pattern
        let object = Sphere::new();
        let mut pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern.stripe_at_object(Box::new(object), &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());

        // Both object and pattern transforms
        let mut object = Sphere::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::stripe_pattern(Color::white(), Color::black());
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = pattern.stripe_at_object(Box::new(object), &point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }
}
