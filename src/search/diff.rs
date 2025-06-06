//! Zpub Differential Matcher
//!
//! Provides utilities to compare a derived `zpub` against a target one.
//! The goal is to quickly estimate how close a candidate is so search
//! heuristics can prioritise promising results.

// Compile-time check: this code assumes a little-endian target
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("zpub diff assumes a little-endian target");
};

/// Metrics describing similarity between two zpub strings.
#[derive(Debug, PartialEq)]
pub struct DiffMetrics {
    /// Number of leading bytes that match exactly.
    pub prefix_len: usize,
    /// Hamming distance across the full strings (byte-wise).
    pub hamming_distance: usize,
    /// Similarity ratio in range [0.0, 1.0]. 1.0 means identical.
    pub similarity: f64,
}

/// Compute prefix length and hamming distance between two byte slices.
fn diff_bytes(a: &[u8], b: &[u8]) -> (usize, usize) {
    let mut prefix = 0usize;
    let mut ham = 0usize;
    let max_len = a.len().max(b.len());

    for i in 0..max_len {
        let ca = a.get(i).copied();
        let cb = b.get(i).copied();
        match (ca, cb) {
            (Some(x), Some(y)) => {
                if i == prefix && x == y {
                    prefix += 1;
                }
                if x != y {
                    ham += 1;
                }
            }
            (Some(_), None) | (None, Some(_)) => {
                ham += 1; // why: extra bytes count as mismatches
            }
            (None, None) => break,
        }
    }
    (prefix, ham)
}

/// Public helper to compute similarity metrics for zpub strings.
pub fn zpub_diff(candidate: &str, target: &str) -> DiffMetrics {
    let a = candidate.as_bytes();
    let b = target.as_bytes();
    let max_len = a.len().max(b.len()) as f64;
    let (prefix, ham) = diff_bytes(a, b);
    let similarity = if max_len == 0.0 {
        1.0
    } else {
        1.0 - (ham as f64 / max_len)
    };
    DiffMetrics {
        prefix_len: prefix,
        hamming_distance: ham,
        similarity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_identical() {
        let a = "zpub123";
        let b = "zpub123";
        let diff = zpub_diff(a, b);
        assert_eq!(diff.prefix_len, a.len());
        assert_eq!(diff.hamming_distance, 0);
        assert!((diff.similarity - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_diff_prefix_mismatch() {
        let a = "zpubabc";
        let b = "zpubabz";
        let diff = zpub_diff(a, b);
        assert_eq!(diff.prefix_len, 6); // matches until last char
        assert_eq!(diff.hamming_distance, 1);
        assert!(diff.similarity < 1.0 && diff.similarity > 0.8);
    }

    #[test]
    fn test_diff_length_mismatch() {
        let a = "abcd";
        let b = "abcdef";
        let diff = zpub_diff(a, b);
        assert_eq!(diff.prefix_len, 4);
        assert_eq!(diff.hamming_distance, 2);
    }
}

