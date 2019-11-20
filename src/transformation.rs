/// # transformation
/// `transoformation` is a module to represent matrix, point, and vector transformations

use super::float::Float;
use super::matrix::Matrix4;


/// Returns a 4x4 matrix used to translate either a tuple or matrix
/// by multiplication
pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[0][3] = Float(x);
    new_mat[1][3] = Float(y);
    new_mat[2][3] = Float(z);
    new_mat
}

/// Returns a 4x4 matrix used to scale either a tuple or matrix
/// by multiplication
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[0][0] = Float(x);
    new_mat[1][1] = Float(y);
    new_mat[2][2] = Float(z);
    new_mat
}

/// Returns a 4x4 matrix used to rotate around the x-axis
pub fn rotation_x(radians: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[1][1] = Float(radians.cos());
    new_mat[1][2] = Float(-radians.sin());
    new_mat[2][1] = Float(radians.sin());
    new_mat[2][2] = Float(radians.cos());
    new_mat
}

/// Returns a 4x4 matrix used to rotate around the y-axis
pub fn rotation_y(radians: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[0][0] = Float(radians.cos());
    new_mat[0][2] = Float(radians.sin());
    new_mat[2][0] = Float(-radians.sin());
    new_mat[2][2] = Float(radians.cos());
    new_mat
}

/// Returns a 4x4 matrix used to rotate around the z-axis
pub fn rotation_z(radians: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[0][0] = Float(radians.cos());
    new_mat[0][1] = Float(-radians.sin());
    new_mat[1][0] = Float(radians.sin());
    new_mat[1][1] = Float(radians.cos());
    new_mat
}

/// Returns a 4x4 matrix used in shearing
///
/// x_y denotes "x moved in proportion to y"
pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix4 {
    let mut new_mat = Matrix4::identity();
    new_mat[0][1] = Float(x_y);
    new_mat[0][2] = Float(x_z);
    new_mat[1][2] = Float(y_z);

    new_mat[1][0] = Float(y_x);
    new_mat[2][0] = Float(z_x);
    new_mat[2][1] = Float(z_y);
    new_mat
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple;
    use std::f64::consts::PI;

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

    #[test]
    fn transformation_scale() {
        // Point
        let t = scaling(2.0, 3.0, 4.0);
        let p = tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(t * p, tuple::point(-8.0, 18.0, 32.0));

        // Vector
        let t = scaling(2.0, 3.0, 4.0);
        let p = tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(t * p, tuple::vector(-8.0, 18.0, 32.0));

        let t = scaling(2.0, 3.0, 4.0);
        let inv = t.inverse();
        let p = tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * p, tuple::vector(-2.0, 2.0, 2.0));

        // Reflection
        let t = scaling(-1.0, 1.0, 1.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(t * p, tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn transformation_rotate() {
        // x-rotation
        let p = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI/4.0);
        let full_quarter = rotation_x(PI/2.0);
        assert_eq!(half_quarter * &p, tuple::point(0.0, 2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        assert_eq!(full_quarter * &p, tuple::point(0.0, 0.0, 1.0));

        let p = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI/4.0);
        let inv = half_quarter.inverse();
        assert_eq!(inv * p, tuple::point(0.0, 2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0));

        // y-rotation
        let p = tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI/4.0);
        let full_quarter = rotation_y(PI/2.0);
        assert_eq!(half_quarter * &p, tuple::point(2.0f64.sqrt()/2.0, 0.0, 2.0f64.sqrt()/2.0));
        assert_eq!(full_quarter * &p, tuple::point(1.0, 0.0, 0.0));

        // z-rotation
        let p = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI/4.0);
        let full_quarter = rotation_z(PI/2.0);
        assert_eq!(half_quarter * &p, tuple::point(-2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0, 0.0));
        assert_eq!(full_quarter * &p, tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn transformation_shearing() {
        // A shearing transformation moves x in proportion to y
        let s = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(s * p, tuple::point(5.0, 3.0, 4.0));

        // x in proportion to z
        let s = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(s * p, tuple::point(6.0, 3.0, 4.0));

        // y in proportion to x
        let s = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(s * p, tuple::point(2.0, 5.0, 4.0));

        // y in proportion to z
        let s = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(s * p, tuple::point(2.0, 7.0, 4.0));

        // z in proportion to x
        let s = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(s * p, tuple::point(2.0, 3.0, 6.0));

        // z in proportion to y
        let s = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        assert_eq!(s * p, tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn transformation_chaining() {
        let p = tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI/2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, tuple::point(15.0, 0.0, 7.0));


        let p = tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI/2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, tuple::point(15.0, 0.0, 7.0));

    }
}