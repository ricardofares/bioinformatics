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
