/// # Plane
/// `plane` is a module to represent an xy plane

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::{shape};
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::tuple::{Tuple, vector};
use crate::float::Float;

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub id: i32,
    pub transform: Matrix4,
    pub material: Material,
}

impl Plane {
    pub fn new() -> Plane {
        let id = shape::get_shape_id();
        Plane {id, transform: Matrix4::identity(), material: Material::new()}
    }

    pub fn new_with_material(material: Material) -> Plane {
        let id = shape::get_shape_id();
        Plane {id, transform: Matrix4::identity(), material}
    }
}

impl Shape for Plane {
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

    fn parent(&self) -> Option<Box<dyn Shape>> {
        self.parent.clone()
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

        // If the ray is parallel with the plane (including coplanar)
        // return an empty vec
        if t_ray.direction.y == Float(0.0) {
            return vec![]
        }

        let t = (t_ray.origin.y * -1.0) / t_ray.direction.y;
        return vec![Intersection::new(t.value(), Box::new(self.clone()))]
    }

    fn normal_at(&self, _world_point: &Tuple) -> Tuple {
        // Constant normal of an xy plane
        vector(0.0, 1.0, 0.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::point;

    #[test]
    fn plane_normal() {
        let p = Plane::new();
        let n1 = p.normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn plane_intersects() {
        // Ray is parallel to the plane
        let p = Plane::new();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersects(&r);
        assert!(xs.is_empty());

        // Ray is coplanar to plane
        let p = Plane::new();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersects(&r);
        assert!(xs.is_empty()); // Although really it intersects infinitely

        // Ray intersects the plane from above
        let p = Plane::new();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object.box_eq(p.as_any()));

        // Ray intersects the plane from below
        let p = Plane::new();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersects(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object.box_eq(p.as_any()));
    }
}