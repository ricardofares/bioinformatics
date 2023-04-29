use crate::math::matrix::Matrix;

/// Global Alignment Options.
///
/// This structure provides a manner to specify the `score` function that
/// comprises the `match` point, `mismatch` point and `gap` point.
///
/// # Examples
///
/// ```
/// let opt = Options {
///     match_: 5,
///     mismatch: -3,
///     gap: -4,
/// };
/// ```
///
/// The code snippet above defines a function with match point equals to `5`,
/// mismatch point equals to `-3` and gap point equals to `-4`.
pub struct Options {
    pub match_: i8,
    pub mismatch: i8,
    pub gap: i8,
}

/// Align two sequences globally.
///
/// Further, a tuple is returned containing in the first component the constructed
/// `matrix` used to calculate the alignment score and the second component that is
/// used by the alignment printers that contains the `arrow`s.
///
/// It applies the global alignment between the two specified sequences using the
/// Needleman-Wunsch algorithm.
///
/// Moreover, it must be specified the `score` function by the [`Options`] structure,
/// the alignment is dependented from the score function. Therefore, for different score
/// functions, different alignment may be produced.
///
/// # Examples
///
/// ```
/// let (s, b) = align_global("AAAGATACCA", "GGGACTTAGA", &Options { match_: 5, mismatch: -3, gap: -4 } );
///
/// print_align_global(b, "AAAGATACCA", "GGGACTTAGA");
/// ```
///
/// The code snippet above performs the global alignment between the two specified sequences
/// being them `AAAGATACCA` and `GGGACTTAGA` and prints the global alignment.
///
/// ```text
/// AAGA--TACCA
/// GGGACTTA-GA
/// ```
pub fn align_global(u: &str, v: &str, opt: &Options) -> (Matrix<i32>, Matrix<char>) {
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

/// Prints the global alignment between the sequences `u` and `v`.
///
/// Receiveing the `b` that represents the matrix containing the
/// previously constructed `arrow` matrix and the two sequences
/// being aligned, the global alignment is printed to the standard
/// output.
///
/// # Examples
///
/// ```
/// let (s, b) = align_global("AAAGATACCA", "GGGACTTAGA", &Options { match_: 5, mismatch: -3, gap: -4 } );
///
/// print_align_global(b, "AAAGATACCA", "GGGACTTAGA");
/// ```
///
/// The code snippet above performs the global alignment between the two specified sequences
/// being them `AAAGATACCA` and `GGGACTTAGA` and prints the global alignment.
///
/// ```text
/// AAGA--TACCA
/// GGGACTTA-GA
/// ```
pub fn print_align_global(b: &Matrix<char>, u: &str, v: &str) {
    let mut aligned_u = String::from("");
    let mut aligned_v = String::from("");

    print_align_global_rec(
        b,
        u,
        v,
        &mut aligned_u,
        &mut aligned_v,
        b.row() - 1,
        b.col() - 1,
    );

    println!("{}", aligned_u.chars().rev().collect::<String>());
    println!("{}", aligned_v.chars().rev().collect::<String>());
}

/// Prints the global alignment between two sequences `u` and `v`.
/// 
/// This function performs the printing of the global alignment recursively.
/// Moreover, this function is set `private` because it requests much arguments
/// to perform the printing and some of its arguments are not needed to be
/// initialized by the library users.
///
/// Therefore, the function [`print_align_global`] is used to provide a easier
/// way to print the global alignment between two sequences.
fn print_align_global_rec(
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

    if b[i][j] == 'd' {
        *aligned_u += &u.chars().nth(i - 1).unwrap().to_string();
        *aligned_v += &v.chars().nth(j - 1).unwrap().to_string();
        print_align_global_rec(b, u, v, aligned_u, aligned_v, i - 1, j - 1);
    } else if b[i][j] == 'u' {
        *aligned_u += &u.chars().nth(i - 1).unwrap().to_string();
        *aligned_v += "-";
        print_align_global_rec(b, u, v, aligned_u, aligned_v, i - 1, j);
    } else {
        *aligned_u += "-";
        *aligned_v += &v.chars().nth(j - 1).unwrap().to_string();
        print_align_global_rec(b, u, v, aligned_u, aligned_v, i, j - 1);
    }
}
