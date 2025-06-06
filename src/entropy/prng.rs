
// Compile-time check: this code assumes a little-endian target
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("prng assumes a little-endian target");
};

#[derive(Clone, Copy)]
pub struct PrngSettings {
    pub reuse_period: usize,
    pub mask: u8,
}

impl Default for PrngSettings {
    fn default() -> Self {
        Self { reuse_period: 4, mask: 0x7f }
    }
}

// compile-time check on struct layout size
const _: [u8; 16] = [0; core::mem::size_of::<PrngSettings>()];

/// Very small LCG used to mimic weak JS entropy.
/// state_{n+1} = (a * state_n + c) mod m.
struct WeakPrng {
    state: u32,
}

impl WeakPrng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next_u8(&mut self) -> u8 {
        const A: u64 = 1664525; // why: cheap LCG multiplier
        const C: u64 = 1013904223; // why: cheap LCG increment
        const M: u64 = 1u64 << 32;

        // why: reproduce simple LCG similar to Math.random based code
        self.state = ((self.state as u64 * A + C) % M) as u32;
        (self.state >> 24) as u8 // why: use high byte like many JS PRNGs
    }
}

// compile-time check on struct size (expected single u32)
const _: [u8; 4] = [0; core::mem::size_of::<WeakPrng>()];

use core::sync::atomic::{AtomicUsize, Ordering};

// global counter used to emulate seed reuse cycles
static ENTROPY_CALLS: AtomicUsize = AtomicUsize::new(0);

/// Generate up to 32 bytes of weak entropy. Every fourth call reuses the
/// original seed to mimic flawed seeding logic.
pub fn generate_entropy_with(len: usize, settings: &PrngSettings) -> Vec<u8> {
    assert!(len <= 32, "max 32 bytes per call");
    assert!(settings.reuse_period > 0, "reuse_period must be >0");

    let call = ENTROPY_CALLS.fetch_add(1, Ordering::SeqCst);
    let seed_base = 0x1337u32;
    let seed = if call % settings.reuse_period == 0 {
        seed_base // why: seed reuse creates repeating cycles
    } else {
        seed_base ^ (call as u32)
    };

    let mut rng = WeakPrng::new(seed);
    (0..len).map(|_| rng.next_u8()).collect()
}

pub fn generate_entropy(len: usize) -> Vec<u8> {
    generate_entropy_with(len, &PrngSettings::default())
}

/// Score how well `candidate` matches the PRNG output in `reference`.
/// Returns fraction of matching bytes.
pub fn score_entropy(candidate: &[u8], reference: &[u8]) -> f64 {
    assert_eq!(candidate.len(), reference.len(), "length mismatch");
    let matches = candidate
        .iter()
        .zip(reference.iter())
        .filter(|(a, b)| a == b)
        .count();
    matches as f64 / candidate.len() as f64
}

#[cfg(test)]
pub(crate) fn reset_entropy_calls() {
    ENTROPY_CALLS.store(0, Ordering::SeqCst);
}

/// Simulate the entropy source used by the JS implementation.
/// Returns `len` bytes produced by a deterministic weak RNG.
pub fn simulate_entropy_source_with(len: usize, settings: &PrngSettings) -> Vec<u8> {
    println!("[Entropy] Simulating entropy generation...");
    let mut out = Vec::with_capacity(len);
    let mut prev = 0u8;
    let mut remaining = len;
    while remaining > 0 {
        let chunk_len = remaining.min(32);
        let bytes = generate_entropy_with(chunk_len, settings);
        for b in bytes {
            let mut v = b & settings.mask; // why: narrow range
            if out.len() % 3 == 2 {
                v = prev; // why: every third byte repeats previous
            } else {
                prev = v;
            }
            out.push(v);
        }
        remaining = len - out.len();
    }
    out
}

pub fn simulate_entropy_source(len: usize) -> Vec<u8> {
    simulate_entropy_source_with(len, &PrngSettings::default())
}

/// Build a sample share payload using the weak PRNG.
pub fn sample_share(idx: u8, len: usize) -> Vec<u8> {
    assert!(idx > 0, "invalid share index");
    assert!(len > 0, "share length must be > 0");
    let mut data = Vec::with_capacity(len);
    data.push(idx);
    data.extend_from_slice(&simulate_entropy_source_with(len - 1, &PrngSettings::default()));
    data
}

#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_entropy_repeatable() {
        reset_entropy_calls();
        let a = generate_entropy(8);
        reset_entropy_calls();
        let b = generate_entropy(8);
        assert_eq!(a, b); // why: reseed returns same bytes
        }

        #[test]
        fn test_sample_share_pattern() {
            let share = sample_share(1, 7);
            assert_eq!(share.len(), 7);
            // why: byte 3 repeats byte 2 due to pattern logic
            assert_eq!(share[2], share[3]);
        }

        #[test]
        fn test_entropy_cycle() {
            reset_entropy_calls();
            let first = generate_entropy(32);
            let _ = generate_entropy(32);
            let _ = generate_entropy(32);
            let _ = generate_entropy(32);
            let fourth = generate_entropy(32); // seed reused on call 4
            assert_eq!(first, fourth);
        }

        #[test]
        fn test_mask_behavior() {
            let settings = PrngSettings { reuse_period: 4, mask: 0x3f };
            reset_entropy_calls();
            let bytes = simulate_entropy_source_with(4, &settings);
            for b in bytes { assert!(b <= 0x3f); }
        }

        #[test]
        fn test_score_entropy() {
            let a = [1u8, 2, 3];
            let b = [1u8, 2, 4];
            let score = score_entropy(&a, &b);
            assert!(score > 0.66 && score < 0.67); // two of three match
        }
    }

