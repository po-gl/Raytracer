/// # Intersection
/// `intersection` is a module to represent an intersection
/// the t value and the object that was intersected

use crate::float::Float;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::shape::Shape;
use crate::{tuple, FLOAT_THRESHOLD, shape};
use crate::shape::shape_list::ShapeList;

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
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub n1: Float, // Refraction data
    pub n2: Float, // Refraction data
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

pub fn prepare_computations_single_intersection(intersection: Intersection<Box<dyn Shape + Send>>,
                                                ray: &Ray, shape_list: &mut ShapeList) -> PrecomputedData<Box<dyn Shape + Send>> {
    prepare_computations(intersection.clone(), ray, vec![intersection], shape_list)
}

pub fn prepare_computations(intersection: Intersection<Box<dyn Shape + Send>>, ray: &Ray,
                            intersections: Vec<Intersection<Box<dyn Shape + Send>>>, shape_list: &mut ShapeList) -> PrecomputedData<Box<dyn Shape + Send>> {

    let point = ray.position(intersection.t.value());
    let mut normalv =  shape::normal_at(intersection.object.clone(), point, shape_list);
    let eyev = -ray.direction;
    let inside = Float(tuple::dot(&normalv, &eyev)) < Float(0.0);

    // if inside, invert normal
    if inside {normalv = -normalv}

    let over_point = point + (normalv * FLOAT_THRESHOLD);
    let under_point = point - (normalv * FLOAT_THRESHOLD);

    let reflectv = ray.direction.reflect(&normalv);

    // Calculate n1 and n2 for refractions
    let mut n1 = Float(1.0);
    let mut n2 = Float(1.0);
    let mut container: Vec<Box<dyn Shape + Send>> = vec![];
    for inter in &intersections {
        let is_inter_hit = *inter == intersection;

        // 1. If the intersection is a hit set n1
        if is_inter_hit {
            if container.is_empty() {
                n1 = Float(1.0);
            } else {
                n1 = container.last().unwrap().material().clone().refractive_index;
            }
        }

        // 2. remove inter.object from container if it is present
        let mut is_object_present = false;
        for j in 0..container.len() {
            if inter.object == container[j].clone() {
                container.remove(j);
                is_object_present = true;
                break;
            }
        }
        // otherwise append it to container
        if !is_object_present {
            container.push(inter.object.clone());
        }

        // 3. If the intersection is a hit set n2
        if is_inter_hit {
            if container.is_empty() {
                n2 = Float(1.0);
            } else {
                n2 = container.last().unwrap().material().clone().refractive_index;
            }

            // 4. If the intersection is a hit, end the loop
            break;
        }
    }

    PrecomputedData {
        t: intersection.t,
        object: intersection.object,
        point,
        over_point,
        under_point,
        eyev,
        normalv,
        reflectv,
        inside,
        n1,
        n2,
    }
}

