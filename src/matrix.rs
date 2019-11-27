/// # matrix
/// `matrix` is a module to represent a 2x2, 3x3, and 4x4 matrices


use std::ops;
use std::ops::{Index, IndexMut};
use super::float::Float;
use super::tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix4([[Float; 4]; 4]);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix3([[Float; 3]; 3]);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix2([[Float; 2]; 2]);

// -------------------- 4x4 Matrix--------------------

impl Matrix4 {
    pub fn new(mat: [[f64; 4]; 4]) -> Matrix4 {
        let mut new_mat= [[Float(0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_mat[i][j] = Float(mat[i][j]);
            }
        }
        Matrix4(new_mat)
    }

    pub fn identity() -> Matrix4 {
        let mut new_mat= [[Float(0.0); 4]; 4];
        for i in 0..4 {
            new_mat[i][i] = Float(1.0);
        }
        Matrix4(new_mat)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut new_mat= [[Float(0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_mat[j][i] = self.0[i][j];
            }
        }
        Matrix4(new_mat)
    }

    /// Returns a sub matrix that has the given row and column removed
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut new_mut = [[Float(0.0); 3]; 3];

        let mut r :usize = 0;
        let mut c :usize;
        for i in 0..4 {
            c = 0;
            if i != row {
                for j in 0..4 {
                    if j != col {
                        new_mut[r][c] = self[i][j];
                        c += 1;
                    }
                }
                r += 1;
            }
        }
        Matrix3(new_mut)
    }

    /// Returns the determinant of the sub matrix that is
    /// not including the row and column provided
    pub fn minor(&self, row: usize, col: usize) -> Float {
        self.submatrix(row, col).determinant()
    }

    /// Returns the minor of a matrix that is negated depending on
    /// what row and column is removed
    pub fn cofactor(&self, row: usize, col: usize) -> Float {
        if (row + col + 1) % 2 == 0 { // If row + col is odd, negate
            self.minor(row, col) * -1.0
        } else {
            self.minor(row, col) * 1.0
        }
    }

    pub fn determinant(&self) -> Float {
        self[0][0] * self.cofactor(0, 0) +
        self[0][1] * self.cofactor(0, 1) +
        self[0][2] * self.cofactor(0, 2) +
        self[0][3] * self.cofactor(0, 3)
    }

    /// Returns the inverse of the matrix
    ///
    /// The process involves getting a matrix
    /// of co-factors, transposing the matrix, and
    /// dividing each element by the determinant of
    /// the original matrix. However, steps are
    /// combined here for efficiency.
    pub fn inverse(&self) -> Matrix4 {
        assert!(self.is_invertible());

        let mut new_mat= [[Float(0.0); 4]; 4];
        let determinant = self.determinant();

        for i in 0..4 {
            for j in 0..4 {
                let cofactor = self.cofactor(i, j);
                new_mat[j][i] = cofactor / determinant;
            }
        }
        Matrix4(new_mat)
    }
}

impl Index<usize> for Matrix4 {
    type Output = [Float; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// Matrix Multiplication
impl_op_ex!(* |a: &Matrix4, b: &Matrix4| -> Matrix4 {
    let mut prod_mat = [[Float(0.0); 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            prod_mat[r][c] = a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c] + a[r][3] * b[3][c];
        }
    }
    Matrix4(prod_mat)
});

// Tuple Multiplication
impl_op_ex!(* |a: &Matrix4, b: &Tuple| -> Tuple {
    Tuple {
        x: a[0][0] * b.x + a[0][1] * b.y + a[0][2] * b.z + a[0][3] * b.w,
        y: a[1][0] * b.x + a[1][1] * b.y + a[1][2] * b.z + a[1][3] * b.w,
        z: a[2][0] * b.x + a[2][1] * b.y + a[2][2] * b.z + a[2][3] * b.w,
        w: a[3][0] * b.x + a[3][1] * b.y + a[3][2] * b.z + a[3][3] * b.w,
    }
});

// -------------------- 3x3 Matrix--------------------

impl Matrix3 {
    pub fn new(mat: [[f64; 3]; 3]) -> Matrix3 {
        let mut new_mat= [[Float(0.0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_mat[i][j] = Float(mat[i][j]);
            }
        }
        Matrix3(new_mat)
    }

    /// Returns a sub matrix that has the given row and column removed
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut new_mut = [[Float(0.0); 2]; 2];

        let mut r :usize = 0;
        let mut c :usize;
        for i in 0..3 {
            c = 0;
            if i != row {
                for j in 0..3 {
                    if j != col {
                        new_mut[r][c] = self[i][j];
                        c += 1;
                    }
                }
                r += 1;
            }
        }
        Matrix2(new_mut)
    }

