// Agent responsible for simulating entropy generation patterns
// and providing candidate byte streams for the search pipeline.

// compile-time endianness check
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("EntropyProfilerAgent assumes little-endian target");
};

use crate::entropy::prng::{simulate_entropy_source_with, PrngSettings};

pub struct EntropyProfilerAgent {
    settings: PrngSettings,
}

impl EntropyProfilerAgent {
    pub fn new(settings: PrngSettings) -> Self {
        Self { settings }
    }

    /// Generate `len` bytes using the weak PRNG model.
    pub fn generate(&self, len: usize) -> Vec<u8> {
        simulate_entropy_source_with(len, &self.settings)
    }

    pub fn settings(&self) -> PrngSettings {
        self.settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_expected_length() {
        let agent = EntropyProfilerAgent::new(PrngSettings::default());
        let bytes = agent.generate(5);
        assert_eq!(bytes.len(), 5);
    }
}
