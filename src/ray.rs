/// # ray
/// `ray` is a module to represent a ray tracer's ray

use super::tuple::Tuple;
use super::matrix::Matrix4;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    /// Constructor for Ray
    ///
    /// origin is a point and direction is a vector
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        assert!(origin.is_point() && direction.is_vector());
        Ray {origin, direction}
    }

    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &self.direction * t
    }

    pub fn transform(&self, matrix: &Matrix4) -> Ray{
        Ray::new(matrix * self.origin, matrix * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple;
    use crate::transformation;

    #[test]
    fn ray_creation() {
        let origin = tuple::point(1.0, 2.0, 3.0);
        let direction = tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn ray_position() {
        let r = Ray::new(tuple::point(2.0, 3.0, 4.0), tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_transformations() {
        // Translating
        let r = Ray::new(tuple::point(1.0, 2.0, 3.0), tuple::vector(0.0, 1.0, 0.0));
        let m = transformation::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, tuple::vector(0.0, 1.0, 0.0));

        // Scaling
        let r = Ray::new(tuple::point(1.0, 2.0, 3.0), tuple::vector(0.0, 1.0, 0.0));
        let m = transformation::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, tuple::vector(0.0, 3.0, 0.0));
    }
}
