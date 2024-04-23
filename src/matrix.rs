use crate::prelude::*;
use std::fmt::Error;

#[derive(Debug, Clone)]
struct Matrix {
    pub points: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            points: vec![vec![0.0; columns as usize]; rows as usize],
        }
    }
    //TODO roll these into one function that takes a row of and just dynamically make the vecs instead
    pub fn new_4x4_from_rows(
        row1: Vec<f32>,
        row2: Vec<f32>,
        row3: Vec<f32>,
        row4: Vec<f32>,
    ) -> Self {
        let mut matrix = Self::new(4, 4);
        matrix.points[0] = row1;
        matrix.points[1] = row2;
        matrix.points[2] = row3;
        matrix.points[3] = row4;

        matrix
    }
    pub fn new_3x3_from_rows(row1: Vec<f32>, row2: Vec<f32>, row3: Vec<f32>) -> Self {
        let mut matrix = Self::new(3, 3);
        matrix.points[0] = row1;
        matrix.points[1] = row2;
        matrix.points[2] = row3;
        matrix
    }
    pub fn new_2x2_from_rows(row1: Vec<f32>, row2: Vec<f32>) -> Self {
        let mut matrix = Self::new(2, 2);
        matrix.points[0] = row1;
        matrix.points[1] = row2;

        matrix
    }
    pub fn from_identity() -> Self {
        Matrix::new_4x4_from_rows(
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        )
    }
    pub fn multiply_matrices(first: &Matrix, second: &Matrix) -> Matrix {
        let mut new_matrix = Matrix::new(first.points.len() as i32, first.points[0].len() as i32);

        for x in 0..first.points.len() {
            for y in 0..first.points[0].len() {
                new_matrix.points[x][y] = first.points[x][0] * second.points[0][y]
                    + first.points[x][1] * second.points[1][y]
                    + first.points[x][2] * second.points[2][y]
                    + first.points[x][3] * second.points[3][y];
            }
        }

        new_matrix
    }
    pub fn transposed_matrix(matrix: &Matrix) -> Self {
        let mut new_matrix = Matrix::new(matrix.points.len() as i32, matrix.points[0].len() as i32);
        for x in 0..new_matrix.points.len() {
            for y in 0..new_matrix.points[0].len() {
                new_matrix.points[x][y] = matrix.points[y][x];
            }
        }
        new_matrix
    }
    fn multiply_matrix_row(row: &Vec<f32>, other_row: &Vec<f32>) -> f32 {
        let mut total = 0.0;

        for x in 0..row.len() {
            total += row[x] * other_row[x];
        }

        total
    }
    fn submatrix(
        matrix: &Matrix,
        row_to_delete: i32,
        column_to_delete: i32,
    ) -> Result<Matrix, &str> {
        if row_to_delete > matrix.points.len() as i32
            || column_to_delete > matrix.points[0].len() as i32
        {
            return Err("Cannot delete a row that doesn't exist");
        }

        let mut new_matrix = Matrix::new(0, 0);

        //  fuck
        for x in 0..matrix.points.len() {
            if x != row_to_delete as usize {
                new_matrix.points.push(vec![]);
                for y in 0..matrix.points[0].len() {
                    if y != column_to_delete as usize {
                        new_matrix
                            .points
                            .last_mut()
                            .unwrap()
                            .push(matrix.points[x][y])
                    }
                }
            }
        }
        Ok(new_matrix)
    }
    fn minor(matrix: &Matrix, row: i32, column: i32) -> Result<f32, &str> {
        if matrix.points.len() >= 3 && matrix.points[0].len() >= 3 {
            let submatrix = Matrix::submatrix(matrix, row, column)?;
            Ok(submatrix.get_determinant())
        } else {
            Err("Matrix must be larger than 3x3 ")
        }
    }

    fn cofactor_3x3(matrix: &Matrix, row: i32, column: i32) -> Result<f32, &str> {
        if matrix.points.len() != 3 && matrix.points[0].len() != 3 {
            return Err("Must be a 3x3 matrix");
        }
        let is_even = row + column % 2 == 0;
        let minor = Matrix::minor(matrix, row, column)?;

        if is_even {
            Ok(minor)
        } else {
            Ok(-minor)
        }
    }

    fn cofactor_4x4(matrix: &Matrix, row: i32, column: i32) -> Result<f32, &str> {
        if matrix.points.len() != 4 || matrix.points[0].len() != 4 {
            return Err("Must be a 4x4 matrix");
        }

        let is_even = row + column % 2 == 0;
        let minor = Matrix::minor(matrix, row, column)?;

        if is_even {
            Ok(minor)
        } else {
            Ok(-minor)
        }
    }
    
    fn inverse(matrix: &Matrix) -> Result<Matrix,&str>{
        let length = matrix.points.len();
        if !matrix.is_invertible() { return Err("Cannot invert this matrix")};
        if length != 4 { return Err("Only 4x4 Matrices for now") }
        let mut new_matrix = Matrix::new(matrix.points.len() as i32,matrix.points[0].len() as i32);
        
        for row in 0.. length{
            for column in 0..new_matrix.points[0].len(){
                let cofactor = 
                    Matrix::cofactor_4x4(&matrix, row as i32,column as i32).expect("Err logically unreachable");
                // Col / Row flipped for transposition.
                new_matrix.points[column][row] = cofactor / Matrix::get_determinant(matrix);
            }
        } 
        
        Ok(new_matrix)
        
    }
    pub fn set_to(&mut self, matrix: &Matrix) {
        for x in 0..self.points.len() {
            for y in 0..self.points[0].len() {
                self.points[x][y] = matrix.points[x][y];
            }
        }
    }
    pub fn transpose(&mut self) {
        self.set_to(&Self::transposed_matrix(&self));
    }
    pub fn transposed(&self) -> Self {
        Self::transposed_matrix(self)
    }
    pub fn get_determinant(&self) -> f32 {
        // This is horrendous. probably worth refactoring cofactor to read what size the matrix is/ making all functions matrix size agnostic
        if self.points.len() == 2 && self.points[0].len() == 2 {
            self.points[0][0] * self.points[1][1] - (self.points[0][1] * self.points[1][0])
        } else {
            if self.points.len() == 3 && self.points[0].len() == 3 {
                let mut determinant = 0.0;
                for x in 0..self.points[0].len() {
                    let cofactor = Self::cofactor_3x3(&self, 0, x as i32).expect("Err is logically unreachable");
                    determinant += self.points[0][x] * cofactor;
                }
                determinant
            } else {
                {
                    let mut determinant = 0.0;
                    for x in 0..self.points[0].len() {
                        let cofactor = Self::cofactor_4x4(&self, 0, x as i32).expect("Err is logically unreachable");
                        determinant += self.points[0][x] * cofactor;
                    }
                    determinant
                }
            }
        }
    }
    pub fn multiply_by_matrix(&self, matrix: &Matrix) -> Matrix {
        if self.points.len() != matrix.points.len() {
            panic!("Matrices are not of same length")
        };
        Self::multiply_matrices(&self, matrix)
    }
    pub fn multiply_by_point(&self, point: &Point) -> (Point, f32) {
        if self.points.len() != 4 {
            panic!("Do not multiply a non 4x4 Matrix by a point")
        };
        let mut new_point = Point::zero();

        let point_row = vec![point.position.x, point.position.y, point.position.z, 1.0];
        new_point.position.x = Self::multiply_matrix_row(&self.points[0], &point_row);
        new_point.position.y = Self::multiply_matrix_row(&self.points[1], &point_row);
        new_point.position.z = Self::multiply_matrix_row(&self.points[2], &point_row);

        (
            new_point,
            Self::multiply_matrix_row(&self.points[3], &vec![1.0; 4]),
        )
    }
    pub fn multiply_by_vector(&self, vector: &Vector) -> (Vector, f32) {
        if self.points.len() != 4 {
            panic!("Do not multiply a non 4x4 Matrix by a vector")
        };
        let mut new_vector = Vector::zero();

        let point_row = vec![vector.x, vector.y, vector.z, 1.0];
        new_vector.x = Self::multiply_matrix_row(&self.points[0], &point_row);
        new_vector.y = Self::multiply_matrix_row(&self.points[1], &point_row);
        new_vector.z = Self::multiply_matrix_row(&self.points[2], &point_row);

        (
            new_vector,
            Self::multiply_matrix_row(&self.points[3], &vec![1.0; 4]),
        )
    }
    pub fn multiply_by_tuple(&self, tuple: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
        if self.points.len() != 4 {
            panic!("Do not multiply a non 4x4 matrix by a tuple")
        }
        let mut new_tuple = (0.0, 0.0, 0.0, 0.0);

        let tuple_row = vec![tuple.0, tuple.1, tuple.2, tuple.3];
        new_tuple.0 = Self::multiply_matrix_row(&self.points[0], &tuple_row);
        new_tuple.1 = Self::multiply_matrix_row(&self.points[1], &tuple_row);
        new_tuple.2 = Self::multiply_matrix_row(&self.points[2], &tuple_row);
        new_tuple.3 = Self::multiply_matrix_row(&self.points[3], &tuple_row);

        new_tuple
    }
    pub fn equals(&self, other: &Matrix) -> bool {
        if self.points.len() != other.points.len() {
            return false;
        };
        if self.points[0].len() != other.points[0].len() {
            return false;
        };

        for row in self.points.iter().enumerate() {
            let (x, _) = row;
            for point in row.1.iter().enumerate() {
                let (y, _) = point;
                if !crate::compare_float(other.points[x][y], self.points[x][y]) {
                    return false;
                }
            }
        }
        true
    }
    
    pub fn is_invertible(&self) -> bool {
        let determinant = self.get_determinant();
        !crate::compare_float(determinant,0.0)
    }
}

