/// # Bounds
/// `bounds` is a module to encapsulate a shape in a simpler shape (cube) for optimizing ray intersections

use crate::tuple::{Tuple, point};
use crate::shape::Shape;
use crate::shape::cone::Cone;
use crate::shape::cylinder::Cylinder;
use num_traits::float::Float as NumFloat;
use crate::shape::triangle::Triangle;
use crate::float::Float;
use crate::shape::group::Group;
use crate::shape::shape_list::ShapeList;
use crate::shape::cube::Cube;


#[derive(Debug, Clone)]
pub struct Bounds {
    pub min_point: Tuple,
    pub max_point: Tuple,
    pub cube: Cube,
}

impl Bounds {
    pub fn new(shape_list: &mut ShapeList) -> Bounds {
        Bounds {
            min_point: point(-1.0, -1.0, -1.0),
            max_point: point(1.0, 1.0, 1.0),
            cube: Cube::new(shape_list),
        }
    }

     pub fn new_with_bounds(min_point: Tuple, max_point: Tuple, shape_list: &mut ShapeList) -> Bounds {
         Bounds {
             min_point,
             max_point,
             cube: Cube::new_including_points(min_point, max_point, shape_list),
         }
     }

    /// Returns the bounds encapsulating a shape or group
    pub fn bounds(shape: Box<dyn Shape + Send>, shape_list: &mut ShapeList) -> Option<Bounds> {
        // Bounds are returned in Object space
        // un-transformed

        match shape.shape_type().as_ref() {
            "sphere"|"cube" => {
                Some(Bounds::new_with_bounds(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0), shape_list))
            }
            "plane" => {
                Some(Bounds::new_with_bounds(point(NumFloat::neg_infinity(), -0.01, NumFloat::neg_infinity()), point(NumFloat::infinity(), 0.01, NumFloat::infinity()), shape_list))
            }
            "cylinder" => {
                // Downcast to shape to work with cylinder properties
                let cylinder: &Cylinder = shape.as_any().downcast_ref::<Cylinder>().unwrap();
                let min: Tuple;
                let max: Tuple;

                if cylinder.minimum == Float(NumFloat::neg_infinity()) {
                    min = point(-1.0, NumFloat::neg_infinity(), -1.0);
                } else {
                    min = point(-1.0, cylinder.minimum, -1.0);
                }

                if cylinder.maximum == Float(NumFloat::infinity()) {
                    max = point(1.0, NumFloat::infinity(), 1.0);
                } else {
                    max = point(1.0, cylinder.maximum, 1.0);
                }
                Some(Bounds::new_with_bounds(min, max, shape_list))
            }
            "cone" => {
                // Downcast to shape to work with cone properties
                let cone: &Cone = shape.as_any().downcast_ref::<Cone>().unwrap();
                let min: Tuple;
                let max: Tuple;

                if cone.minimum == Float(NumFloat::neg_infinity()) {
                    min = point(-1.0, NumFloat::neg_infinity(), -1.0);
                } else {
                    min = point(-1.0, cone.minimum, -1.0);
                }

                if cone.maximum == Float(NumFloat::infinity()) {
                    max = point(1.0, NumFloat::infinity(), 1.0);
                } else {
                    max = point(1.0, cone.maximum, 1.0);
                }
                Some(Bounds::new_with_bounds(min, max, shape_list))
            }
            "triangle" => {
                // Downcast to shape to work with triangle properties
                let triangle: &Triangle = shape.as_any().downcast_ref::<Triangle>().unwrap();
                // Find lowest and highest x, y, and z values
                let mut l_x: f64 = NumFloat::infinity(); let mut h_x: f64 = NumFloat::neg_infinity();
                let mut l_y: f64 = NumFloat::infinity(); let mut h_y: f64 = NumFloat::neg_infinity();
                let mut l_z: f64 = NumFloat::infinity(); let mut h_z: f64 = NumFloat::neg_infinity();

                for point in [triangle.p1, triangle.p2, triangle.p3].iter() {
                    if point.x < Float(l_x) {
                        l_x = point.x.value();
                    }
                    if point.y < Float(l_y) {
                        l_y = point.y.value();
                    }
                    if point.z < Float(l_z) {
                        l_z = point.z.value();
                    }

                    if point.x > Float(h_x) {
                        h_x = point.x.value();
                    }
                    if point.y > Float(h_y) {
                        h_y = point.y.value();
                    }
                    if point.z > Float(h_z) {
                        h_z = point.z.value();
                    }
                }
                Some(Bounds::new_with_bounds(point(l_x, l_y, l_z), point(h_x, h_y, h_z), shape_list))
            }
            "group" => {
                // Here's the interesting bit
                // Downcast to group to work with group properties
                let group: &Group = shape.as_any().downcast_ref::<Group>().unwrap();

                // Find lowest and highest x, y, and z values
                let mut l_x: f64 = NumFloat::infinity(); let mut h_x: f64 = NumFloat::neg_infinity();
                let mut l_y: f64 = NumFloat::infinity(); let mut h_y: f64 = NumFloat::neg_infinity();
                let mut l_z: f64 = NumFloat::infinity(); let mut h_z: f64 = NumFloat::neg_infinity();

                for id in group.children_ids.clone() {
                    let child = shape_list.get(id);
                    let child_bounds = Bounds::bounds(child.clone(), shape_list);
                    // Transform child bounds from object space to group space
                    let group_min_point: Tuple = child.transform() * child_bounds.clone().unwrap().min_point;
                    let group_max_point: Tuple = child.transform() * child_bounds.clone().unwrap().max_point;

                    if group_min_point.x < Float(l_x) {
                        l_x = group_min_point.x.value();
                    }
                    if group_min_point.y < Float(l_y) {
                        l_y = group_min_point.y.value();
                    }
                    if group_min_point.z < Float(l_z) {
                        l_z = group_min_point.z.value();
                    }

                    if group_max_point.x > Float(h_x) {
                        h_x = group_max_point.x.value();
                    }
                    if group_max_point.y > Float(h_y) {
                        h_y = group_max_point.y.value();
                    }
                    if group_max_point.z > Float(h_z) {
                        h_z = group_max_point.z.value();
                    }
                }
                Some(Bounds::new_with_bounds(point(l_x, l_y, l_z), point(h_x, h_y, h_z), shape_list))
            }
            "test_shape" => {
                Some(Bounds::new(shape_list))
            }
            _ => {
                Some(Bounds::new(shape_list))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::shape_list::ShapeList;
    use crate::shape::sphere::Sphere;
    use crate::ray::Ray;
    use crate::tuple::vector;
    use crate::transformation::{translation, scaling};

    #[test]
    fn bounds_creation() {
        let shape_list = &mut ShapeList::new();
        let b = Bounds::new(shape_list);
        assert_eq!(b.min_point, point(-1.0, -1.0, -1.0));
        assert_eq!(b.max_point, point(1.0, 1.0, 1.0));
    }

    #[test]
    fn bounds_sphere() {
        let shape_list = &mut ShapeList::new();
        let s = Sphere::new(shape_list);
        let b = Bounds::bounds(Box::new(s.clone()), shape_list);

        // Ray shot through center of sphere and bounding box
        let r = Ray::new(point(0.0, 0.0, -1.5), vector(0.0, 0.0, 1.0));
        let xs = s.intersects(&r, shape_list);
        let xb = b.clone().unwrap().cube.intersects(&r, shape_list);

        assert_eq!(xs.len(), 2);
        assert_eq!(xb.len(), 2);
        assert_eq!(xs[0].t, 0.5);
        assert_eq!(xs[1].t, 2.5);
        assert_eq!(xb[0].t, 0.5);
        assert_eq!(xb[1].t, 2.5);

        // Ray shot at corner of bounding box, should miss sphere
        let r = Ray::new(point(0.2, 0.0, -1.5), vector(1.0, 0.0, 1.0));
        let xs = s.intersects(&r, shape_list);
        let xb = b.clone().unwrap().cube.intersects(&r, shape_list);

        assert_eq!(xs.len(), 0);
        assert_eq!(xb.len(), 2);
        assert_eq!(xb[0].t, 0.5);
        assert_eq!(xb[1].t, 0.8);
    }

    #[test]
    fn bounds_group_object() {
        let shape_list = &mut ShapeList::new();
        let mut s: Box<dyn Shape + Send> = Box::new(Sphere::new(shape_list));
        s.set_transform(translation(1.0, 2.0, 0.0) * scaling(0.5, 0.5, 3.0), shape_list);

        let mut group = Group::new(shape_list);
        group.add_child(&mut s, shape_list);
        let group_shape: Box<dyn Shape + Send> = Box::new(group.clone());

        let b = Bounds::bounds(group_shape, shape_list);

        // Ray shot through center of sphere and bounding box
        let r = Ray::new(point(1.0, 2.0, -3.0), vector(0.0, 0.0, 1.0));
        let xs = s.intersects(&r, shape_list);
        let xb = b.clone().unwrap().cube.intersects(&r, shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xb.len(), 2);
        assert_eq!(xs[0].t, 0.0);
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xb[0].t, 0.0);
        assert_eq!(xb[1].t, 6.0);


        let mut s2: Box<dyn Shape + Send> = Box::new(Sphere::new(shape_list));
        s2.set_transform(translation(-4.0, -3.0, 0.0) * scaling(0.5, 0.5, 3.0), shape_list);
        group.add_child(&mut s2, shape_list);
        let group_shape: Box<dyn Shape + Send> = Box::new(group.clone());

        let b = Bounds::bounds(group_shape, shape_list);

        // Ray hits second sphere at an angle and should also intersect with
        // the now larger group bounding box
        let r = Ray::new(point(-3.8, -3.0, -1.0), vector(-0.4, 0.0, 1.0));
        let xs = s2.intersects(&r, shape_list);
        let xb = b.clone().unwrap().cube.intersects(&r, shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xb.len(), 2);
        assert_eq!(xs[0].t, -0.5661449);
        assert_eq!(xs[1].t, 1.71407);
        assert_eq!(xb[0].t, -2.0);
        assert_eq!(xb[1].t, 1.75);

        // Ray hits first sphere in group as well
        let r = Ray::new(point(0.9, 2.0, -1.0), vector(0.4, 0.0, 1.0));
        let xs = s.intersects(&r, shape_list);
        let xb = b.clone().unwrap().cube.intersects(&r, shape_list);
        assert_eq!(xs.len(), 2);
        assert_eq!(xb.len(), 2);
        assert_eq!(xs[0].t, -0.761755);
        assert_eq!(xs[1].t, 1.48364);
        assert_eq!(xb[0].t, -2.0);
        assert_eq!(xb[1].t, 1.5);

        // Ray misses in-between both spheres but intersects bounding box
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = s.intersects(&r, shape_list);
        let xs2 = s2.intersects(&r, shape_list);
        let xb = b.clone().unwrap().cube.intersects(&r, shape_list);
        assert_eq!(xs.len(), 0);
        assert_eq!(xs2.len(), 0);
        assert_eq!(xb.len(), 2);
        assert_eq!(xb[0].t, 2.0);
        assert_eq!(xb[1].t, 8.0);
    }
}
