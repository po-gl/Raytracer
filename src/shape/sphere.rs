/// # Sphere
/// `sphere` is a module to represent a sphere shape

use crate::shape::Shape;
use crate::ray::Ray;
use crate::shape;
use crate::tuple;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::{Tuple, point};
use crate::float::Float;
use crate::material::Material;
use std::any::Any;
use std::fmt::{Formatter, Error};


#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub id: i32,
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        let id = shape::get_shape_id();
        Sphere {id, transform: Matrix4::identity(), material: Material::new()}
    }

    pub fn new_with_material(material: Material) -> Sphere {
        let id = shape::get_shape_id();
        Sphere{id, transform: Matrix4::identity(), material}
    }
}

impl Shape for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Box {:?}", self)
    }

    fn shape_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn intersects(&self, ray: &Ray) -> Vec<Intersection<Box<dyn Shape>>> {
        // Transform the ray
        let t_ray = ray.transform(&self.transform.inverse());
        // vector from the sphere's center to the ray origin
        let sphere_to_ray =t_ray.origin - point(0.0, 0.0, 0.0);

        let a = tuple::dot(&t_ray.direction, &t_ray.direction);
        let b = 2.0 * tuple::dot(&t_ray.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if Float(discriminant) < Float(0.0) {
            return vec![Intersection::new(0.0, Box::new(self.clone())); 0]
        } else {
            let disc_sqrt = discriminant.sqrt();
            let t1 = (-b - disc_sqrt) / (2.0 * a);
            let t2 = (-b + disc_sqrt) / (2.0 * a);
            return vec![Intersection::new(t1, Box::new(self.clone())),
                        Intersection::new(t2, Box::new(self.clone()))];
        }
    }

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = Float(0.0);
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformation;
    use crate::tuple::vector;
    use std::f64::consts::PI;

    #[test]
    fn sphere_intersection() {
        // Straight through
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        // Just the top (tangent)
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);

        // Missing the sphere
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 0);

        // Starting inside the sphere
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);

        // Starting after the sphere (should have negative t value)
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);


        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert!(s.box_eq(xs[0].object.as_any()));
        assert!(s.box_eq(xs[1].object.as_any()));
//        assert_eq!(&xs[0].object, &s);
//        assert_eq!(&xs[1].object, &s);
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
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(transformation::scaling(2.0, 2.0, 2.0));
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);

        // Intersecting a translated sphere with a ray
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.set_transform(transformation::translation(5.0, 0.0, 0.0));
        let xs = s.intersects(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_normals() {
        let s = Sphere::new();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));

        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));

        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));

        let s = Sphere::new();
        let n = s.normal_at(&point(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));
        assert_eq!(n, vector(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));

        // Verify normals are normalized
        let s = Sphere::new();
        let n = s.normal_at(&point(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));
        assert_eq!(n, n.normalize());

        // Transformed normals
        let mut s = Sphere::new();
        s.set_transform(transformation::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
        assert_eq!(n, vector(0.0, 0.70711, -0.70711));

        let mut s = Sphere::new();
        let m = transformation::scaling(1.0, 0.5, 1.0) * transformation::rotation_z(PI/5.0);
        s.set_transform(m);
        let n = s.normal_at(&point(0.0, 2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0));
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_material() {
        let s = Sphere::new();
        let m = s.material;
        assert_eq!(m, Material::new());

        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = Float(1.0);
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn sphere_glassy_material() {
        let s = Sphere::new_with_material(Material::glass());
        assert_eq!(s.transform, Matrix4::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}