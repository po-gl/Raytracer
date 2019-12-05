/// # Plane
/// `plane` is a module to represent an xy plane

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::tuple::{Tuple, vector};
use crate::float::Float;
use crate::shape::shape_list::ShapeList;

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
}

impl Plane {
    pub fn new(shape_list: &mut ShapeList) -> Plane {
        let id = shape_list.get_id();
        let shape = Plane {id, parent_id: None, transform: Matrix4::identity(), material: Material::new()};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> Plane {
        let id = shape_list.get_id();
        let shape = Plane {id, parent_id: None, transform: Matrix4::identity(), material};
        shape_list.push(Box::new(shape.clone()));
        shape
    }
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_shape(&self) -> Box<&dyn Shape> {
        Box::new(self)
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

    fn parent(&self, shape_list: &mut ShapeList) -> Option<Box<dyn Shape>> {
        if self.parent_id.is_some() {
            Some(shape_list[self.parent_id.unwrap() as usize].clone())
        } else {
            None
        }
    }

    fn includes(&self, id: i32) -> bool {
        self.id == id
    }

    fn set_parent(&mut self, parent_id: i32, shape_list: &mut ShapeList) {
        self.parent_id = Some(parent_id);
        shape_list.update(Box::new(self.clone()));
    }

    fn transform(&self) -> Matrix4 {
        self.transform
    }


    fn set_transform(&mut self, transform: Matrix4, shape_list: &mut ShapeList) {
        self.transform = transform;
        shape_list.update(Box::new(self.clone()))
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: Material, shape_list: &mut ShapeList) {
        self.material = material;
        shape_list.update(Box::new(self.clone()))
    }

    fn intersects(&self, ray: &Ray, _shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape>>> {
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

    fn normal_at(&self, _point: &Tuple) -> Tuple {
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
        let mut shape_list = ShapeList::new();
        let p = Plane::new(&mut shape_list);
        let n1 = p.normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn plane_intersects() {
        let mut shape_list = ShapeList::new();
        // Ray is parallel to the plane
        let p = Plane::new(&mut shape_list);
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersects(&r, &mut shape_list);
        assert!(xs.is_empty());

        // Ray is coplanar to plane
        let p = Plane::new(&mut shape_list);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersects(&r, &mut shape_list);
        assert!(xs.is_empty()); // Although really it intersects infinitely

        // Ray intersects the plane from above
        let p = Plane::new(&mut shape_list);
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object.box_eq(p.as_any()));

        // Ray intersects the plane from below
        let p = Plane::new(&mut shape_list);
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object.box_eq(p.as_any()));
    }
}