#[cfg(test)]
mod tests_matrix {
    use log::debug;
    use crate::matrix::Matrix;
    use crate::points::{Point, Vector};

    #[test]
    fn test_4_4() {
        let mut matrix4 = Matrix::new(4, 4);
        matrix4.points[0][0] = 11.2;
        matrix4.points[0][1] = 1.2;
        matrix4.points[3][3] = 22.2;
        assert_eq!(matrix4.points[0][0], 11.2);
        assert_eq!(matrix4.points[3][3], 22.2);
        assert_eq!(matrix4.points[1][3], 0.0);
    }
    #[test]
    fn test_2_2() {
        let matrix2 = Matrix::new(2, 2);
        assert_eq!(matrix2.points[0][0], 0.0);
    }
    #[test]
    fn test_matrix_comparison() {
        let mut matrix1 = Matrix::new(4, 4);
        matrix1.points[0] = vec![1.0, 2.0, 3.0, 4.0];
        matrix1.points[1] = vec![5.0, 6.0, 7.0, 8.0];
        matrix1.points[2] = vec![9.0, 8.0, 7.0, 6.0];
        matrix1.points[3] = vec![5.0, 4.0, 3.0, 2.0];

        let mut matrix2 = Matrix::new(4, 4);
        matrix2.points[0] = vec![1.0, 2.0, 3.0, 4.0];
        matrix2.points[1] = vec![5.0, 6.0, 7.0, 8.0];
        matrix2.points[2] = vec![9.0, 8.0, 7.0, 6.0];
        matrix2.points[3] = vec![5.0, 4.0, 3.0, 2.0];

        let mut matrix3 = Matrix::new(4, 4);
        matrix3.points[0] = vec![2.0, 2.0, 3.0, 4.0];
        matrix3.points[1] = vec![3.0, 6.0, 7.0, 8.0];
        matrix3.points[2] = vec![4.0, 8.0, 7.0, 6.0];
        matrix3.points[3] = vec![6.0, 4.0, 3.0, 2.0];

        assert!(matrix1.equals(&matrix2));
        assert!(!matrix1.equals(&matrix3));
        assert!(matrix1.equals(&matrix1));
        assert!(!matrix1.equals(&Matrix::new(4, 4)));
    }
    #[test]
    fn test_matrix_multiplication() {
        let mut matrix1 = Matrix::new(4, 4);
        matrix1.points[0] = vec![1.0, 2.0, 3.0, 4.0];
        matrix1.points[1] = vec![5.0, 6.0, 7.0, 8.0];
        matrix1.points[2] = vec![9.0, 8.0, 7.0, 6.0];
        matrix1.points[3] = vec![5.0, 4.0, 3.0, 2.0];

        let mut matrix2 = Matrix::new(4, 4);
        matrix2.points[0] = vec![-2.0, 1.0, 2.0, 3.0];
        matrix2.points[1] = vec![3.0, 2.0, 1.0, -1.0];
        matrix2.points[2] = vec![4.0, 3.0, 6.0, 5.0];
        matrix2.points[3] = vec![1.0, 2.0, 7.0, 8.0];

        let mut correct_matrix = Matrix::new(4, 4);
        correct_matrix.points[0] = vec![20.0, 22.0, 50.0, 48.0];
        correct_matrix.points[1] = vec![44.0, 54.0, 114.0, 108.0];
        correct_matrix.points[2] = vec![40.0, 58.0, 110.0, 102.0];
        correct_matrix.points[3] = vec![16.0, 26.0, 46.0, 42.0];
        println!("{:?}", Matrix::multiply_matrices(&matrix1, &matrix2));
        println!("{:?}", correct_matrix);
        assert!(correct_matrix.equals(&Matrix::multiply_matrices(&matrix1, &matrix2)))
    }
    #[test]
    fn test_matrix_tuple_multiplication() {
        let mut matrix = Matrix::new(4, 4);
        matrix.points[0] = vec![1.0, 2.0, 3.0, 4.0];
        matrix.points[1] = vec![2.0, 4.0, 4.0, 2.0];
        matrix.points[2] = vec![8.0, 6.0, 4.0, 1.0];
        matrix.points[3] = vec![0.0, 0.0, 0.0, 1.0];

        // Point x matrix
        let point = Point::new(1.0, 2.0, 3.0);
        let correct_answer = Point::new(18.0, 24.0, 33.0);

        let computed_answer = matrix.multiply_by_point(&point);
        assert!(correct_answer.compare(&computed_answer.0));

        // Vector x Matrix
        let vector = Vector::new(1.0, 2.0, 3.0);
        let computed_answer = matrix.multiply_by_vector(&vector);
        let correct_answer = Vector::new(18.0, 24.0, 33.0);

        assert!(correct_answer.is_same(&computed_answer.0));

        // Tuple x Matrix
        let tuple = (1.0, 2.0, 3.0, 1.0);
        let comp_tuple = matrix.multiply_by_tuple(tuple);
        let correct_tup_answer = (18.0, 24.0, 33.0, 1.0);

        assert_eq!(comp_tuple, correct_tup_answer);
    }
    #[test]
    fn test_multiplicative_identity() {
        let mut matrix = Matrix::new(4, 4);
        matrix.points[0] = vec![0.0, 1.0, 2.0, 3.0];
        matrix.points[1] = vec![1.0, 2.0, 4.0, 8.0];
        matrix.points[2] = vec![2.0, 4.0, 8.0, 16.0];
        matrix.points[3] = vec![4.0, 8.0, 16.0, 32.0];

        let matrix2 = matrix.multiply_by_matrix(&Matrix::from_identity());

        assert!(matrix.equals(&matrix2));

        let tuple = (1.0, 2.0, 3.0, 4.0);

        let identity_matrix = Matrix::from_identity();
        let new_tuple = identity_matrix.multiply_by_tuple(tuple);
        assert_eq!(tuple, new_tuple);
    }
    #[test]
    fn test_matrix_transposition() {
        let mut matrix = Matrix::new_4x4_from_rows(
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        );
        let transposed_matrix = Matrix::new_4x4_from_rows(
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        );

        matrix.transpose();
        assert!(transposed_matrix.equals(&matrix));

        // Test IdentityMatrix transposition
        let mut matrix = Matrix::from_identity();
        matrix.transpose();
        assert!(Matrix::from_identity().equals(&matrix));
    }
    #[test]
    fn test_invert_matrix() {}
    #[test]
    fn test_find_determinant_of_2x2() {
        let mut matrix = Matrix::new(2, 2);
        matrix.points[0] = vec![1.0, 5.0];
        matrix.points[1] = vec![-3.0, 2.0];

        assert_eq!(matrix.get_determinant(), 17.0)
    }
    #[test]
    fn test_submatrix_3x3_to_2x2() {
        let matrix = Matrix::new_3x3_from_rows(
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        );

        let correct_submatrix = Matrix::new_2x2_from_rows(vec![-3.0, 2.0], vec![0.0, 6.0]);
        let computed_submatrix = Matrix::submatrix(&matrix, 0, 2);

        println!("{:?} {:?}", correct_submatrix, computed_submatrix);
        assert!(computed_submatrix
            .expect("Test data, should not panic")
            .equals(&correct_submatrix));
    }
    #[test]
    fn test_submatrix_4x4_to_3x3() {
        let matrix = Matrix::new_4x4_from_rows(
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        );

        let correct_submatrix = Matrix::new_3x3_from_rows(
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        );

        let computed_submatrix = Matrix::submatrix(&matrix, 2, 1);
        assert!(computed_submatrix
            .expect("Test data, should not panic")
            .equals(&correct_submatrix));
    }
    #[test]
    fn test_minor_3x3() {
        let matrix = Matrix::new_3x3_from_rows(
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        );

        let b_matrix = Matrix::submatrix(&matrix, 1, 0);
        let b_matrix = b_matrix.expect("Test data, should not panic");

        let correct_determinant = 25.0;
        assert_eq!(
            correct_determinant,
            b_matrix.get_determinant()
        );
        assert_eq!(
            correct_determinant,
            Matrix::minor(&matrix, 1, 0).expect("Test data is valid")
        )
    }
    #[test]
    fn test_cofactors_3x3() {
        let matrix = Matrix::new_3x3_from_rows(
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        );

        let minor_1 = Matrix::minor(&matrix, 0, 0);
        let cofactor_1 = Matrix::cofactor_3x3(&matrix, 0, 0);
        let minor_2 = Matrix::minor(&matrix, 1, 0);
        let cofactor_2 = Matrix::cofactor_3x3(&matrix, 1, 0);

        assert_eq!(minor_1.expect("Test data is valid"), -12.0);
        assert_eq!(minor_2.expect("Test data is valid"), 25.0);
        assert_eq!(cofactor_1.expect("Test data is valid"), -12.0);
        assert_eq!(cofactor_2.expect("Test data is valid"), -25.0);
    }

