/// # Intersection
/// `intersection` is a module to represent an intersection
/// the t value and the object that was intersected

use crate::float::Float;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::shape::Shape;
use crate::tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<T> {
    pub t: Float,
    pub object: T  // object that was intersected
}

pub struct PrecomputedData<T> {
    pub t: Float,
    pub object: T,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl<T> Intersection<T> {
    pub fn new(t: f64, object: T) -> Intersection<T> {
       Intersection {t: Float(t), object}
    }
}

/// A partial function that returns the intersection with the lowest t value
/// If all t values are negative, then None is returned
pub fn hit<T>(intersections: Vec<Intersection<T>>) -> Option<Intersection<T>> {
    if intersections.len() == 0 {
        return None
    }
    let mut min_intersect = None;
    let mut min_t = Float::max();
    for intersect in intersections {
        if intersect.t > Float(0.0) && intersect.t < min_t {
            min_t = intersect.t;
            min_intersect = Some(intersect);
        }
    }
    if min_t == Float::max() {
        None // If all intersect t's are negative return None
    } else {
        min_intersect
    }
}

pub fn prepare_computations<'a>(intersection: Intersection<&'a Box<dyn Shape>>, ray: &Ray) -> PrecomputedData<&'a Box<dyn Shape>> {

    let point = ray.position(intersection.t.value());
    let mut normalv = intersection.object.normal_at(&point);
    let eyev = -ray.direction;
    let inside = Float(tuple::dot(&normalv, &eyev)) < Float(0.0);

    // if inside, invert normal
    if inside {normalv = -normalv}


    PrecomputedData {
        t: intersection.t,
        object: intersection.object,
        point,
        eyev,
        normalv,
        inside,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;
    use crate::tuple::{point, vector};

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
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i, Some(i1));

        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i, Some(i2));

        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];
        let i = hit(xs);
        assert_eq!(i, None);

        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(xs);
        assert_eq!(i, Some(i4));
    }

    #[test]
    fn intersection_prep() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::new());
        let i = Intersection::new(4.0, &shape);
        let comps = prepare_computations(i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));

        // If the hit occurs outside of the object
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::new());
        let i = Intersection::new(4.0, &shape);
        let comps = prepare_computations(i, &r);
        assert_eq!(comps.inside, false);

        // If the hit occurs inside the object
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape> = Box::new(Sphere::new());
        let i = Intersection::new(4.0, &shape);
        let comps = prepare_computations(i, &r);
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0)); // inverted from (0, 0, 1)
    }
}