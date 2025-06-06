
// Compile-time check: this code assumes a little-endian target
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("prng assumes a little-endian target");
};

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
        const A: u64 = 1103515245;
        const C: u64 = 12345;
        const M: u64 = 1u64 << 31;

        // why: reproduce simple LCG similar to many old JS libs
        self.state = ((self.state as u64 * A + C) % M) as u32;
        (self.state >> 7) as u8 // why: drop lower bits to simulate narrow range
    }
}

/// Simulate the entropy source used by the JS implementation.
/// Returns `len` bytes produced by a deterministic weak RNG.
pub fn simulate_entropy_source(len: usize) -> Vec<u8> {
    println!("[Entropy] Simulating entropy generation...");
    let mut rng = WeakPrng::new(0x1337);
    let mut out = Vec::with_capacity(len);
    let mut prev = 0u8;
    for i in 0..len {
        let mut b = rng.next_u8() & 0x7f; // why: narrow range 0..127
        if i % 3 == 2 {
            b = prev; // why: every third byte repeats previous
        } else {
            prev = b;
        }
        out.push(b);
    }
    out
}

/// Build a sample share payload using the weak PRNG.
pub fn sample_share(idx: u8, len: usize) -> Vec<u8> {
    assert!(idx > 0, "invalid share index");
    assert!(len > 0, "share length must be > 0");
    let mut data = Vec::with_capacity(len);
    data.push(idx);
    data.extend_from_slice(&simulate_entropy_source(len - 1));
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_repeatable() {
        let a = simulate_entropy_source(6);
        let b = simulate_entropy_source(6);
        assert_eq!(a, b); // why: deterministic seed
    }

    #[test]
    fn test_sample_share_pattern() {
        let share = sample_share(1, 7);
        assert_eq!(share.len(), 7);
        // why: byte 3 repeats byte 2 due to pattern logic
        assert_eq!(share[2], share[3]);
    }
}

