/// # transformation
/// `transoformation` is a module to represent matrix, point, and vector transformations

use super::float::Float;
use super::matrix::Matrix4;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[0][3] = Float(x);
    new_mat[1][3] = Float(y);
    new_mat[2][3] = Float(z);
    new_mat
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple;

    #[test]
    fn transformation_translation() {
        // Point
        let t = translation(5.0, -3.0, 2.0);
        let p = tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(t * p, tuple::point(2.0, 1.0, 7.0));

        let t = translation(5.0, -3.0, 2.0);
        let inv = t.inverse();
        let p = tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, tuple::point(-8.0, 7.0, 3.0));

        // Vector
        let t = translation(5.0, -3.0, 2.0);
        let v = tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(t * &v, v); // shouldn't change
    }

}