/// # Shape
/// `shape` is the module containing all shape modules as well as the Shape trait


use crate::ray::Ray;
use std::sync::Mutex;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::Tuple;

pub mod sphere;


lazy_static! {
    static ref SHAPE_ID: Mutex<i32> = Mutex::new(0);
}
pub fn get_shape_id() -> i32{
    let mut id = SHAPE_ID.lock().unwrap();
    *id += 1;
    *id
}


pub trait Shape<T: Copy> {
    fn intersects(&self, ray: Ray) -> Vec<Intersection<T>>;

    fn set_transform(&mut self, transform: Matrix4);

    fn normal_at(&self, point: Tuple) -> Tuple;
}