    #[test]
    fn test_determinant_3x3() {
        let matrix_1 = Matrix::new_3x3_from_rows(
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        );

        assert_eq!(
            Matrix::cofactor_3x3(&matrix_1, 0, 0).expect("Test Data"),
            56.0
        );
        assert_eq!(
            Matrix::cofactor_3x3(&matrix_1, 0, 1).expect("Test Data"),
            12.0
        );
        assert_eq!(
            Matrix::cofactor_3x3(&matrix_1, 0, 2).expect("Test Data"),
            -46.0
        );
        assert_eq!(matrix_1.get_determinant(), -196.0);
    }
    #[test]
    fn test_determinant_4x4() {
        let matrix = Matrix::new_4x4_from_rows(
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        );

        assert_eq!(
            Matrix::cofactor_4x4(&matrix, 0, 0).expect("Test Data"),
            690.0
        );
        assert_eq!(
            Matrix::cofactor_4x4(&matrix, 0, 1).expect("Test Data"),
            447.0
        );
        assert_eq!(
            Matrix::cofactor_4x4(&matrix, 0, 2).expect("Test Data"),
            210.0
        );
        assert_eq!(
            Matrix::cofactor_4x4(&matrix, 0, 3).expect("Test Data"),
            51.0
        );
        assert_eq!(matrix.get_determinant(), -4071.0);
    }
    
