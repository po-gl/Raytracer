/// # Shape
/// `shape` is the module containing all shape modules as well as the Shape trait


use crate::ray::Ray;
use std::sync::Mutex;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::Tuple;
use std::any::Any;
use std::fmt::{Debug, Formatter, Error};
use crate::material::Material;

pub mod sphere;


lazy_static! {
    static ref SHAPE_ID: Mutex<i32> = Mutex::new(0);
}
pub fn get_shape_id() -> i32{
    let mut id = SHAPE_ID.lock().unwrap();
    *id += 1;
    *id
}

pub trait Shape: Any {
    fn as_any(&self) -> &dyn Any;

    fn box_eq(&self, other: &dyn Any) -> bool;

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;

    fn shape_clone(&self) -> Box<dyn Shape>;

    fn id(&self) -> i32;

    fn transform(&self) -> Matrix4;

    fn material(&self) -> Material;

    fn intersects(&self, ray: &Ray) -> Vec<Intersection<Box<dyn Shape>>>;

    fn set_transform(&mut self, transform: Matrix4);

    fn normal_at(&self, point: &Tuple) -> Tuple;
}

impl PartialEq for Box<dyn Shape> {
    fn eq(&self, other: &Box<dyn Shape>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Debug for Box<dyn Shape> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.debug_fmt(f)
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.shape_clone()
    }
}