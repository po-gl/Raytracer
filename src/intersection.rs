/// # Intersection
/// `intersection` is a module to represent an intersection
/// the t value and the object that was intersected

use crate::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<T: Copy> {
    pub t: Float,
    pub object: T  // object that was intersected
}

impl<T: Copy> Intersection<T> {
    pub fn new(t: f64, object: T) -> Intersection<T> {
       Intersection {t: Float(t), object}
    }
}

/// A partial function that returns the intersection with the lowest t value
/// If all t values are negative, then None is returned
pub fn hit<T: Copy>(intersections: Vec<Intersection<T>>) -> Option<Intersection<T>> {
    assert!(intersections.len() > 0);
    let mut min_intersect: Intersection<T> = intersections[0];
    let mut min_t = Float::max();
    for intersect in intersections {
        if intersect.t > Float(0.0) && intersect.t < min_t {
            min_t = intersect.t;
            min_intersect = intersect;
        }
    }
    if min_t == Float::max() {
        None // If all intersect t's are negative return None
    } else {
        Some(min_intersect)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;

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
}