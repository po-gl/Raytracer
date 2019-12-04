/// # Group
/// `group` is a module to represent a group of shapes (or group of groups even)

use crate::shape::Shape;
use crate::ray::Ray;
use crate::shape;
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
    pub parent: Option<Box<dyn Shape>>,
    pub transform: Matrix4,
    pub material: Material,
    pub shapes: Vec<Box<dyn Shape>>
}

impl Group {
    pub fn new() -> Group {
        let id = shape::get_shape_id();
        Group {id, parent: None, transform: Matrix4::identity(), material: Material::new(), shapes: vec![]}
    }

    pub fn new_with_material(material: Material) -> Group {
        let id = shape::get_shape_id();
        Group{id, parent: None, transform: Matrix4::identity(), material, shapes: vec![]}
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn add_child(&mut self, child: &mut Box<dyn Shape>) {
        let mut temp_parent = self.clone();
        temp_parent.shapes.clear();
        let new_parent: Box<dyn Shape> = Box::new(temp_parent);
        child.set_parent(new_parent);
        self.shapes.push(child.clone());
    }
}

impl Shape for Group {
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

    fn set_parent(&mut self, parent: Box<dyn Shape>) -> Box<dyn Shape>{
        self.parent = Some(parent);
        Box::new(self.clone())
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

        let mut xs: Vec<Intersection<Box<dyn Shape>>> = vec![];
        for shape in self.shapes.iter() {
            xs.append(&mut shape.intersects(&t_ray)); // Or ray?
        }
        return xs
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
    use crate::shape::test_shape::TestShape;
    use crate::tuple::vector;
    use crate::shape::sphere::Sphere;
    use crate::transformation::{translation, scaling};

    #[test]
    fn groups_creation() {
        let g = Group::new();
        assert_eq!(g.transform, Matrix4::identity());
        assert!(g.is_empty())
    }

    #[test]
    fn groups_add_child() {
        let mut g = Group::new();
        let s = TestShape::new();
        let mut shape: Box<dyn Shape> = Box::new(s);
        g.add_child(&mut shape);
        assert!(!g.is_empty());

//        println!("Group: {:?}", g);
//        println!("Group shapes: {:?}", g.shapes);
//        println!("Shape : {:?}", shape);
//        println!("Shape0: {:?}", g.shapes[0]);
//        assert!(false);

//        assert_eq!(g.shapes[0], shape)
//        assert_eq!(Some(s.parent()), g);
    }

    #[test]
    fn groups_intersects_empty() {
        let g = Group::new();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersects(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn groups_intersects() {
        let mut g = Group::new();
        let s1: Box<dyn Shape> = Box::new(Sphere::new());
        let mut s2: Box<dyn Shape> = Box::new(Sphere::new());
        s2.set_transform(translation(0.0, 0.0, -3.0));
        let mut s3: Box<dyn Shape> = Box::new(Sphere::new());
        s3.set_transform(translation(5.0, 0.0, 0.0));
        g.add_child(&mut Box::new(s1.clone()));
        g.add_child(&mut Box::new(s2.clone()));
        g.add_child(&mut Box::new(s3.clone()));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut xs = g.intersects(&r);
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        assert_eq!(xs.len(), 4);

//        assert_eq!(xs[0].object, s2.shape_clone());
//        assert_eq!(xs[1].object, s2.shape_clone());
//        assert_eq!(xs[2].object, s1.shape_clone());
//        assert_eq!(xs[3].object, s1.shape_clone());

//        println!("Shape 1: {:?}", s1);
//        println!("Shape 2: {:?}", s2);
//        println!("Intersection: {:?}", xs[0]);
//        println!("Intersection: {:?}", xs[1]);
//        println!("Intersection: {:?}", xs[2]);
//        println!("Intersection: {:?}", xs[3]);
//        assert!(false);
    }

    #[test]
    fn groups_transformations() {
        let mut g = Group::new();
        g.set_transform(scaling(2.0, 2.0, 2.0));
        let mut s: Box<dyn Shape> = Box::new(Sphere::new());
        s.set_transform(translation(5.0, 0.0, 0.0));
        g.add_child(&mut s);
        let r = Ray::new(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersects(&r);
        assert_eq!(xs.len(), 2);
    }
}