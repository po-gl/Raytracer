/// # Test Shape
/// `test_shape` is a module to test an abstract Shape object

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::tuple::{Tuple, vector};
use crate::shape::shape_list::ShapeList;

#[derive(Debug, PartialEq, Clone)]
pub struct TestShape {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
}

impl TestShape {
    pub fn new(shape_list: &mut ShapeList) -> TestShape {
        let id = shape_list.get_id();
        let shape = TestShape {id, parent_id: None, transform: Matrix4::identity(), material: Material::new()};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> TestShape {
        let id = shape_list.get_id();
        let shape = TestShape {id, parent_id: None, transform: Matrix4::identity(), material};
        shape_list.push(Box::new(shape.clone()));
        shape
    }
}

impl Shape for TestShape {
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

    fn shape_clone(&self) -> Box<dyn Shape + Send> {
        Box::new(self.clone())
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn parent(&self, shape_list: &mut ShapeList) -> Option<Box<dyn Shape + Send>> {
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

    fn intersects(&self, _ray: &Ray, _shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape + Send>>> {
        vec![]
    }

    fn normal_at(&self, _world_point: &Tuple) -> Tuple {
        vector(0.0, 0.0, 0.0)
    }
}