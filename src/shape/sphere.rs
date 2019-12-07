/// # Sphere
/// `sphere` is a module to represent a sphere shape

use crate::shape::Shape;
use crate::ray::Ray;
use crate::tuple;
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::{Tuple, point};
use crate::float::Float;
use crate::material::Material;
use std::any::Any;
use std::fmt::{Formatter, Error};
use crate::shape::shape_list::ShapeList;
use crate::normal_perturber::NormalPerturber;


#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub id: i32,
    pub shape_type: String,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
}

impl Sphere {
    pub fn new(shape_list: &mut ShapeList) -> Sphere {
        let id = shape_list.get_id();
        let shape = Sphere {id, shape_type: String::from("sphere"), parent_id: None, transform: Matrix4::identity(), material: Material::new()};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> Sphere {
        let id = shape_list.get_id();
        let shape = Sphere{id, shape_type: String::from("sphere"), parent_id: None, transform: Matrix4::identity(), material};
        shape_list.push(Box::new(shape.clone()));
        shape
    }
}

impl Shape for Sphere {
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
        shape_list.update(Box::new(self.clone()));
    }

    fn intersects(&self, ray: &Ray, _shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape + Send>>> {
        // Transform the ray
        let t_ray = ray.transform(&self.transform.inverse());
        // vector from the sphere's center to the ray origin
        let sphere_to_ray =t_ray.origin - point(0.0, 0.0, 0.0);

        let a = tuple::dot(&t_ray.direction, &t_ray.direction);
        let b = 2.0 * tuple::dot(&t_ray.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if Float(discriminant) < Float(0.0) {
            return vec![Intersection::new(0.0, Box::new(self.clone())); 0]
        } else {
            let disc_sqrt = discriminant.sqrt();
            let t1 = (-b - disc_sqrt) / (2.0 * a);
            let t2 = (-b + disc_sqrt) / (2.0 * a);
            return vec![Intersection::new(t1, Box::new(self.clone())),
                        Intersection::new(t2, Box::new(self.clone()))];
        }
    }

    fn normal_at(&self, object_point: &Tuple) -> Tuple {
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = Float(0.0);
        if self.material.normal_perturb.is_some() {
            let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                          object_point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
            world_normal = world_normal + perturb;
        }
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformation;
    use crate::tuple::vector;

    #[test]
    fn sphere_intersection() {
        let mut shape_list = ShapeList::new();
        // Straight through
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new(&mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        // Just the top (tangent)
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new(&mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);

        // Missing the sphere
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new(&mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 0);

        // Starting inside the sphere
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new(&mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);

        // Starting after the sphere (should have negative t value)
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new(&mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);


        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new(&mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 2);
        assert!(s.box_eq(xs[0].object.as_any()));
        assert!(s.box_eq(xs[1].object.as_any()));
//        assert_eq!(&xs[0].object, &s);
//        assert_eq!(&xs[1].object, &s);
    }

    #[test]
    fn sphere_transforms() {
        let mut shape_list = ShapeList::new();
        let s = Sphere::new(&mut shape_list);
        assert_eq!(s.transform, Matrix4::identity());

        let mut s = Sphere::new(&mut shape_list);
        let t = transformation::translation(2.0, 3.0, 4.0);
        s.set_transform(t, &mut shape_list);
        assert_eq!(s.transform, t);

        // Intersecting a scaled sphere with a ray
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new(&mut shape_list);
        s.set_transform(transformation::scaling(2.0, 2.0, 2.0), &mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);

        // Intersecting a translated sphere with a ray
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new(&mut shape_list);
        s.set_transform(transformation::translation(5.0, 0.0, 0.0), &mut shape_list);
        let xs = s.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 0);
    }


    #[test]
    fn sphere_material() {
        let mut shape_list = ShapeList::new();
        let s = Sphere::new(&mut shape_list);
        let m = s.material;
        assert_eq!(m, Material::new());

        let mut s = Sphere::new(&mut shape_list);
        let mut m = Material::new();
        m.ambient = Float(1.0);
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn sphere_glassy_material() {
        let mut shape_list = ShapeList::new();
        let s = Sphere::new_with_material(Material::glass(), &mut shape_list);
        assert_eq!(s.transform, Matrix4::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}