pub fn schlick(comps: PrecomputedData<Box<dyn Shape + Send>>) -> Float {
    // Find cosine of the angle between the eye and normal vectors
    let mut cos = tuple::dot(&comps.eyev, &comps.normalv);

    // Total internal reflection only occurs when n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n * n * (1.0 - cos * cos);
        if sin2_t > Float(1.0) {
            return Float(1.0)
        }

        // cosine of theta_t (using trig identity)
        let cos_t = (1.0 - sin2_t).sqrt();

        cos = cos_t;
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).value().powi(2);
    return Float(r0 + (1.0 - r0) * (1.0 - cos).powi(5));
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;
    use crate::tuple::{point, vector};
    use crate::{FLOAT_THRESHOLD, transformation};
    use crate::shape::plane::Plane;
    use crate::material::Material;
    use crate::transformation::{scaling, translation};
    use crate::shape::shape_list::ShapeList;

    #[test]
    fn intersection_creation() {
        let mut shape_list = ShapeList::new();
        let s = Sphere::new(&mut shape_list);
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn intersection_aggregation() {
        let mut shape_list = ShapeList::new();
        let s = Sphere::new(&mut shape_list);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersection_hits() {
        let mut shape_list = ShapeList::new();
        let s = Sphere::new(&mut shape_list);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let mut xs = vec![i1, i2];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, Some(i1));

        let s = Sphere::new(&mut shape_list);
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let mut xs = vec![i1, i2];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, Some(i2));

        let s = Sphere::new(&mut shape_list);
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let mut xs = vec![i1, i2];
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let i = hit(xs);
        assert_eq!(i, None);

        let s = Sphere::new(&mut shape_list);
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
        let mut shape_list = ShapeList::new();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        let i = Intersection::new(4.0, shape);
        let i_clone = i.clone();
        let comps = prepare_computations_single_intersection(i, &r, &mut shape_list);
        assert_eq!(&comps.t, &i_clone.t);
//        assert_eq!(comps.object, Box::new(Sphere::new(&mut shape_list)));
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));

        // If the hit occurs outside of the object
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations_single_intersection(i, &r, &mut shape_list);
        assert_eq!(comps.inside, false);

        // If the hit occurs inside the object
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape: Box<dyn Shape + Send> = Box::new(Sphere::new(&mut shape_list));
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations_single_intersection(i, &r, &mut shape_list);
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0)); // inverted from (0, 0, 1)
    }

    #[test]
    fn intersection_over_point() {
        let mut shape_list = ShapeList::new();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s1 = Sphere::new(&mut shape_list);
        s1.transform = transformation::translation(0.0, 0.0, 1.0);
        let shape: Box<dyn Shape + Send> = Box::new(s1);
        let i = Intersection::new(5.0, shape);
        let comps = prepare_computations_single_intersection(i, &r, &mut shape_list);
        assert!(comps.over_point.z < Float(-FLOAT_THRESHOLD/2.0));
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn intersection_reflection() {
        let mut shape_list = ShapeList::new();
        let shape: Box<dyn Shape + Send> = Box::new(Plane::new(&mut shape_list));
        let r = Ray::new(point(0.0, 1.0, -1.0), vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        let i = Intersection::new(2.0f64.sqrt(), shape);
        let comps = prepare_computations_single_intersection(i, &r, &mut shape_list);
        assert_eq!(comps.reflectv, vector(0.0, 2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0))
    }

    #[test]
    fn intersection_refraction() {
        let mut shape_list = ShapeList::new();
        let mut a = Sphere::new_with_material(Material::glass(), &mut shape_list);
        a.transform = scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = Float(1.5);
        let mut b = Sphere::new_with_material(Material::glass(), &mut shape_list);
        b.transform = translation(0.0, 0.0, -0.25);
        b.material.refractive_index = Float(2.0);
        let mut c = Sphere::new_with_material(Material::glass(), &mut shape_list);
        c.transform = translation(0.0, 0.0, 0.25);
        c.material.refractive_index = Float(2.5);

        let shape_a: Box<dyn Shape + Send> = Box::new(a.clone());
        let shape_b: Box<dyn Shape + Send> = Box::new(b.clone());
        let shape_c: Box<dyn Shape + Send> = Box::new(c.clone());

        let r = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(2.0, shape_a.clone()),
            Intersection::new(2.75, shape_b.clone()),
            Intersection::new(3.25, shape_c.clone()),
            Intersection::new(4.75, shape_b.clone()),
            Intersection::new(5.25, shape_c.clone()),
            Intersection::new(6.0, shape_a.clone()),
        ];

        let n_pairs = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for i in 0..n_pairs.len() {
            let comps = prepare_computations(xs[i].clone(), &r, xs.clone(), &mut shape_list);
            assert_eq!(comps.n1, Float(n_pairs[i].0));
            assert_eq!(comps.n2, Float(n_pairs[i].1));
        }
    }

    #[test]
    fn intersection_refraction_under_point() {
        let mut shape_list = ShapeList::new();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut a = Sphere::new_with_material(Material::glass(), &mut shape_list);
        a.transform = translation(0.0, 0.0, 1.0);
        let shape: Box<dyn Shape + Send> = Box::new(a.clone());
        let i = Intersection::new(5.0, shape);
        let xs = vec![i.clone()];
        let comps = prepare_computations(i.clone(), &r, xs.clone(), &mut shape_list);
        assert!(comps.under_point.z > Float(FLOAT_THRESHOLD/2.0));
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn intersection_schlick_total_internal_reflection() {
        let mut shape_list = ShapeList::new();
        let a = Sphere::new_with_material(Material::glass(), &mut shape_list);
        let shape: Box<dyn Shape + Send> = Box::new(a.clone());
        let r = Ray::new(point(0.0, 0.0, 2.0f64.sqrt()/2.0), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-2.0f64.sqrt()/2.0, shape.clone()),
            Intersection::new(2.0f64.sqrt()/2.0, shape.clone()),
        ];
        let comps = prepare_computations(xs[1].clone(), &r, xs.clone(), &mut shape_list);
        let reflectance = schlick(comps);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn intersection_schlick_perpendicular() {
        let mut shape_list = ShapeList::new();
        // reflectance should be small when a ray strikes at a perpendicular angle
        let a = Sphere::new_with_material(Material::glass(), &mut shape_list);
        let shape: Box<dyn Shape + Send> = Box::new(a.clone());
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-1.0, shape.clone()),
            Intersection::new(1.0, shape.clone()),
        ];
        let comps = prepare_computations(xs[1].clone(), &r, xs.clone(), &mut shape_list);
        let reflectance = schlick(comps);
        assert_eq!(reflectance, 0.04);
    }

    #[test]
    fn intersection_schlick_significant() {
        let mut shape_list = ShapeList::new();
        // When n2 > n1 and the ray strikes at a small angle,
        // reflectance should be significant
        let a = Sphere::new_with_material(Material::glass(), &mut shape_list);
        let shape: Box<dyn Shape + Send> = Box::new(a.clone());
        let r = Ray::new(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(1.8589, shape.clone()),
        ];
        let comps = prepare_computations(xs[0].clone(), &r, xs.clone(), &mut shape_list);
        let reflectance = schlick(comps);
        assert_eq!(reflectance, 0.48873);
    }
}