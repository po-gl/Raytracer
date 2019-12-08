/// # Cube
/// `cube` is a module to represent a cube shape

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
use crate::transformation::{translation, scaling};

#[derive(Debug, PartialEq, Clone)]
pub struct Cube {
    pub id: i32,
    pub shape_type: String,
    pub parent_id: Option<i32>,
    pub transform: Matrix4,
    pub material: Material,
}

impl Cube {
    pub fn new(shape_list: &mut ShapeList) -> Cube {
        let id = shape_list.get_id();
        let shape = Cube {id, shape_type: String::from("cube"), parent_id: None, transform: Matrix4::identity(), material: Material::new()};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_with_material(material: Material, shape_list: &mut ShapeList) -> Cube {
        let id = shape_list.get_id();
        let shape = Cube{id, shape_type: String::from("cube"), parent_id: None, transform: Matrix4::identity(), material};
        shape_list.push(Box::new(shape.clone()));
        shape
    }

    pub fn new_including_points(min_point: Tuple, max_point: Tuple, shape_list: &mut ShapeList) -> Cube {
        let id = shape_list.get_id();
        let mut shape = Cube {id, shape_type: String::from("cube"), parent_id: None, transform: Matrix4::identity(), material: Material::new()};
        shape_list.push(Box::new(shape.clone()));
        shape.transform_to_fit_points(min_point, max_point, shape_list);
        shape
    }

    pub fn transform_to_fit_points(&mut self, min: Tuple, max: Tuple, shape_list: &mut ShapeList) {
        // First get the center point of the cube
        let center: Tuple = (max + min) / 2.0;

        // Translate to the point and scale to points
        self.set_transform(
            translation(center.x.value(), center.y.value(), center.z.value()) *
                scaling((center.x.value() - max.x.value()).abs(),
                        (center.y.value() - max.y.value()).abs(),
                        (center.z.value() - max.z.value()).abs()), shape_list);
    }
}

impl Shape for Cube {
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

        let xtminmax = check_axis(t_ray.origin.x.value(), t_ray.direction.x.value());
        let ytminmax = check_axis(t_ray.origin.y.value(), t_ray.direction.y.value());
        let ztminmax = check_axis(t_ray.origin.z.value(), t_ray.direction.z.value());

        let tmin = xtminmax.0.max(ytminmax.0.max(ztminmax.0));
        let tmax = xtminmax.1.min(ytminmax.1.min(ztminmax.1));

        if tmin > tmax {
            return vec![]
        }

        vec![
            Intersection::new(tmin, Box::new(self.clone())),
            Intersection::new(tmax, Box::new(self.clone())),
        ]
    }

    fn normal_at(&self, object_point: &Tuple) -> Tuple {

        let maxc = object_point.x.value().abs().max(object_point.y.value().abs().max(object_point.z.value().abs()));

        if Float(maxc) == Float(object_point.x.value().abs()) {
            let mut normal = vector(object_point.x.value(), 0.0, 0.0);
            if self.material.normal_perturb.is_some() {
                let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                          object_point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
                normal = normal + perturb;
            }
            normal
        } else if Float(maxc) == Float(object_point.y.value().abs()) {
            let mut normal = vector(0.0, object_point.y.value(), 0.0);
            if self.material.normal_perturb.is_some() {
                let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                          object_point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
                normal = normal + perturb;
            }
            normal
        } else {
            let mut normal = vector(0.0, 0.0, object_point.z.value());
            if self.material.normal_perturb.is_some() {
                let perturb = NormalPerturber::perturb_normal(self.material.clone().normal_perturb.unwrap(),
                                                          object_point, self.material.clone().normal_perturb_factor, self.material.clone().normal_perturb_perlin);
                normal = normal + perturb;
            }
            normal
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin: f64;
    let mut tmax: f64;
    if direction.abs() >= FLOAT_THRESHOLD {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        if tmin_numerator > 0.0 {tmin = NumFloat::infinity()} else {tmin = NumFloat::neg_infinity()}
        if tmax_numerator > 0.0 {tmax = NumFloat::infinity()} else {tmax = NumFloat::neg_infinity()}
        if tmin_numerator == 0.0 {tmin = 0.0}
        if tmax_numerator == 0.0 {tmax = 0.0}
    }

    if Float(tmin) > Float(tmax) {
        // swap
        let temp = tmin;
        tmin = tmax;
        tmax = temp;
    }

    return (tmin, tmax)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::vector;
    use crate::tuple::point;
    use crate::shape;

    #[test]
    fn cube_intersects() {
        let examples = [
            // Origin, direction, t1, t2
            (point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0), // +x
            (point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0), // -x
            (point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0), // +y
            (point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0), // -y
            (point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0), // +z
            (point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0), // -z
            (point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0), // inside
        ];
        let mut shape_list = ShapeList::new();

        for i in 0..examples.len() {
            let c = Cube::new(&mut shape_list);
            let r = Ray::new(examples[i].0, examples[i].1);
            let xs = c.intersects(&r, &mut shape_list);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, examples[i].2);
            assert_eq!(xs[1].t, examples[i].3);
        }
    }
    
    #[test]
    fn cube_ray_misses_cube() {
        let examples = [
            // Origin, direction
            (point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
            (point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345)),
            (point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673)),
            (point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0)),
            (point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0)),
        ];
        let mut shape_list = ShapeList::new();

        for i in 0..examples.len() {
            let c = Cube::new(&mut shape_list);
            let r = Ray::new(examples[i].0, examples[i].1);
            let xs = c.intersects(&r, &mut shape_list);
            assert_eq!(xs.len(), 0);
        }
    }
    
    #[test]
    fn cube_normals() {
        let examples = [
            // point, normal
            (point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0)),
            (point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0)),
            (point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0)),
            (point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0)),
            (point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0)),
            (point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0)),
            (point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0)),
            (point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0)),
        ];
        let mut shape_list = ShapeList::new();

        for i in 0..examples.len() {
            let c = Cube::new(&mut shape_list);
            let p = examples[i].0;
            let normal = shape::normal_at(Box::new(c), p, &mut shape_list);
            assert_eq!(normal, examples[i].1)
        }
    }
    
    #[test]
    fn cube_fit_around_points() {
        let shape_list = &mut ShapeList::new();
        let mut c = Cube::new(shape_list);
        let min = point(-2.0, -2.0, -2.0);
        let max = point(1.0, 1.0, 2.0);
        c.transform_to_fit_points(min, max, shape_list);

        let r = Ray::new(point(-1.9, -1.9, -5.0), vector(0.0, 0.0, 1.0));
        let xs = c.intersects(&r, shape_list);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }
}