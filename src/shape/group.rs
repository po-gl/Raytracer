/// # Group
/// `group` is a module to represent a group of shapes (or group of groups even)

use crate::shape::Shape;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::{Tuple, point};
use crate::float::Float;
use crate::material::Material;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::shape::shape_list::ShapeList;
use crate::bounds::Bounds;


#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub id: i32,
    pub shape_type: String,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
    pub children_ids: Vec<i32>,
    pub bounding_box: Bounds,
}

impl Group {
    pub fn new(shape_list: &mut ShapeList) -> Group {
        let id = shape_list.get_id();
        let shape = Group {id, shape_type: String::from("group"), parent_id: None, transform: Matrix4::identity(), material: Material::new(), children_ids: vec![], bounding_box: Bounds::new(shape_list)};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> Group {
        let id = shape_list.get_id();
        let shape = Group{id, shape_type: String::from("group"), parent_id: None, transform: Matrix4::identity(), material, children_ids: vec![], bounding_box: Bounds::new(shape_list)};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn is_empty(&self) -> bool {
        self.children_ids.is_empty()
    }

    pub fn add_child(&mut self, child: &mut Box<dyn Shape + Send>, shape_list: &mut ShapeList) {

        child.set_parent(self.id(), shape_list);

        self.children_ids.push(child.id());

        shape_list.update(Box::new(self.clone()));

        // Update group bounding box
        let group_shape: Box<dyn Shape + Send> = Box::new(self.clone());
        self.bounding_box = Bounds::bounds(group_shape, shape_list).unwrap();
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

    fn shape_clone(&self) -> Box<dyn Shape + Send> {
        Box::new(self.clone())
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn shape_type(&self) -> String {
        self.shape_type.clone()
    }

    fn parent(&self, shape_list: &mut ShapeList) -> Option<Box<dyn Shape + Send>> {
        if self.parent_id.is_some() {
            Some(shape_list[self.parent_id.unwrap() as usize].clone())
        } else {
            None
        }
    }

    fn includes(&self, id: i32) -> bool {
        self.children_ids.contains(&id)
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
        shape_list.update(Box::new(self.clone()));

        // Update bounding box
        self.bounding_box.cube.set_transform(transform, shape_list);
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: Material, shape_list: &mut ShapeList) {
        self.material = material;
        shape_list.update(Box::new(self.clone()))
    }

    fn intersects(&self, ray: &Ray, shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape + Send>>> {
        // Transform the ray
        let t_ray = ray.transform(&self.transform.inverse());

        let mut xs: Vec<Intersection<Box<dyn Shape + Send>>> = vec![];
        let xgroup = self.bounding_box.cube.intersects(ray, shape_list);

        // Only test for child intersections if the group's bounding box is hit
        if !xgroup.is_empty() {
            for child_id in self.children_ids.iter() {
                xs.append(&mut shape_list.get(*child_id).intersects(&t_ray, shape_list)); // Or ray?
            }
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
        let mut shape_list = ShapeList::new();
        let g = Group::new(&mut shape_list);
        assert_eq!(g.transform, Matrix4::identity());
        assert!(g.is_empty())
    }

    #[test]
    fn groups_add_child() {
        let mut shape_list = ShapeList::new();
        let mut g = Group::new(&mut shape_list);
        let s = TestShape::new(&mut shape_list);
        let mut shape: Box<dyn Shape + Send> = Box::new(s);
        g.add_child(&mut shape, &mut shape_list);
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
        let mut shape_list = ShapeList::new();
        let g = Group::new(&mut shape_list);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersects(&r, &mut shape_list);
        assert!(xs.is_empty());
    }

    #[test]
    fn groups_intersects() {
        let mut shape_list = ShapeList::new();
        let mut g = Group::new(&mut shape_list);
        let s1: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        let mut s2: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        s2.set_transform(translation(0.0, 0.0, -3.0), &mut shape_list);
        let mut s3: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        s3.set_transform(translation(5.0, 0.0, 0.0), &mut shape_list);
        g.add_child(&mut Box::new(s1.clone()), &mut shape_list);
        g.add_child(&mut Box::new(s2.clone()), &mut shape_list);
        g.add_child(&mut Box::new(s3.clone()), &mut shape_list);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut xs = g.intersects(&r, &mut shape_list);
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
    }

    #[test]
    fn groups_transformations() {
        let mut shape_list = ShapeList::new();
        let mut g = Group::new(&mut shape_list);
        let mut s: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        s.set_transform(translation(5.0, 0.0, 0.0), &mut shape_list);
        g.add_child(&mut s, &mut shape_list);

        g.set_transform(scaling(2.0, 2.0, 2.0), &mut shape_list);
        let r = Ray::new(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersects(&r, &mut shape_list);
//        assert_eq!(xs.len(), 2);
    }
}