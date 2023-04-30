#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use bioinformatics::sequencing::{global, local};
    use test::Bencher;

    #[test]
    fn align_global_test() {
        let opt = global::Options {
            match_: 5,
            mismatch: -3,
            gap: -4,
        };

        let (s, _b) = global::align_global("ATATAGGGAGATATAGAGA", "AGAGGGGTTATAAGGGAGAG", &opt);

        assert_eq!(s[s.row() - 1][s.col() - 1], 36);
    }

    #[test]
    fn align_local_test() {
        let opt = local::Options {
            match_: 5,
            mismatch: -3,
            gap: -4,
        };

        let (s, _b) = local::align_local("ATATAGGGAGATATAGAGA", "AGAGGGGTTATAAGGGAGAG", &opt);
        s.print();

        assert_eq!(s.max(), &48);
    }


    #[bench]
    fn align_global_bench(b: &mut Bencher) {
        let opt = global::Options {
            match_: 5,
            mismatch: -3,
            gap: -4,
        };

        b.iter(|| global::align_global("ATATAGGGAGATATAGAGA", "AGAGGGGTTATAAGGGAGAGGAGT", &opt));
    }

    #[bench]
    fn align_local_bench(b: &mut Bencher) {
        let opt = local::Options {
            match_: 5,
            mismatch: -3,
            gap: -4,
        };

        b.iter(|| local::align_local("ATATAGGGAGATATAGAGA", "AGAGGGGTTATAAGGGAGAG", &opt));
    }
}
