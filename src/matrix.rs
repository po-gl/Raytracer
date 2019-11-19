/// # matrix
/// `matrix` is a module to represent a 4x4 matrix


use std::ops;
use std::ops::Index;
use super::float::Float;
use super::tuple::Tuple;

#[derive(Debug, PartialEq)]
struct Matrix4([[Float; 4]; 4]);

#[derive(Debug, PartialEq)]
struct Matrix3([[Float; 3]; 3]);

#[derive(Debug, PartialEq)]
struct Matrix2([[Float; 2]; 2]);

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

}

impl Index<usize> for Matrix4 {
    type Output = [Float; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
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
}

impl Index<usize> for Matrix3 {
    type Output = [Float; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index as usize]
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
