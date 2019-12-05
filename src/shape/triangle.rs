/// # Triangle
/// `triangle` is a module to represent a basic triangle shape

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::{tuple, FLOAT_THRESHOLD};
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::tuple::{Tuple};
use crate::float::Float;
use crate::shape::shape_list::ShapeList;

#[derive(Debug, PartialEq, Clone)]
pub struct Triangle {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,

    // 3 points
    pub p1: Tuple,
    pub p2: Tuple,
    pub p3: Tuple,

    // 2 edges
    pub e1: Tuple,
    pub e2: Tuple,

    pub normal: Tuple,
}

impl Triangle {
    pub fn new(p1: Tuple, p2: Tuple, p3: Tuple, shape_list: &mut ShapeList) -> Triangle {
        let id = shape_list.get_id();
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let shape = Triangle {id, parent_id: None, transform: Matrix4::identity(), material: Material::new(),
            p1, p2, p3, e1, e2, normal: tuple::cross(&e2, &e1).normalize()};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(p1: Tuple, p2: Tuple, p3: Tuple, material: Material, shape_list: &mut ShapeList) -> Triangle {
        let id = shape_list.get_id();
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let shape = Triangle {id, parent_id: None, transform: Matrix4::identity(), material,
            p1, p2, p3, e1, e2, normal: tuple::cross(&e2, &e1).normalize()};
        shape_list.push(Box::new(shape.clone()));
        shape
    }
}

impl Shape for Triangle {
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

        let dir_cross_e2 = tuple::cross(&t_ray.direction, &self.e2);
        let det = tuple::dot(&self.e1, &dir_cross_e2);
        if Float(det.abs()) < Float(FLOAT_THRESHOLD) {
            return vec![]
        }

        let f = 1.0 / det;
        let p1_to_origin = t_ray.origin - self.p1;
        let u = f * tuple::dot(&p1_to_origin, &dir_cross_e2);
        if Float(u) < Float(0.0) || Float(u) > Float(1.0) {
            return vec![] // miss the edge p1-p3
        }

        let origin_cross_e1 = tuple::cross(&p1_to_origin, &self.e1);
        let v = f * tuple::dot(&t_ray.direction, &origin_cross_e1);
        if Float(v) < Float(0.0) || Float(u + v) > Float(1.0) {
            return vec![] // miss the edge p2-p3
        }

        let t= f * tuple::dot(&self.e2, &origin_cross_e1);
        return vec![Intersection::new(t, Box::new(self.clone()))]
    }

    fn normal_at(&self, _point: &Tuple) -> Tuple {
        self.normal
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};

    #[test]
    fn triangle_creation() {
        let mut shape_list = ShapeList::new();
        let p1 = point(0.0, 1.0, 0.0);
        let p2 = point(-1.0, 0.0, 0.0);
        let p3 = point(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3, &mut shape_list);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, vector(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, vector(1.0, -1.0, 0.0));
        assert_eq!(t.normal, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn triangle_normal() {
        let mut shape_list = ShapeList::new();
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0), &mut shape_list);
        let n1 = t.normal_at(&point(0.0, 0.5, 0.0));
        let n2 = t.normal_at(&point(-0.5, 0.75, 0.0));
        let n3 = t.normal_at(&point(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn triangle_intersects() {
        let mut shape_list = ShapeList::new();
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0), &mut shape_list);
        let r = Ray::new(point(0.0, -1.0, -2.0), vector(0.0, 1.0, 0.0));
        let xs = t.intersects(&r, &mut shape_list);
        assert!(xs.is_empty());
    }

    #[test]
    fn triangle_ray_misses_p1_p3_edge() {
        let mut shape_list = ShapeList::new();
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0), &mut shape_list);
        let r = Ray::new(point(1.0, 1.0, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn triangle_ray_misses_p2_p3_edge() {
        let mut shape_list = ShapeList::new();
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0), &mut shape_list);
        let r = Ray::new(point(1.0, -1.0, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn trinagle_ray_strikes() {
        let mut shape_list = ShapeList::new();
        let t = Triangle::new(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0), &mut shape_list);
        let r = Ray::new(point(0.0, 0.5, -2.0), vector(0.0, 0.0, 1.0));
        let xs = t.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }
}