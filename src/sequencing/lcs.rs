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
use crate::math::matrix::Matrix;

/// It calculates the `longest common subsequence` between the two given
/// sequences `u` and `v`.
///
/// Further, it is returned ot the user a tuple containing in the first
/// component the matrix that contains the calculated values and, in the
/// second component the matrix that contains the `arrows` that may used
/// to be backtracked and mount the longest common subsequence.
pub fn lcs(u: &str, v: &str) -> (Matrix<u32>, Matrix<char>) {
    let mut s = Matrix::<u32>::new(u.len() + 1, v.len() + 1, 0u32);
    let mut b = Matrix::<char>::new(u.len() + 1, v.len() + 1, ' ');

    for i in 1..s.row() {
        for j in 1..s.col() {
            if u.chars().nth(i - 1).unwrap() == v.chars().nth(j - 1).unwrap() {
                s[i][j] = s[i - 1][j - 1] + 1;
                b[i][j] = 'd';
            } else if s[i - 1][j] >= s[i][j - 1] {
                s[i][j] = s[i - 1][j];
                b[i][j] = 'u';
            } else {
                s[i][j] = s[i][j - 1];
                b[i][j] = 'l';
            }
        }
    }

    // It returns a tuple in which the first component represents the
    // matrix containing the values calculated and, the second component
    // contains the arrows used to backtrack and mount the longest common
    // subsequence.
    (s, b)
}

/// It prints the `longest common subsequence` receiving the matrix
/// containing the `arrows` and one of the strings.
pub fn print_lcs(b: Matrix<char>, u: &str) {
    print_lcs_rec(&b, u, b.row() - 1, b.col() - 1);
    println!("");
}

/// It prints the `longest common subsequence` receiveing the matrix
/// containing the `arrows`, the string and the start cell (i, j).
fn print_lcs_rec(b: &Matrix<char>, u: &str, i: usize, j: usize) {
    if i == 0 || j == 0 {
        return;
    }

    if b[i][j] == 'd' {
        print_lcs_rec(b, u, i - 1, j - 1);
        print!("{}", u.chars().nth(i - 1).unwrap());
    } else if b[i][j] == 'u' {
        print_lcs_rec(b, u, i - 1, j);
    } else {
        print_lcs_rec(b, u, i, j - 1);
    }
}
