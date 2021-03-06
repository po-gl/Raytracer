/// # Cone
/// `cone` is a module to represent a cone shape

use crate::shape::Shape;
use crate::ray::Ray;
use crate::{FLOAT_THRESHOLD};
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::{Tuple, vector};
use crate::float::Float;
use crate::material::Material;
use std::any::Any;
use std::fmt::{Formatter, Error};
use num_traits::float::Float as NumFloat;
use crate::shape::shape_list::ShapeList;
use crate::normal_perturber::NormalPerturber;

#[derive(Debug, PartialEq, Clone)]
pub struct Cone {
    pub id: i32,
    pub shape_type: String,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cone {
    pub fn new(shape_list: &mut ShapeList) -> Cone {
        let id = shape_list.get_id();
        let shape = Cone {id, shape_type: String::from("cone"), parent_id: None, transform: Matrix4::identity(), material: Material::new(), minimum: NumFloat::neg_infinity(), maximum: NumFloat::infinity(), closed: false};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> Cone {
        let id = shape_list.get_id();
        let shape = Cone{id, shape_type: String::from("cone"), parent_id: None, transform: Matrix4::identity(), material, minimum: NumFloat::neg_infinity(), maximum: NumFloat::infinity(), closed: false};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_bounded(minimum: f64, maximum: f64, shape_list: &mut ShapeList) -> Cone {
        let id = shape_list.get_id();
        let shape = Cone {id, shape_type: String::from("cone"), parent_id: None, transform: Matrix4::identity(), material: Material::new(), minimum, maximum, closed: false};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    /// Check if the intersection at t is within a radius of the y axis
    fn check_cap(ray: &Ray, t: Float, y: Float) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x * x + z * z) <= Float(y.value().abs())
    }

    fn intersect_caps(&self, ray: &Ray, xs: &mut Vec<Intersection<Box<dyn Shape + Send>>>) {
        if !self.closed {
            return // If a cylinder isn't closed, just return
        }

        // Check for an intersection with the lower cap
        let t = (self.minimum - ray.origin.y.value()) / ray.direction.y.value();
        if Cone::check_cap(ray, Float(t), Float(self.minimum)) {
            xs.push(Intersection::new(t, Box::new(self.clone())));
        }

        // Check for an intersection with the upper cap
        let t = (self.maximum - ray.origin.y.value()) / ray.direction.y.value();
        if Cone::check_cap(ray, Float(t), Float(self.maximum)) {
            xs.push(Intersection::new(t, Box::new(self.clone())));
        }
    }
}

impl Shape for Cone {
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
        shape_list.update(Box::new(self.clone()))
    }

    fn intersects(&self, ray: &Ray, _shape_list: &mut ShapeList) -> Vec<Intersection<Box<dyn Shape + Send>>> {
        // Transform the ray
        let t_ray = ray.transform(&self.transform.inverse());

        let a = (t_ray.direction.x * t_ray.direction.x -
            t_ray.direction.y * t_ray.direction.y +
            t_ray.direction.z * t_ray.direction.z).value();

        let b = (t_ray.origin.x * t_ray.direction.x * 2.0 -
            t_ray.origin.y * t_ray.direction.y * 2.0 +
            t_ray.origin.z * t_ray.direction.z * 2.0).value();

        // Ray misses walls of cylinder
        if a == Float(0.0) && b == Float(0.0) {
            // The walls are not intersected but the caps may be
            let mut xs: Vec<Intersection<Box<dyn Shape + Send>>> = vec![];
            self.intersect_caps(&t_ray, &mut xs);
            return xs
        }

        let c = (t_ray.origin.x * t_ray.origin.x -
            t_ray.origin.y * t_ray.origin.y +
            t_ray.origin.z * t_ray.origin.z).value();

        // If the ray misses the first half of the cone
        // but hits the next one, return a single point of intersection
        if a == Float(0.0) && b != Float(0.0) {
            let t = -c / (2.0 * b);
            let mut xs: Vec<Intersection<Box<dyn Shape + Send>>> = vec![];
            xs.push(Intersection::new(t, Box::new(self.clone())));
            self.intersect_caps(&t_ray, &mut xs);
            return xs;
        }


        let discriminant = b * b - 4.0 * a * c;

        if Float(discriminant) < Float(0.0) {  // Ray does not intersect the cylinder
            return vec![]
        } else {
            let disc_sqrt = discriminant.sqrt();
            let mut t0 = (-b - disc_sqrt) / (2.0 * a);
            let mut t1 = (-b + disc_sqrt) / (2.0 * a);

            if Float(t0) > Float(t1) {
                std::mem::swap(&mut t0, &mut t1);
            }

            let mut xs: Vec<Intersection<Box<dyn Shape + Send>>> = vec![];

            let y0 = t_ray.origin.y.value() + t0 * t_ray.direction.y.value();
            let y1 = t_ray.origin.y.value() + t1 * t_ray.direction.y.value();

            if Float(self.minimum) < Float(y0) && Float(y0) < Float(self.maximum) {
                xs.push(Intersection::new(t0, Box::new(self.clone())));
            }
            if Float(self.minimum) < Float(y1) && Float(y1) < Float(self.maximum) {
                xs.push(Intersection::new(t1, Box::new(self.clone())));
            }

            self.intersect_caps(&t_ray, &mut xs);

            return xs;
        }
    }

    fn normal_at(&self, point: &Tuple) -> Tuple {
        let distance = point.x * point.x + point.z * point.z;

        if distance < Float(1.0) && point.y >= Float(self.maximum) - FLOAT_THRESHOLD {
            let mut normal = vector(0.0, 1.0, 0.0); // Top cap
            if self.material.normal_perturb.is_some() {
                let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                              point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
                normal = normal + perturb;
            }
            normal
        } else if distance < Float(1.0) && point.y <= Float(self.minimum) + FLOAT_THRESHOLD {
            let mut normal =  vector(0.0, -1.0, 0.0); // Bottom cap
            if self.material.normal_perturb.is_some() {
                let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                              point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
                normal = normal + perturb;
            }
            normal
        } else {
            let mut y = (point.x * point.x + point.z * point.z).value().sqrt();
            if point.y > Float(0.0) {
                y = -y;
            }
            let mut normal = vector(point.x.value(), y, point.z.value());
            if self.material.normal_perturb.is_some() {
                let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                              point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
                normal = normal + perturb;
            }
            normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::point;

    #[test]
    fn cone_intersects() {
        let mut shape_list = ShapeList::new();
        let examples = vec![
            // origin, direction, t0, t1
            (point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0),
            (point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0), 8.66025, 8.66025),
            (point(1.0, 1.0, -5.0), vector(-0.5, -1.0, 1.0), 4.55006, 49.44994),
        ];

        for i in 0..examples.len() {
            let cone = Cone::new(&mut shape_list);
            let direction = examples[i].1.normalize();
            let r = Ray::new(examples[i].0, direction);
            let xs = cone.intersects(&r, &mut shape_list);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, examples[i].2);
            assert_eq!(xs[1].t, examples[i].3);
        }
    }

    #[test]
    fn cone_intersects_only_one_half() {
        // Intersecting a cone with a ray parallel to one of its halves
        let mut shape_list = ShapeList::new();
        let cone = Cone::new(&mut shape_list);
        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(point(0.0, 0.0, -1.0), direction);
        let xs = cone.intersects(&r, &mut shape_list);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 0.35355);
    }

    #[test]
    fn cone_intersects_cap() {
        let examples = vec![
            // origin, direction, count
            (point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4),
        ];

        let mut shape_list = ShapeList::new();
        for i in 0..examples.len() {
            let mut cone = Cone::new(&mut shape_list);
            cone.minimum = -0.5;
            cone.maximum = 0.5;
            cone.closed = true;
            let direction = examples[i].1.normalize();
            let r = Ray::new(examples[i].0, direction);
            let xs = cone.intersects(&r, &mut shape_list);
            assert_eq!(xs.len(), examples[i].2);
        }
    }

    #[test]
    fn cone_normal_at() {
        let examples = vec![
            // point, normal
            (point(0.0, 0.0, 0.0), vector(0.0, 0.0, 0.0)),
            (point(1.0, 1.0, 1.0), vector(1.0, -2.0f64.sqrt(), 1.0)),
            (point(-1.0, -1.0, 0.0), vector(-1.0, 1.0, 0.0)),
        ];
        let mut shape_list = ShapeList::new();

        for i in 0..examples.len() {
            let cone = Cone::new(&mut shape_list);
            let n = cone.normal_at(&examples[i].0);
            assert_eq!(n, examples[i].1);
        }
    }
}