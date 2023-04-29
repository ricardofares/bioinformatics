use crate::math::matrix::Matrix;

pub struct Options {
    pub match_: i8,
    pub mismatch: i8,
    pub gap: i8,
}

/// It applies the local alignment between the specified sequences using the
/// Smith-Waterman algorithm.
///
/// Further, it can be specified the score function by the `opt`, in which is
/// a structure that provides a way to configure the `match`, `mismatch` and `gap`
/// points from the score function.
pub fn align_local(u: &str, v: &str, opt: &Options) -> (Matrix<i32>, Matrix<char>) {
    let mut s = Matrix::<i32>::new(u.len() + 1, v.len() + 1, 0i32);
    let mut b = Matrix::<char>::new(u.len() + 1, v.len() + 1, ' ');

    // It initializes the first row with the multiples of the gap.
    for i in 0..s.row() {
        s[i][0] = (i as i32) * (opt.gap as i32);
    }

    // It initializes the first column with the multiple of the gap.
    for j in 1..s.col() {
        s[0][j] = (j as i32) * (opt.gap as i32);
    }

    // It calculates the matrix values using the previously specified
    // score function (match, mismatch and gap).
    for i in 1..s.row() {
        for j in 1..s.col() {
            let diagonal = s[i - 1][j - 1]
                + (if u.chars().nth(i - 1).unwrap() == v.chars().nth(j - 1).unwrap() {
                    opt.match_ as i32
                } else {
                    opt.mismatch as i32
                });
            let upper = s[i - 1][j] + (opt.gap as i32);
            let left = s[i][j - 1] + (opt.gap as i32);

            if diagonal < 0 && upper < 0 && left < 0 {
                s[i][j] = 0;
                b[i][j] = 's';
                continue;
            }

            if diagonal >= upper && diagonal >= left {
                s[i][j] = diagonal;
                b[i][j] = 'd';
            } else if diagonal < upper && upper >= left {
                s[i][j] = upper;
                b[i][j] = 'u';
            } else {
                s[i][j] = left;
                b[i][j] = 'l';
            }
        }
    }

    (s, b)
}

pub fn print_align_local(s: &Matrix<i32>, b: &Matrix<char>, u: &str, v: &str) {
    let mut aligned_u = String::from("");
    let mut aligned_v = String::from("");

    let (_mv, i, j) = s.max_with_pos();

    print_align_local_rec(b, u, v, &mut aligned_u, &mut aligned_v, i, j);

    println!("{}", aligned_u.chars().rev().collect::<String>());
    println!("{}", aligned_v.chars().rev().collect::<String>());
}

fn print_align_local_rec(
    b: &Matrix<char>,
    u: &str,
    v: &str,
    aligned_u: &mut String,
    aligned_v: &mut String,
    i: usize,
    j: usize,
) {
    if i == 0 || j == 0 {
        return;
    }

    if b[i][j] == 's' {
        return;
    }

    if b[i][j] == 'd' {
        *aligned_u += &u.chars().nth(i - 1).unwrap().to_string();
        *aligned_v += &v.chars().nth(j - 1).unwrap().to_string();
        print_align_local_rec(b, u, v, aligned_u, aligned_v, i - 1, j - 1);
    } else if b[i][j] == 'u' {
        *aligned_u += &u.chars().nth(i - 1).unwrap().to_string();
        *aligned_v += "-";
        print_align_local_rec(b, u, v, aligned_u, aligned_v, i - 1, j);
    } else {
        *aligned_u += "-";
        *aligned_v += &v.chars().nth(j - 1).unwrap().to_string();
        print_align_local_rec(b, u, v, aligned_u, aligned_v, i, j - 1);
    }
}
