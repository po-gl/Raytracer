/// # Intersection
/// `intersection` is a module to represent an intersection
/// the t value and the object that was intersected

use crate::float::Float;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::shape::Shape;
use crate::{tuple, FLOAT_THRESHOLD};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<T> {
    pub t: Float,
    pub object: T  // object that was intersected
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PrecomputedData<T> {
    pub t: Float,
    pub object: T,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
}

impl<T> Intersection<T> {
    pub fn new(t: f64, object: T) -> Intersection<T> {
       Intersection {t: Float(t), object}
    }
}

/// A partial function that returns the intersection with the lowest t value
/// If all t values are negative, then None is returned
///
/// It is assumed that the vector is sorted ascending by t value
pub fn hit<T>(intersections: Vec<Intersection<T>>) -> Option<Intersection<T>> {
    if intersections.len() == 0 {
        return None
    }
    let mut min_intersect = None;
    let min_t = Float::max();
    for intersect in intersections {
        if intersect.t > Float(0.0) && intersect.t < min_t {
//            min_t = intersect.t;
            min_intersect = Some(intersect);

            // Optimization: return immediately at the first
            // non-negative t value
            return min_intersect
        }
    }
    if min_t == Float::max() {
        None // If all intersect t's are negative return None
    } else {
        min_intersect
    }
}

pub fn prepare_computations(intersection: Intersection<Box<dyn Shape>>, ray: &Ray) -> PrecomputedData<Box<dyn Shape>> {

    let point = ray.position(intersection.t.value());
    let mut normalv = intersection.object.normal_at(&point);
    let eyev = -ray.direction;
    let inside = Float(tuple::dot(&normalv, &eyev)) < Float(0.0);

    // if inside, invert normal
    if inside {normalv = -normalv}

    let over_point = point + (normalv * FLOAT_THRESHOLD);

    let reflectv = ray.direction.reflect(&normalv);

    PrecomputedData {
        t: intersection.t,
        object: intersection.object,
        point,
        over_point,
        eyev,
        normalv,
        reflectv,
        inside,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;
    use crate::tuple::{point, vector};
    use crate::{FLOAT_THRESHOLD, transformation};
    use crate::shape::plane::Plane;

    #[test]
    fn intersection_creation() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn intersection_aggregation() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersection_hits() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, Some(i1));

        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = vec![i1, i2];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, Some(i2));

        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let mut xs = vec![i1, i2];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, None);

        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2, i3, i4];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, Some(i4));
    }

    #[test]
    fn intersection_prep() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::new());
        let i = Intersection::new(4.0, shape);
        let i_clone = i.clone();
        let comps = prepare_computations(i, &r);
        assert_eq!(&comps.t, &i_clone.t);
//        assert_eq!(comps.object, Box::new(Sphere::new()));
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));

        // If the hit occurs outside of the object
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::new());
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(i, &r);
        assert_eq!(comps.inside, false);

        // If the hit occurs inside the object
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::new());
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(i, &r);
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0)); // inverted from (0, 0, 1)
    }

    #[test]
    fn intersection_over_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s1 = Sphere::new();
        s1.transform = transformation::translation(0.0, 0.0, 1.0);
        let shape: Box<dyn Shape> = Box::new(s1);
        let i = Intersection::new(5.0, shape);
        let comps = prepare_computations(i, &r);
        assert!(comps.over_point.z < Float(-FLOAT_THRESHOLD/2.0));
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn intersection_reflection() {
        let shape: Box<dyn Shape> = Box::new(Plane::new());
        let r = Ray::new(point(0.0, 1.0, -1.0), vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        let i = Intersection::new(2.0f64.sqrt(), shape);
        let comps = prepare_computations(i, &r);
        assert_eq!(comps.reflectv, vector(0.0, 2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0))
    }
}