    #[test]
    fn test_identify_invertible(){
        let matrix = Matrix::new_4x4_from_rows(
            vec![6.0,4.0,4.0,4.0],
            vec![5.0,5.0,7.0,6.0],
            vec![4.0,-9.0,3.0,-7.0],
            vec![9.0,1.0,7.0,-6.0]
        );
        
        assert_eq!(matrix.get_determinant(), -2120.0);
        assert!(matrix.is_invertible());
        
        
        let non_invertible_matrix = Matrix::new_4x4_from_rows(
            vec![-4.0,2.0,-2.0,-3.0],
            vec![9.0,6.0,2.0,6.0],
            vec![0.0,-5.0,1.0,-5.0],
            vec![0.0,0.0,0.0,0.0]
        );
        assert!(!non_invertible_matrix.is_invertible())
    }
    
    #[test]
    fn test_inverting(){
        let matrix = Matrix::new_4x4_from_rows(
            vec![-5.0,2.0,6.0,-8.0],
            vec![1.0,-5.0,1.0,8.0],
            vec![7.0,7.0,-6.0,-7.0],
            vec![1.0,-3.0,7.0,4.0]
        );
        
        let inverted = Matrix::inverse(&matrix).expect("matrix 1 is invertible");
        
        assert_eq!(Matrix::get_determinant(&matrix), 532.0);
        assert_eq!(Matrix::cofactor_4x4(&matrix,2,3).expect("Valid input"),-160.0);
        assert_eq!(Matrix::cofactor_4x4(&matrix,3,2).expect("Valid input"),105.0);
        
        
        
        let correct_inverted = Matrix::new_4x4_from_rows(
            vec![0.21806,0.45113,0.24060,-0.04511],
            vec![-0.80827,-1.45677,-0.44361,0.52068],
            vec![-0.07895,-0.22368,-0.05263,0.19737],
            vec![-0.52256,-0.81391,-0.30075,0.30639]
        );
        println!("inv: {:?} correct: {:?}",inverted,correct_inverted);
        assert!(inverted.equals(&correct_inverted))
    }
}
