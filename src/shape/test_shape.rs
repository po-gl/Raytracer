/// # Test Shape
/// `test_shape` is a module to test an abstract Shape object

use crate::material::Material;
use crate::matrix::Matrix4;
use crate::{shape};
use crate::shape::Shape;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::tuple::{Tuple, vector};

#[derive(Debug, PartialEq, Clone)]
pub struct TestShape {
    pub id: i32,
    pub parent: Option<Box<dyn Shape>>,
    pub transform: Matrix4,
    pub material: Material,
}

impl TestShape {
    pub fn new() -> TestShape {
        let id = shape::get_shape_id();
        TestShape {id, parent: None, transform: Matrix4::identity(), material: Material::new()}
    }

    pub fn new_with_material(material: Material) -> TestShape {
        let id = shape::get_shape_id();
        TestShape {id, parent: None, transform: Matrix4::identity(), material}
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

    fn shape_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn parent(&self) -> Option<Box<dyn Shape>> {
        self.parent.clone()
    }

    fn set_parent(&mut self, parent: Box<dyn Shape>) {
        self.parent = Some(parent);
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

    fn intersects(&self, _ray: &Ray) -> Vec<Intersection<Box<dyn Shape>>> {
        vec![]
    }

    fn normal_at(&self, _world_point: &Tuple) -> Tuple {
        vector(0.0, 0.0, 0.0)
    }
}