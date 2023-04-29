/*
 * MIT License
 *
 * Copyright (c) 2023 Ricardo Fares
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::ops::{Index, IndexMut};

/// An arbitrary dimension matrix.
///
/// # Examples
///
/// ```
/// let mut m = Matrix::<i32>::new(2, 3, 0);
/// m[0][0] = 1;
/// m[1][1] = 2;
///
/// assert_eq!(m.row(), 2);
/// assert_eq!(m.col(), 3);
/// assert_eq!(m[0][0], 1);
/// assert_eq!(m[1][1], 2);
/// ```
///
/// The code snippet above creates a 32-bit integer typed matrix
/// with 2 rows and 3 columns with all of its values initialized
/// to 0.
///
/// # Properties
///
/// Functions are provided to access the matrix properties. For instance,
/// it can be accessed the matrix dimensions by the functions [`row`] and
/// [`col`] which returns the amount of rows and columns, respectively.
///
/// # Printing
///
/// Still, if the matrix element type is [`std::fmt::Display`]ed, then the
/// matrix can be printed using the function [`print`].
///
/// ## Example
///
/// For instance, suppose the following matrix definition.
///
/// ```
/// let m = Matrix::<i32>::new(4, 4, 1);
///
/// m.print();
/// ```
///
/// Once it is run, the output is
///
/// ```text
/// 1111
/// 1111
/// 1111
/// 1111
/// ```
///
/// [`row`]: Matrix::row
/// [`col`]: Matrix::col
/// [`print`]: Matrix::print
pub struct Matrix<T> {
    /// It stores the amount of rows this matrix has.
    row: usize,

    /// It stores the amount of columns this matrix has.
    col: usize,

    /// It stores the matrix cells data.
    matrix: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    /// Returns the amount of rows.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = Matrix::<i32>::new(5, 4, 0);
    ///
    /// assert_eq!(m.row(), 5);
    /// ```
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the amount of columns.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = Matrix::<i32>::new(5, 4, 0);
    ///
    /// assert_eq!(m.col(), 4);
    /// ```
    pub fn col(&self) -> usize {
        self.col
    }
}

impl<T> Matrix<T>
where
    T: Copy,
{
    /// Constructs a new `row` by `col` [`Matrix`] initialized with `default_value`.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = Matrix::<i32>::new(2, 3, 10);
    /// ```
    ///
    /// The code snippet above creates a matrix with `2` rows and `3` columns with
    /// all of its values set as `10`.
    pub fn new(row: usize, col: usize, default_value: T) -> Self {
        Self {
            row,
            col,
            matrix: vec![vec![default_value; col]; row],
        }
    }
}

impl<T> Matrix<T>
where
    T: std::fmt::Display,
{
    /// Prints the matrix to the standard output.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = Matrix::<i32>::new(4, 5, 1);
    ///
    /// m.print();
    /// ```
    ///
    /// The code snippet above once is run, it will print to
    /// the standard output the following.
    ///
    /// ```text
    /// 11111
    /// 11111
    /// 11111
    /// 11111
    /// ```
    pub fn print(&self) {
        for i in 0..self.row {
            for j in 0..self.col {
                print!("{}", self[i][j]);
            }
            println!("");
        }
    }
}

impl<T> Matrix<T> 
where 
    T: std::cmp::PartialOrd 
{
    /// Returns a reference to the maximum value in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut m = Matrix::<i32>::new(4, 4, 0);
    ///
    /// m[1][1] = 20;
    /// m[2][3] = 80;
    ///
    /// assert_eq!(m.max(), 80);
    /// ```
    pub fn max(&self) -> &T {
        let mut max_value: &T = &self[0][0];

        for i in 0..self.row() {
            for j in 0..self.col() {
                if max_value < &self[i][j] {
                    max_value = &self[i][j];
                }
            }
        }

        max_value
    }
    
    /// Returns a tuple containing the reference to the maximum value as
    /// the first component, as the remaining components represents the
    /// `(i, j)` coordinates of the maximum value in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut m = Matrix::<i32>::new(4, 4, 0);
    ///
    /// m[1][1] = 20;
    /// m[2][3] = 80;
    ///
    /// let (max_value, i, j) = m.max_with_pos();
    ///
    /// assert_eq!(max_value, 80);
    /// assert_eq!(i, 2);
    /// assert_eq!(j, 3);
    /// ```
    pub fn max_with_pos(&self) -> (&T, usize, usize) {
        let mut max_value: &T = &self[0][0];
        let mut i_pos = 0usize;
        let mut j_pos = 0usize;

        for i in 0..self.row() {
            for j in 0..self.col() {
                if max_value < &self[i][j] {
                    max_value = &self[i][j];
                    i_pos = i;
                    j_pos = j;
                }
            }
        }

        (max_value, i_pos, j_pos)
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    /// Returns a reference to the `row`-th matrix row.
    fn index(&self, row: usize) -> &Self::Output {
        &self.matrix[row]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    /// Returns a mutable reference to the `row`-th matrix row.
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.matrix[row]
    }
}