    /// Returns the determinant of the sub matrix that is
    /// not including the row and column provided
    pub fn minor(&self, row: usize, col: usize) -> Float {
        self.submatrix(row, col).determinant()
    }

    /// Returns the minor of a matrix that is negated depending on
    /// what row and column is removed
    pub fn cofactor(&self, row: usize, col: usize) -> Float {
        if (row + col + 1) % 2 == 0 { // If row + col is odd, negate
            self.minor(row, col) * -1.0
        } else {
            self.minor(row, col) * 1.0
        }
    }

    pub fn determinant(&self) -> Float {
        self[0][0] * self.cofactor(0, 0) +
        self[0][1] * self.cofactor(0, 1) +
        self[0][2] * self.cofactor(0, 2)
    }
}

impl Index<usize> for Matrix3 {
    type Output = [Float; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// -------------------- 2x2 Matrix--------------------

impl Matrix2 {
    pub fn new(mat: [[f64; 2]; 2]) -> Matrix2 {
        let mut new_mat= [[Float(0.0); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                new_mat[i][j] = Float(mat[i][j]);
            }
        }
        Matrix2(new_mat)
    }

    pub fn determinant(&self) -> Float {
        // ad - bc
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Index<usize> for Matrix2 {
    type Output = [Float; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn matrix_creation() {
        let m = Matrix4::new(
            [[1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);

        let m = Matrix2::new(
            [[-3.0, 5.0],
                [1.0, -2.0]]);
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);

        let m = Matrix3::new(
            [[-3.0, 5.0, 0.0],
                [1.0, -2.0, -7.0],
                [0.0, 1.0, 1.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn matrix_compare() {
        let a = Matrix4::new(
            [[1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]]);

        let b = Matrix4::new(
            [[1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.500000001]]);

        let c = Matrix4::new(
            [[8.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]]);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn matrix_operations() {
        // Multiplication
        let a = Matrix4::new(
            [[1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]]);

        let b = Matrix4::new(
            [[-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0]]);

        let c = Matrix4::new(
            [[20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]]);

        assert_eq!(a * b, c);


        // Tuple Multiplication
        let a = Matrix4::new(
            [[1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]]);

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));


        // Transpose
        let a = Matrix4::new(
            [[0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]]);

        let b = Matrix4::new(
            [[0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]]);

        assert_eq!(a.transpose(), b);

        let a = Matrix4::identity().transpose();
        assert_eq!(a, Matrix4::identity());


        // Determinants
        let a = Matrix2::new(
            [[1.0, 5.0],
                [-3.0, 2.0]]);
        assert_eq!(a.determinant(), 17.0);

        // Minor
        let a = Matrix3::new(
            [[3.0, 5.0, 0.0],
             [2.0, -1.0, -7.0],
             [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_eq!(a.minor(1, 0), b.determinant());

        // Co-factor
        let a = Matrix3::new(
            [[3.0, 5.0, 0.0],
             [2.0, -1.0, -7.0],
             [6.0, -1.0, 5.0]]);
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);

        // 3x3 determinant
        let a = Matrix3::new(
            [[1.0, 2.0, 6.0],
             [-5.0, 8.0, -4.0],
             [2.0, 6.0, 4.0]]);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);

        // 4x4 determinant (the big one)
        let a = Matrix4::new(
            [[-2.0, -8.0, 3.0, 5.0],
             [-3.0, 1.0, 7.0, 3.0],
             [1.0, 2.0, -9.0, 6.0],
             [-6.0, 7.0, 7.0, -9.0]]);
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn matrix_inverse_operations() {
        // Invertible test
        let a = Matrix4::new(
            [[6.0, 4.0, 4.0, 4.0],
             [5.0, 5.0, 7.0, 6.0],
             [4.0, -9.0, 3.0, -7.0],
             [9.0, 1.0, 7.0, -6.0]]);
        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());

        let a = Matrix4::new(
            [[-4.0, 2.0, -2.0, -3.0],
             [9.0, 6.0, 2.0, 6.0],
             [0.0, -5.0, 1.0, -5.0],
             [0.0, 0.0, 0.0, 0.0]]);
        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());

        // Inverse
        let a = Matrix4::new(
            [[-5.0, 2.0, 6.0, -8.0],
             [1.0, -5.0, 1.0, 8.0],
             [7.0, 7.0, -6.0, -7.0],
             [1.0, -3.0, 7.0, 4.0]]);
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[3][2], -160.0/532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[2][3], 105.0/532.0);

        let c = Matrix4::new(
            [[0.21805, 0.45113, 0.24060, -0.04511],
             [-0.80827, -1.45677, -0.44361, 0.52068],
             [-0.07895, -0.22368, -0.05263, 0.19737],
             [-0.52256, -0.81391, -0.30075, 0.30639]]);
        assert_eq!(b, c);
        
        // More inverse
        let a = Matrix4::new(
            [[8.0, -5.0, 9.0, 2.0],
             [7.0, 5.0, 6.0, 1.0],
             [-6.0, 0.0, 9.0, 6.0],
             [-3.0, 0.0, -9.0, -4.0]]);
        let b = Matrix4::new(
            [[-0.15385, -0.15385, -0.28205, -0.53846],
             [-0.07692, 0.12308, 0.02564, 0.03077],
             [0.35897, 0.35897, 0.43590, 0.92308],
             [-0.69231, -0.69231, -0.76923, -1.92308]]);
        assert_eq!(a.inverse(), b);

        let a = Matrix4::new(
            [[9.0, 3.0, 0.0, 9.0],
             [-5.0, -2.0, -6.0, -3.0],
             [-4.0, 9.0, 6.0, 4.0],
             [-7.0, 6.0, 6.0, 2.0]]);
        let b = Matrix4::new(
            [[-0.04074, -0.07778, 0.14444, -0.22222],
             [-0.07778, 0.03333, 0.36667, -0.33333],
             [-0.02901, -0.14630, -0.10926, 0.12963],
             [0.17778, 0.06667, -0.26667, 0.33333]]);
        assert_eq!(a.inverse(), b);

        let a = Matrix4::new(
            [[3.0, -9.0, 7.0, 3.0],
             [3.0, -8.0, 2.0, -9.0],
             [-4.0, 4.0, 4.0, 1.0],
             [-6.0, 5.0, -1.0, 1.0]]);
        let b = Matrix4::new(
            [[8.0, 2.0, 2.0, 2.0],
             [3.0, -1.0, 7.0, 0.0],
             [7.0, 0.0, 5.0, 4.0],
             [6.0, -2.0, 0.0, 5.0]]);
        let c = &a * &b;
        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn matrix_special_matrices() {
        // Identity
        let a = Matrix4::new(
            [[0.0, 1.0, 2.0, 4.0],
                [1.0, 2.0, 4.0, 8.0],
                [2.0, 4.0, 8.0, 16.0],
                [4.0, 8.0, 16.0, 32.0]]);

        assert_eq!(&a * Matrix4::identity(), a);


        // Sub-matrices
        let a = Matrix3::new(
            [[1.0, 5.0, 0.0],
             [-3.0, 2.0, 7.0],
             [0.0, 6.0, -3.0]]);

        let b = Matrix2::new(
            [[-3.0, 2.0],
             [0.0, 6.0]]);

        assert_eq!(a.submatrix(0, 2), b);

        let a = Matrix4::new(
            [[-6.0, 1.0, 1.0, 6.0],
             [-8.0, 5.0, 8.0, 6.0],
             [-1.0, 0.0, 8.0, 2.0],
             [-7.0, -1.0, -1.0, 1.0]]);

        let b = Matrix3::new(
            [[-6.0, 1.0, 6.0],
             [-8.0, 8.0, 6.0],
             [-7.0, -1.0, 1.0]]);

        assert_eq!(a.submatrix(2, 1), b);
    }
}
