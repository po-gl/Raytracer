/// # Constructive Solid Geometry
/// `csg` is a module to add operators for shapes

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


#[derive(Debug, PartialEq, Clone)]
pub struct CSG {
    pub id: i32,
    pub shape_type: String,
    pub left_id: Option<i32>,
    pub right_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
    pub operation: Option<String>,
}

impl CSG {
    pub fn new(shape_list: &mut ShapeList) -> CSG {
        let id = shape_list.get_id();
        let shape = CSG { id, shape_type: String::from("csg"), parent_id: None, left_id: None, right_id: None,
            transform: Matrix4::identity(), material: Material::new(), operation: None};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_operation(operation: &str, left_id: i32, right_id: i32, shape_list: &mut ShapeList) -> CSG {
        let id = shape_list.get_id();

        shape_list.get(left_id).set_parent(id, shape_list);
        shape_list.get(right_id).set_parent(id, shape_list);

        let shape = CSG { id, parent_id: None, left_id: Some(left_id), right_id: Some(right_id),
            transform: Matrix4::identity(), material: Material::new(),
            operation: Some(String::from(operation))};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> CSG {
        let id = shape_list.get_id();
        let shape = CSG { id, parent_id: None, left_id: None, right_id: None,
            transform: Matrix4::identity(), material, operation: None};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn intersection_allowed(op: String, lhit: bool, inl: bool, inr: bool) -> bool {

        match op.as_ref() {
            "union" => (lhit && !inr) || (!lhit && !inl),
            "intersection" => (lhit && inr) || (!lhit && inl),
            "difference" => (lhit && !inr) || (!lhit && inl),

            _ => return false
        }
    }

    pub fn filter_intersects(&self, xs: &Vec<Intersection<Box<dyn Shape + Send>>>,
                             shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape + Send>>> {

        // Both children outside
        let mut inl = false;
        let mut inr = false;

        let mut result = vec![];

        for intersection in xs {
            // if the intersection's object is part of the left child, then lhit is true
            let object_id = intersection.object.id();
            let lhit = shape_list.get(self.left_id.unwrap()).includes(object_id);

            if CSG::intersection_allowed(self.operation.clone().unwrap(), lhit, inl, inr) {
                result.push(intersection.clone())
            }

            if lhit {
                inl = !inl
            } else {
                inr = !inr
            }
        }
        result
    }
}


impl Shape for CSG {
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
        self.left_id == Some(id) || self.right_id == Some(id)
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

    fn intersects(&self, ray: &Ray, shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape + Send>>> {
        // Transform the ray
        let t_ray = ray.transform(&self.transform.inverse());

        let left_child = shape_list.get(self.left_id.unwrap());
        let right_child = shape_list.get(self.right_id.unwrap());

        let mut leftxs = left_child.intersects(&t_ray, shape_list);
        let mut rightxs = right_child.intersects(&t_ray, shape_list);

        let mut xs = vec![];
        xs.append(&mut leftxs);
        xs.append(&mut rightxs);

        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        return self.filter_intersects(&xs, shape_list)
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
    use crate::shape::sphere::Sphere;
    use crate::shape::cube::Cube;
    use crate::tuple::vector;
    use crate::transformation::translation;

    #[test]
    fn csg_creation() {
        let mut shape_list = ShapeList::new();
        let s1 = Sphere::new(&mut shape_list);
        let s2 = Sphere::new(&mut shape_list);

        let c = CSG::new_with_operation("union", s1.id(), s2.id(), &mut shape_list);

        println!("Shape list {:#?}", shape_list);

        assert_eq!(c.operation.clone().unwrap(), "union");
        assert_eq!(c.left_id.unwrap().clone(), s1.id());
        assert_eq!(c.right_id.unwrap().clone(), s2.id());
        assert_eq!(shape_list.get(s1.id()).parent(&mut shape_list).unwrap().id(), c.id());
        assert_eq!(shape_list.get(s2.id()).parent(&mut shape_list).unwrap().id(), c.id());
    }

    #[test]
    fn csg_allow_intersection_union() {
        let table: Vec<(&str, bool, bool, bool, bool)> = vec![
            // op, lhit, inl, inr, result
            ("union", true, true, true, false),
            ("union", true, true, false, true),
            ("union", true, false, true, false),

            ("union", true, false, false, true),
            ("union", false, true, true, false),
            ("union", false, true, false, false),

            ("union", false, false, true, true),
            ("union", false, false, false, true),
        ];

        for i in 0..table.len() {
            let result = CSG::intersection_allowed(String::from(table[i].0), table[i].1, table[i].2, table[i].3);
            assert_eq!(result, table[i].4)
        }
    }

    #[test]
    fn csg_allow_intersection_intersection() {
        let table: Vec<(&str, bool, bool, bool, bool)> = vec![
            // op, lhit, inl, inr, result
            ("intersection", true, true, true, true),
            ("intersection", true, true, false, false),
            ("intersection", true, false, true, true),

            ("intersection", true, false, false, false),
            ("intersection", false, true, true, true),
            ("intersection", false, true, false, true),

            ("intersection", false, false, true, false),
            ("intersection", false, false, false, false),
        ];

        for i in 0..table.len() {
            let result = CSG::intersection_allowed(String::from(table[i].0), table[i].1, table[i].2, table[i].3);
            assert_eq!(result, table[i].4)
        }
    }

    #[test]
    fn csg_allow_intersection_difference() {
        let table: Vec<(&str, bool, bool, bool, bool)> = vec![
            // op, lhit, inl, inr, result
            ("difference", true, true, true, false),
            ("difference", true, true, false, true),
            ("difference", true, false, true, false),

            ("difference", true, false, false, true),
            ("difference", false, true, true, true),
            ("difference", false, true, false, true),

            ("difference", false, false, true, false),
            ("difference", false, false, false, false),
        ];

        for i in 0..table.len() {
            let result = CSG::intersection_allowed(String::from(table[i].0), table[i].1, table[i].2, table[i].3);
            assert_eq!(result, table[i].4)
        }
    }

    #[test]
    fn csg_intersection_filtering() {
        let table: Vec<(&str, usize, usize)> = vec![
            // op, x0, x1
            ("union", 0, 3),
            ("intersection", 1, 2),
            ("difference", 0, 1),
        ];

        for i in 0..table.len() {
            let shape_list = &mut ShapeList::new();
            let s1 = Sphere::new(shape_list);
            let s2 = Cube::new(shape_list);
            let c = CSG::new_with_operation(table[i].0, s1.id(), s2.id(), shape_list);
            let xs: Vec<Intersection<Box<dyn Shape + Send>>> = vec![
                Intersection::new(1.0, Box::new(s1.clone())),
                Intersection::new(2.0, Box::new(s2.clone())),
                Intersection::new(3.0, Box::new(s1.clone())),
                Intersection::new(4.0, Box::new(s2.clone())),
            ];
            let result = c.filter_intersects(&xs, shape_list);
            assert_eq!(result[0], xs[table[i].1]);
            assert_eq!(result[1], xs[table[i].2]);
        }
    }

    #[test]
    fn csg_ray_misses() {
        let shape_list = &mut ShapeList::new();
        let c = CSG::new_with_operation("union", Sphere::new(shape_list).id(), Cube::new(shape_list).id(), shape_list);
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = c.intersects(&r, shape_list);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn csg_ray_hits() {
        let shape_list = &mut ShapeList::new();
        let s1 = Sphere::new(shape_list);
        let mut s2 = Sphere::new(shape_list);
        s2.set_transform(translation(0.0, 0.0, 0.5), shape_list);

        let c = CSG::new_with_operation("union", s1.id(), s2.id(), shape_list);
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = c.intersects(&r, shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object.id(), s1.id());
        assert_eq!(xs[1].t, 6.5);
        assert_eq!(xs[1].object.id(), s2.id());
    }
}


