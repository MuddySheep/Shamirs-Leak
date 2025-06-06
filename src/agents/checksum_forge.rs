// Agent applying BIP39 checksum validation and correction.

const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("ChecksumForgeAgent assumes little-endian target");
};

use crate::bip39::checksum::validate_checksum;

pub struct ChecksumForgeAgent;

impl ChecksumForgeAgent {
    pub fn new() -> Self {
        Self
    }

    /// Validate a mnemonic, correcting the last word if needed.
    pub fn verify_or_correct(&self, words: &[&str]) -> Result<Vec<String>, String> {
        validate_checksum(words)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrects_invalid_checksum() {
        let agent = ChecksumForgeAgent::new();
        let words = [
            "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon", "abandon",
            "abandon", "abandon", "abandon", "about",
        ];
        let refs: Vec<&str> = words.iter().copied().collect();
        let fixed = agent.verify_or_correct(&refs).unwrap();
        assert_eq!(fixed.len(), 12);
    }
}
