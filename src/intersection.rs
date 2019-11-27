/// # Intersection
/// `intersection` is a module to represent an intersection
/// the t value and the object that was intersected

use crate::float::Float;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<T> {
    pub t: Float,
    pub object: T  // object that was intersected
}

impl<T> Intersection<T> {
    pub fn new(t: f64, object: T) -> Intersection<T> {
       Intersection {t: Float(t), object}
    }
}

pub fn intersections<T>(i1: Intersection<T>, i2: Intersection<T>) -> Vec<Intersection<T>> {
    vec![i1, i2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;
    use crate::tuple::point;

    #[test]
    fn intersection_creation() {
        let s = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn intersection_aggregation() {
        let s = Sphere::new(point(0.0, 0.0, 0.0), 1.0);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(i1, i2);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }
}