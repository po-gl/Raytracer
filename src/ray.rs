/// # ray
/// `ray` is a module to represent a ray tracer's ray

use super::tuple;
use super::tuple::Tuple;

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn ray_sphere_intersection() {
        let r = Ray::new(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
    }
}
