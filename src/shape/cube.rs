/// # Cube
/// `cube` is a module to represent a cube shape

use crate::shape::Shape;
use crate::ray::Ray;
use crate::{shape, FLOAT_THRESHOLD};
use crate::intersection::Intersection;
use crate::matrix::Matrix4;
use crate::tuple::{Tuple, vector};
use crate::float::Float;
use crate::material::Material;
use std::any::Any;
use std::fmt::{Formatter, Error};
use num_traits::float::Float as NumFloat;

#[derive(Debug, PartialEq, Clone)]
pub struct Cube {
    pub id: i32,
    pub parent: Option<Box<dyn Shape>>,
    pub transform: Matrix4,
    pub material: Material,
}

impl Cube {
    pub fn new() -> Cube {
        let id = shape::get_shape_id();
        Cube {id, parent: None, transform: Matrix4::identity(), material: Material::new()}
    }

    pub fn new_with_material(material: Material) -> Cube {
        let id = shape::get_shape_id();
        Cube{id, parent: None, transform: Matrix4::identity(), material}
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

    fn intersects(&self, ray: &Ray) -> Vec<Intersection<Box<dyn Shape>>> {
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

    fn normal_at(&self, world_point: &Tuple) -> Tuple {
        // Transform point to local space
        let point = self.transform.inverse() * world_point;

        let maxc = point.x.value().abs().max(point.y.value().abs().max(point.z.value().abs()));

        if Float(maxc) == Float(point.x.value().abs()) {
            return vector(point.x.value(), 0.0, 0.0)
        } else if Float(maxc) == Float(point.y.value().abs()) {
            return vector(0.0, point.y.value(), 0.0)
        } else {
            return vector(0.0, 0.0, point.z.value())
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

        for i in 0..examples.len() {
            let c = Cube::new();
            let r = Ray::new(examples[i].0, examples[i].1);
            let xs = c.intersects(&r);
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

        for i in 0..examples.len() {
            let c = Cube::new();
            let r = Ray::new(examples[i].0, examples[i].1);
            let xs = c.intersects(&r);
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

        for i in 0..examples.len() {
            let c = Cube::new();
            let p = examples[i].0;
            let normal = c.normal_at(&p);
            assert_eq!(normal, examples[i].1)
        }
    }
}