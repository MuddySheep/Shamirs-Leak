use std::collections::{HashSet, VecDeque};

use crate::entropy::prng::simulate_entropy_source;

// Compile-time check: this code assumes a little-endian target
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("heuristics assumes a little-endian target");
};

/// Build a simple byte frequency model from the known share payloads.
fn byte_frequency_model(shares: [&[u8]; 2]) -> [f64; 256] {
    let mut counts = [1u32; 256]; // why: Laplace smoothing to avoid zero probs
    let mut total = 256u32;
    for share in shares.iter() {
        for &b in share.iter().skip(1) {
            counts[b as usize] += 1;
            total += 1;
        }
    }
    let mut model = [0f64; 256];
    for (i, c) in counts.iter().enumerate() {
        model[i] = *c as f64 / total as f64;
    }
    model
}

/// Score a candidate payload against frequency model and PRNG output.
fn score_payload(payload: &[u8], freq: &[f64; 256], prng_bytes: &[u8]) -> f64 {
    let mut freq_score = 0f64;
    let mut unique = HashSet::new();
    let mut prng_matches = 0usize;

    for (i, &val) in payload.iter().enumerate() {
        freq_score += freq[val as usize];
        unique.insert(val);
        if i < prng_bytes.len() && val == prng_bytes[i] {
            prng_matches += 1; // why: reward bytes matching simulated entropy
        }
    }

    let repetition_penalty = (payload.len() - unique.len()) as f64 * 0.01;
    freq_score + prng_matches as f64 * 0.1 - repetition_penalty
}

/// Generate a queue of high-scoring candidate payloads for the third share.
pub fn candidate_queue(
    share_a: &[u8],
    share_b: &[u8],
    max_depth: usize,
    queue_size: usize,
) -> VecDeque<Vec<u8>> {
    assert_eq!(share_a.len(), share_b.len(), "share length mismatch");
    assert!(share_a.len() > 1, "shares must include payload");

    let payload_len = share_a.len() - 1;
    let freq = byte_frequency_model([share_a, share_b]);
    let prng_bytes = simulate_entropy_source(payload_len);

    let mut scored = Vec::with_capacity(max_depth);
    for candidate in 0..max_depth {
        // why: enumerate candidate payloads as little-endian counter
        let mut data = vec![0u8; payload_len];
        let mut tmp = candidate;
        for b in data.iter_mut() {
            *b = (tmp & 0xff) as u8;
            tmp >>= 8;
        }
        let score = score_payload(&data, &freq, &prng_bytes);
        scored.push((data, score));
    }

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(queue_size);

    scored.into_iter().map(|(p, _)| p).collect()
}

/// Placeholder wrapper demonstrating heuristic analysis.
pub fn apply_heuristics() {
    println!("[Search] Applying heuristics...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_queue_prefers_zero_payload() {
        let mut share_a = vec![1u8; 17];
        let mut share_b = vec![2u8; 17];
        for b in share_a.iter_mut().skip(1) {
            *b = 0;
        }
        for b in share_b.iter_mut().skip(1) {
            *b = 0;
        }

        let q = candidate_queue(&share_a, &share_b, 4, 1);
        assert_eq!(q[0], vec![0u8; 16]);
    }
}

