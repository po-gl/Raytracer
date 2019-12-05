/// # Shape
/// `shape` is the module containing all shape modules as well as the Shape trait


use crate::ray::Ray;
use std::sync::Mutex;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::{Tuple};
use std::any::Any;
use std::fmt::{Debug, Formatter, Error};
use crate::material::Material;
use crate::float::Float;
use crate::shape::shape_list::ShapeList;

pub mod shape_list;

pub mod test_shape;
pub mod sphere;
pub mod plane;
pub mod cube;
pub mod cylinder;
pub mod cone;
pub mod group;
pub mod triangle;


lazy_static! {
    static ref SHAPE_ID: Mutex<i32> = Mutex::new(-1);
}
pub fn get_shape_id() -> i32{
    let mut id = SHAPE_ID.lock().unwrap();
    *id += 1;
    *id
}

pub trait Shape: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_shape(&self) -> Box<&dyn Shape>;

    fn box_eq(&self, other: &dyn Any) -> bool;

    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;

    fn shape_clone(&self) -> Box<dyn Shape>;

    fn id(&self) -> i32;

    fn parent(&self, shape_list: &mut ShapeList) -> Option<Box<dyn Shape>>;

    fn set_parent(&mut self, parent_id: i32, shape_list: &mut ShapeList) ;

    fn transform(&self) -> Matrix4;

    fn set_transform(&mut self, transform: Matrix4, shape_list: &mut ShapeList);

    fn material(&self) -> Material;

    fn set_material(&mut self, material: Material, shape_list: &mut ShapeList);

    fn intersects(&self, ray: &Ray, shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape>>>;

    fn normal_at(&self, point: &Tuple) -> Tuple;
}

impl PartialEq for Box<dyn Shape> {
    fn eq(&self, other: &Box<dyn Shape>) -> bool {
//        self.box_eq(other.as_any())
        self.id() == other.id() &&
        self.material() == other.material() &&
        self.transform() == other.transform()
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

/// Recursively converts a point to its parent's point until
/// getting a world space point
pub fn world_to_object(shape: Box<dyn Shape>, point: Tuple, shape_list: &mut ShapeList) -> Tuple {
    let mut new_point = point;
    if shape.parent(shape_list) != None {
        new_point = world_to_object(shape.parent(shape_list).unwrap(), point, shape_list);
    }
    return shape.transform().inverse() * new_point;
}

/// Recursively convert a normal to world space
pub fn normal_to_world(shape: Box<dyn Shape>, normal: Tuple, shape_list: &mut ShapeList) -> Tuple {
    let mut new_normal: Tuple = shape.transform().inverse().transpose() * normal;
    new_normal.w = Float(0.0);
    new_normal = new_normal.normalize();

    if shape.parent(shape_list) != None {
        new_normal = normal_to_world(shape.parent(shape_list).unwrap(), new_normal, shape_list);
    }

    return new_normal
}

pub fn normal_at(shape: Box<dyn Shape>, world_point: Tuple, shape_list: &mut ShapeList) -> Tuple {
    let local_point = world_to_object(shape.clone(), world_point, shape_list);
    let local_normal = shape.normal_at(&local_point);
    return normal_to_world(shape, local_normal, shape_list);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::test_shape::TestShape;
    use crate::transformation;
    use crate::float::Float;
    use crate::shape::group::Group;
    use crate::transformation::{rotation_y, scaling, translation};
    use std::f64::consts::PI;
    use crate::shape::sphere::Sphere;
    use crate::shape::shape_list::ShapeList;
    use crate::tuple::{point, vector};

    #[test]
    fn shape_creation() {
        let mut shape_list = ShapeList::new();
        let s = TestShape::new(&mut shape_list);
        assert_eq!(s.transform, Matrix4::identity());

        let s = TestShape::new(&mut shape_list);
        let m = s.material.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn shape_transform() {
        let mut shape_list = ShapeList::new();
        let mut s = TestShape::new(&mut shape_list);
        s.set_transform(transformation::translation(2.0, 3.0, 4.0), &mut shape_list);
        assert_eq!(s.transform, transformation::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn shape_material() {
        let mut shape_list = ShapeList::new();
        let mut m = Material::new();
        m.ambient = Float(1.0);
        let mut s = TestShape::new(&mut shape_list);
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn shape_parent() {
        let mut shape_list = ShapeList::new();
        let s = TestShape::new(&mut shape_list);
        assert_eq!(s.parent_id, None);
    }

    #[test]
    fn shape_world_to_object() {
        let mut shape_list = ShapeList::new();
        let mut g1: Box<dyn Shape> = Box::new(Group::new(&mut shape_list));
        g1.set_transform(rotation_y(PI/2.0), &mut shape_list);
        let mut g2: Box<dyn Shape> = Box::new(Group::new(&mut shape_list));
        g2.set_transform(scaling(2.0, 2.0, 2.0), &mut shape_list);
        let mut s: Box<dyn Shape> = Box::new(Sphere::new(&mut shape_list));
        s.set_transform(translation(5.0, 0.0, 0.0), &mut shape_list);

        s.set_parent(g2.id(), &mut shape_list);
        g2.set_parent(g1.id(), &mut shape_list);

        let p = world_to_object(s, point(-2.0, 0.0, -10.0), &mut shape_list);
        assert_eq!(p, point(0.0, 0.0, -1.0));
    }
    
    #[test]
    fn shape_normal_to_world() {
        let mut shape_list = ShapeList::new();
        let mut g1: Box<dyn Shape> = Box::new(Group::new(&mut shape_list));
        g1.set_transform(rotation_y(PI/2.0), &mut shape_list);
        let mut g2: Box<dyn Shape> = Box::new(Group::new(&mut shape_list));
        g2.set_transform(scaling(1.0, 2.0, 3.0), &mut shape_list);
        let mut s: Box<dyn Shape> = Box::new(Sphere::new(&mut shape_list));
        s.set_transform(translation(5.0, 0.0, 0.0), &mut shape_list);

        s.set_parent(g2.id(), &mut shape_list);
        g2.set_parent(g1.id(), &mut shape_list);

        let n = normal_to_world(s, vector(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0), &mut shape_list);
        assert_eq!(n, vector(0.285714, 0.428571, -0.857142))
    }

    #[test]
    fn shape_normal_at_child() {
        let mut shape_list = ShapeList::new();
        let mut g1: Box<dyn Shape> = Box::new(Group::new(&mut shape_list));
        g1.set_transform(rotation_y(PI/2.0), &mut shape_list);
        let mut g2: Box<dyn Shape> = Box::new(Group::new(&mut shape_list));
        g2.set_transform(scaling(1.0, 2.0, 3.0), &mut shape_list);
        let mut s: Box<dyn Shape> = Box::new(Sphere::new(&mut shape_list));
        s.set_transform(translation(5.0, 0.0, 0.0), &mut shape_list);

        s.set_parent(g2.id(), &mut shape_list);
        g2.set_parent(g1.id(), &mut shape_list);

        let n = normal_at(s, point(1.7321, 1.1547, -5.5774), &mut shape_list);
        assert_eq!(n, vector(0.28570368, 0.428543, -0.857160))
    }

}
