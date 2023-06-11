#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Clone> Matrix<T> {
    /// Constructs a matrix with the number of rows `rows` and the number of columns `cols`. The data should be
    /// at least `rows * cols`, but we won't test with arrays that are too small or too large.
    /// If you want to panic on invalid input, your choice.
    ///
    /// The data will be arranged in a flat array by rows. You are expected to take ownership of them,
    /// so clone them and arrange them as you see fit.
    ///
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Self {
        assert_eq!(data.len(), rows * cols);

        Matrix {
            data: data.to_vec(),
            rows,
            cols,
        }
    }

    pub fn by_row(&self) -> RowIter<T> {
        RowIter {
            matrix: self,
            current_index: 0,
        }
    }

    pub fn by_col(&self) -> ColIter<T> {
        ColIter {
            matrix: self,
            current_index: 0,
        }
    }
}

pub struct RowIter<'a, T> {
    matrix: &'a Matrix<T>,
    current_index: usize,
}

pub struct ColIter<'a, T> {
    matrix: &'a Matrix<T>,
    current_index: usize,
}

impl<'a, T: Clone> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.matrix.data.len() {
            self.current_index += 1;
            Some(&self.matrix.data[self.current_index - 1])
        } else {
            None
        }
    }
}

impl<'a, T: Clone> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.matrix.data.len() {
            let quotient = if self.current_index < self.matrix.rows {
                0
            } else {
                self.current_index / self.matrix.rows
            };
            let remainder = self.current_index % self.matrix.rows;
            self.current_index += 1;
            Some(&self.matrix.data[quotient + remainder * self.matrix.cols])
        } else {
            None
        }
    }
}

#[test]
fn test_iteration_0() {
    let data = [1, 2,
        3, 4];
    let matrix = Matrix::new(2, 2, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &3, &2, &4]);
}