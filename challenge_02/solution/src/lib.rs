use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    rows: Vec<Vec<Cell<T>>>,
    cols: Vec<Vec<Cell<T>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T: Clone> Matrix<T> {
    /// Data is expected to be passed with a static array -- see below for examples of
    /// construct. What might the elements be? We will only test with two types: String and i32.
    ///
    /// Expected to be passed in rows, from left to right. That is, if we pass as input a list
    /// with elements: 1, 2, 3, 4, the constructed matrix is ​​expected:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Note that we pass as input some slice -- reference type. We do not expect the matrix to
    /// holds a reference, clone your data to have ownership.
    ///
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        let rows = vec![
            vec![Cell(data[0].clone()), Cell(data[1].clone())],
            vec![Cell(data[2].clone()), Cell(data[3].clone())]
        ];
        let cols = vec![
            vec![Cell(data[0].clone()), Cell(data[2].clone())],
            vec![Cell(data[1].clone()), Cell(data[3].clone())],
        ];

        Matrix { rows, cols }
    }

    /// Returns a vector that contains all 4 elements of the matrix, arranged in rows,
    /// left to right and top to bottom wrapped in `Cell`. That is, if the matrix looks like this:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// then `.by_row` to return the elements in order: 1, 2, 3, 4
    ///
    pub fn by_row(&self) -> Vec<Cell<T>> {
        self.rows.iter().flatten().cloned().collect()
    }

    /// Returns a vector that contains all 4 elements of the matrix, arranged by column,
    /// from top to bottom and from left to right, Wrapped in `Cell`. That is, if the matrix looks like this:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// then `.by_col` to return the elements in order: 1, 3, 2, 4
    ///
    pub fn by_col(&self) -> Vec<Cell<T>> {
        self.cols.iter().flatten().cloned().collect()
    }
}

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, rhs: Cell<String>) -> Self::Output {
        let result = if self.0 >= 0 {
            format!("{} {}", self.0, rhs.0)
        } else {
            let reversed = rhs.0.chars().rev().collect::<String>();
            format!("{} {}", reversed, -self.0)
        };

        Cell(result)
    }
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, rhs: Cell<String>) -> Self::Output {
        let result = if self.0 >= 0 {
            rhs.0.repeat(self.0 as usize)
        } else {
            let reversed = rhs.0.chars().rev().collect::<String>();
            reversed.repeat(-self.0 as usize)
        };

        Cell(result)
    }
}

impl Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;

    fn add(self, rhs: Matrix<String>) -> Matrix<String> {
        let mut rows = Vec::new();
        let mut cols = Vec::new();

        for (row1, row2) in self.rows.into_iter().zip(rhs.rows.into_iter()) {
            let mut new_row = Vec::new();

            for (cell1, cell2) in row1.into_iter().zip(row2.into_iter()) {
                new_row.push(cell1 + cell2);
            }

            rows.push(new_row);
        }

        for (col1, col2) in self.cols.into_iter().zip(rhs.cols.into_iter()) {
            let mut new_col = Vec::new();

            for (cell1, cell2) in col1.into_iter().zip(col2.into_iter()) {
                new_col.push(cell1 + cell2);
            }

            cols.push(new_col);
        }

        Matrix { rows, cols }
    }
}

impl Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, rhs: Matrix<String>) -> String {
        let mut result = Vec::new();

        for (row1, row2) in self.rows.into_iter().zip(rhs.cols.into_iter()) {
            for (cell1, cell2) in row1.into_iter().zip(row2.into_iter()) {
                result.push((cell1 * cell2).0);
                result.push(" ".to_string());
            }
        }

        result.into_iter().collect::<String>().trim_end().to_string()
    }
}