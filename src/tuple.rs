/// # tuple
/// `tuple` is a module to represent our most basic data structure, an ordered list

use std::ops;
use super::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {x: Float(x), y: Float(y), z: Float(z), w: Float(w)}
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (&self.x * &self.x + &self.y * &self.y + &self.z * &self.z + &self.w * &self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        if Float(magnitude) == Float(0.0) {
            return Tuple::new(0.0, 0.0, 0.0, 0.0);
        } else {
            Tuple::new(&self.x.value() / magnitude, &self.y.value() / magnitude, &self.z.value() / magnitude, &self.w.value() / magnitude)
        }
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        self - normal * 2.0 * dot(self, normal)
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    (&a.x * &b.x + &a.y * &b.y + &a.z * &b.z + &a.w * &b.w).value()
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    vector((&a.y * &b.z - &a.z * &b.y).value(),
           (&a.z * &b.x - &a.x * &b.z).value(),
           (&a.x * &b.y - &a.y * &b.x).value())
}


// Addition
impl_op_ex!(+ |a: &Tuple, b: &Tuple| -> Tuple { Tuple {x: &a.x + &b.x, y: &a.y + &b.y, z: &a.z + &b.z, w: &a.w + &b.w} });

// Subtraction
impl_op_ex!(- |a: &Tuple, b: &Tuple| -> Tuple { Tuple {x: &a.x - &b.x, y: &a.y - &b.y, z: &a.z - &b.z, w: &a.w - &b.w} });

// Multiplication
impl_op_ex!(* |a: &Tuple, s: f64| -> Tuple { Tuple {x: &a.x * s, y: &a.y * s, z: &a.z * s, w: &a.w * s} });
//impl_op_ex!(* |a: &Tuple, b: &Tuple| -> Tuple { Tuple {x: &a.x * &b.x, y: &a.y * &b.y, z: &a.z * &b.z, w: &a.w * &b.w} });

// Division
impl_op_ex!(/ |a: &Tuple, s: f64| -> Tuple { Tuple {x: &a.x / s, y: &a.y / s, z: &a.z / s, w: &a.w / s} });
//impl_op_ex!(/ |a: &Tuple, b: &Tuple| -> Tuple { Tuple {x: &a.x / &b.x, y: &a.y / &b.y, z: &a.z / &b.z, w: &a.w / &b.w} });

// Negation (unary operator)
impl_op_ex!(- |a: &Tuple| -> Tuple { Tuple {x: 0.0 - &a.x, y: 0.0 - &a.y, z: 0.0 - &a.z, w: 0.0 - &a.w} });


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuples() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        assert!(a.is_point());
        assert!(!a.is_vector());

        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn tuple_eq() {
        let a = Tuple::new(43.8, 23.0, 1.0, 1.0);
        let b = Tuple::new(43.8, 23.0, 1.0, 1.0);
        assert_eq!(a, b);

        let a = Tuple::new(43.8, 23.0, 1.0, 1.0);
        let b = Tuple::new(43.8, 23.1, 1.0, 1.0);
        assert_ne!(a, b);

        let a = Tuple::new(43.8, 237.9, 324.0, 0.0);
        let b = Tuple::new(43.8, 237.9, 324.0, 0.0);
        assert_eq!(a, b);

        let a = Tuple::new(43.8, 23.0, 1.0, 1.0);
        let b = Tuple::new(43.8000000001, 23.0000000001, 1.00000000001, 1.0);
        assert_eq!(a, b);

        let a = Tuple::new(43.8000000001, 23.0000000001, 1.00000000001, 1.0);
        let b = Tuple::new(43.8, 23.0, 1.0, 1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn tuple_creation() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));

        let p = vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn tuple_operations() {
        // Addition
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a + b, Tuple::new(1.0, 1.0, 6.0, 1.0));

        // Subtraction
        let a = point(3.0, 2.0, 1.0);
        let b = point(5.0, 6.0, 7.0);
        assert_eq!(a - b, vector(-2.0, -4.0, -6.0));

        let a = point(3.0, 2.0, 1.0);
        let b = vector(5.0, 6.0, 7.0);
        assert_eq!(a - b, point(-2.0, -4.0, -6.0));

        let a = vector(3.0, 2.0, 1.0);
        let b = vector(5.0, 6.0, 7.0);
        assert_eq!(a - b, vector(-2.0, -4.0, -6.0));

        let a = vector(3.0, 2.0, 1.0);
        assert_eq!(-a, vector(-3.0, -2.0, -1.0));

        // Negation
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));

        // Scalar multiplication
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));

        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));

        // Scalar division
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));

        // Magnitude
        let a = vector(1.0, 0.0, 0.0);
        assert_eq!(a.magnitude(), 1.0);

        let a = vector(0.0, 1.0, 0.0);
        assert_eq!(a.magnitude(), 1.0);

        let a = vector(0.0, 0.0, 1.0);
        assert_eq!(a.magnitude(), 1.0);

        let a = vector(1.0, 2.0, 3.0);
        assert_eq!(a.magnitude(), 14.0f64.sqrt());

        // Normalize
        let a = vector(1.0, 2.0, 3.0);
        assert_eq!(a.normalize(), vector(0.26726, 0.53452, 0.80178));

        // Dot product
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(dot(&a, &b), 20.0);

        // Cross product
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(cross(&a, &b), vector(-1.0, 2.0, -1.0));
        assert_eq!(cross(&b, &a), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn tuple_reflect() {
        // Reflecting a vector approaching at a 45 deg angle
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));

        // Reflecting a vector off a slanted surface
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn tuple_performance() {

    }
}
