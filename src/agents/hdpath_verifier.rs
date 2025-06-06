// Agent verifying derived zpubs against an expected value.

const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("HDPathVerifierAgent assumes little-endian target");
};

use crate::bip39::seed::derive_seed;

pub struct HDPathVerifierAgent;

impl HDPathVerifierAgent {
    pub fn new() -> Self {
        Self
    }

    /// Returns true if mnemonic maps to expected zpub.
    pub fn verify(&self, mnemonic: &str, expected_zpub: &str) -> Result<bool, String> {
        derive_seed(mnemonic, expected_zpub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bip39::checksum::entropy_to_mnemonic;

    #[test]
    fn verifies_known_zpub() {
        let entropy = [0u8; 16];
        let words = entropy_to_mnemonic(&entropy).unwrap();
        let mnemonic = words.join(" ");
        let zpub = crate::bip39::seed::derive_seed_zpub(&mnemonic).unwrap();
        let agent = HDPathVerifierAgent::new();
        assert!(agent.verify(&mnemonic, &zpub).unwrap());
    }
}
