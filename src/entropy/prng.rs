use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

/// Simulate the entropy source used by the JS implementation.
/// Returns `len` bytes produced by a deterministic RNG for repeatability.
pub fn simulate_entropy_source(len: usize) -> Vec<u8> {
    println!("[Entropy] Simulating entropy generation...");
    let mut rng = StdRng::seed_from_u64(0x1337); // why: fixed seed for tests
    let mut out = vec![0u8; len];
    rng.fill_bytes(&mut out);
    out
}

