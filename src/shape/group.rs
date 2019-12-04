/// # Group
/// `group` is a module to represent a group of shapes (or group of groups even)

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
pub struct Group {
    pub id: i32,
    pub transform: Matrix4,
    pub material: Material,
    pub shapes: Vec<Box<dyn Shape>>
}

impl Group {
    pub fn new() -> Group {
        let id = shape::get_shape_id();
        Group {id, transform: Matrix4::identity(), material: Material::new(), shapes: vec![]}
    }

    pub fn new_with_material(material: Material) -> Group {
        let id = shape::get_shape_id();
        Group{id, transform: Matrix4::identity(), material, shapes: vec![]}
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }
}

impl Shape for Group {
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

    #[test]
    fn groups_creation() {
        let g = Group::new();
        assert_eq!(g.transform, Matrix4::identity());
        assert!(g.is_empty())
    }
}