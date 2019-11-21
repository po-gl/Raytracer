/// # Sphere
/// `sphere` is a module to represent a sphere shape

use crate::shape::Shape;
use crate::float::Float;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::shape;
use crate::tuple;


#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub id: i32,
    pub center: Tuple,
    pub radius: Float,
}

impl Sphere {
    pub fn new(center: Tuple, radius: f64) -> Sphere {
        let id = shape::get_shape_id();
        Sphere {id, center, radius: Float(radius)}
    }
}

impl Shape for Sphere {
    fn intersects(&self, ray: Ray) -> Vec<Float> {
        // vector from the sphere's center to the ray origin
        let sphere_to_ray = ray.origin - tuple::point(0.0, 0.0, 0.0);

        let a = tuple::dot(&ray.direction, &ray.direction);
        let b = 2.0 * tuple::dot(&ray.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![Float(0.0); 0]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return vec![Float(t1), Float(t2)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_intersection() {
        // Straight through
        let r = Ray::new(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);

        // Just the top (tangent)
        let r = Ray::new(tuple::point(0.0, 1.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);

        // Missing the sphere
        let r = Ray::new(tuple::point(0.0, 2.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 0);

        // Starting inside the sphere
        let r = Ray::new(tuple::point(0.0, 0.0, 0.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);

        // Starting after the sphere (should have negative t value)
        let r = Ray::new(tuple::point(0.0, 0.0, 5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new(tuple::point(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}