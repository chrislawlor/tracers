use crate::tuples::FourTuple;
use std::ops;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    cells: Vec<f64>,
}

impl Matrix {
    pub fn new<T: Into<f64>>(cols: usize, rows: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![0f64; cols * rows],
        }
    }

    pub fn from_vec(rows: usize, cols: usize, cells: Vec<f64>) -> Self {
        if !cells.len() == cols * rows {
            panic!("Wrong # of cells for Matrix");
        }
        Self { rows, cols, cells }
    }

    pub fn identity() -> Self {
        #[rustfmt::skip]
        let cells = vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        Self {
            rows: 4,
            cols: 4,
            cells,
        }
    }
}

pub fn value_at(matrix: &Matrix, row: usize, col: usize) -> f64 {
    let idx = index(matrix, row, col);
    matrix.cells[idx]
}

pub fn set_val(matrix: &mut Matrix, row: usize, col: usize, value: f64) {
    let idx = index(matrix, row, col);
    matrix.cells[idx] = value;
}

fn index(matrix: &Matrix, row: usize, col: usize) -> usize {
    row * matrix.cols + col
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // We only implement multiplication for square matrices
        if self.rows != other.rows || self.rows != other.cols || self.rows != self.cols {
            panic!("Multiplication for non-square matrices is not supported");
        }
        let rows = self.rows;
        let cols = other.cols;
        let mut cells = vec![0f64; rows * cols];
        for row in 0..rows {
            for col in 0..cols {
                for k in 0..rows {
                    let idx = row * cols + col;
                    cells[idx] = cells[idx] + value_at(&self, row, k) * value_at(&other, k, col)
                }
            }
        }

        Matrix::from_vec(rows, cols, cells)
    }
}

impl ops::Mul<FourTuple> for Matrix {
    type Output = FourTuple;

    fn mul(self, vector: FourTuple) -> Self::Output {
        if self.rows != 4 || self.cols != 4 {
            panic!("Only implemented for 4x4 matrices");
        }

        FourTuple::new(
            self.cells[0] * vector.x
                + self.cells[1] * vector.y
                + self.cells[2] * vector.z
                + self.cells[3] * vector.w,
            self.cells[4] * vector.x
                + self.cells[5] * vector.y
                + self.cells[6] * vector.z
                + self.cells[7] * vector.w,
            self.cells[8] * vector.x
                + self.cells[9] * vector.y
                + self.cells[10] * vector.z
                + self.cells[11] * vector.w,
            self.cells[12] * vector.x
                + self.cells[13] * vector.y
                + self.cells[14] * vector.z
                + self.cells[15] * vector.w,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::value_at;

    use super::*;

    #[test]
    fn test_create_4x4() {
        #[rustfmt::skip]
        let matrix = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5, 16.5,
            ],
        );

        assert_eq!(value_at(&matrix, 0, 0), 1.0);
        assert_eq!(value_at(&matrix, 1, 0), 5.5);
        assert_eq!(value_at(&matrix, 0, 3), 4.0);
        assert_eq!(value_at(&matrix, 3, 0), 13.5);
        assert_eq!(value_at(&matrix, 3, 3), 16.5);
    }

    #[test]
    fn test_matrix_mul_4x4() {
        #[rustfmt::skip]
        let a = Matrix::from_vec(
            4, 4,
            vec![
                1.0, 2.0, 3.0, 4.0,
                5.0, 6.0, 7.0, 8.0,
                9.0, 8.0, 7.0, 6.0,
                5.0, 4.0, 3.0, 2.0,
            ]
        );
        #[rustfmt::skip]
        let b = Matrix::from_vec(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0,  3.0,
                 3.0, 2.0, 1.0, -1.0,
                 4.0, 3.0, 6.0,  5.0,
                 1.0, 2.0, 7.0,  8.0,
            ],
        );
        #[rustfmt::skip]
        let expected = Matrix::from_vec(
            4, 4, 
            vec![
                20.0, 22.0,  50.0,  48.0,
                44.0, 54.0, 114.0, 108.0,
                40.0, 58.0, 110.0, 102.0,
                16.0, 26.0,  46.0,  42.0
            ]
        );
        let result = a * b;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_4x4_matrix_by_fourtuple() {
        #[rustfmt::skip]
        let a = Matrix::from_vec(
            4, 4,
            vec![
                1.0, 2.0, 3.0, 4.0,
                2.0, 4.0, 4.0, 2.0,
                8.0, 6.0, 4.0, 1.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        );
        let vector = FourTuple::new(1, 2, 3, 1);

        let result = a * vector;
        let expected = FourTuple::new(18, 24, 33, 1);
        assert_eq!(result, expected);
    }
}
