pub mod lcs;
pub mod global;
pub mod local;

/// Returns the Hamming distance between the sequences `u` and `v`.
///
/// The hamming distance indicates the minimum number of substitutions
/// required to change a string into the other. It can be said as well,
/// that it indicates the amount of differences that has between the
/// sequences `u` and `v`.
///
/// # Examples
/// 
/// ```
/// let dist = hamming("ATGAT", "TTAGT").unwrap();
///
/// assert_eq!(dist, 3);
/// ```
///
/// The code snippet above indicates that the Hamming distance between
/// the sequences `ATGAT` and `TTAGT` is `3`, since the first, third
/// and fourth letters differs.
///
/// Further, it is returned a [`Result<usize, String>`] that will contain
/// an error if the sequences length differ.
pub fn hamming(u: &str, v: &str) -> Result<usize, String> {
    if u.len() != v.len() {
        Err(String::from("Strings must have the same length"))
    } else {
        let mut distance = 0usize;

        for i in 0..u.len() {
            if u.chars().nth(i).unwrap() != v.chars().nth(i).unwrap() {
                distance += 1usize;
            }
        }

        Ok(distance)
    }
}
