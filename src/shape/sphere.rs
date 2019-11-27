/// # Sphere
/// `sphere` is a module to represent a sphere shape

use crate::shape::Shape;
use crate::ray::Ray;
use crate::shape;
use crate::tuple;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::Tuple;


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub id: i32,
    pub transform: Matrix4,
}

impl Sphere {
    pub fn new() -> Sphere {
        let id = shape::get_shape_id();
        Sphere {id, transform: Matrix4::identity()}
    }
}

impl Shape<Sphere> for Sphere {
    fn intersects(&self, ray: Ray) -> Vec<Intersection<Sphere>> {
        // Transform the ray
        let t_ray = ray.transform(&self.transform.inverse());
        // vector from the sphere's center to the ray origin
        let sphere_to_ray =t_ray.origin - tuple::point(0.0, 0.0, 0.0);

        let a = tuple::dot(&t_ray.direction, &t_ray.direction);
        let b = 2.0 * tuple::dot(&t_ray.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![Intersection::new(0.0, *self); 0]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return vec![Intersection::new(t1, *self),
                        Intersection::new(t2, *self)];
        }
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        (point - tuple::point(0.0, 0.0, 0.0)).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformation;

    #[test]
    fn sphere_intersection() {
        // Straight through
        let r = Ray::new(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        // Just the top (tangent)
        let r = Ray::new(tuple::point(0.0, 1.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);

        // Missing the sphere
        let r = Ray::new(tuple::point(0.0, 2.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 0);

        // Starting inside the sphere
        let r = Ray::new(tuple::point(0.0, 0.0, 0.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);

        // Starting after the sphere (should have negative t value)
        let r = Ray::new(tuple::point(0.0, 0.0, 5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);


        let r = Ray::new(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(&xs[0].object, &s);
        assert_eq!(&xs[1].object, &s);
    }

    #[test]
    fn sphere_transforms() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix4::identity());

        let mut s = Sphere::new();
        let t = transformation::translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transform, t);

        // Intersecting a scaled sphere with a ray
        let r = Ray::new(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(transformation::scaling(2.0, 2.0, 2.0));
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);

        // Intersecting a translated sphere with a ray
        let r = Ray::new(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(transformation::translation(5.0, 0.0, 0.0));
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_normals() {
        let s = Sphere::new();
        let n = s.normal_at(tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, tuple::vector(1.0, 0.0, 0.0));

        let s = Sphere::new();
        let n = s.normal_at(tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, tuple::vector(0.0, 1.0, 0.0));

        let s = Sphere::new();
        let n = s.normal_at(tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::new();
        let n = s.normal_at(tuple::point(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));
        assert_eq!(n, tuple::vector(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));

        // Verify normals are normalized
        let s = Sphere::new();
        let n = s.normal_at(tuple::point(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));
        assert_eq!(n, n.normalize());
    }
}