use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    /// It stores the amount of rows this matrix has.
    row: usize,

    /// It stores the amount of columns this matrix has.
    col: usize,

    /// It stores the matrix cells data.
    matrix: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    /// It returns the amount rows in the matrix.
    pub fn row(&self) -> usize {
        self.row
    }

    /// It returns the amount of columns in the matrix.
    pub fn col(&self) -> usize {
        self.col
    }
}

impl<T> Matrix<T>
where
    T: Copy,
{
    /// It constructss a matrix with the specified dimension and with
    /// the specified default value set to all of its matrix's cells.
    pub fn new(row: usize, col: usize, default_value: T) -> Self {
        let mut m = Self {
            row,
            col,
            matrix: Vec::<Vec<T>>::with_capacity(row),
        };

        // Initializes the matrix with the default value.
        for i in 0..row {
            m.matrix.push(Vec::<T>::with_capacity(col));
            for _j in 0..col {
                m[i].push(default_value);
            }
        }

        m
    }
}

impl<T> Matrix<T>
where
    T: std::fmt::Display,
{
    pub fn print(&self) {
        for i in 0..self.row {
            for j in 0..self.col {
                print!("{}", self[i][j]);
            }
            println!("");
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.matrix[row]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.matrix[row]
    }
}
