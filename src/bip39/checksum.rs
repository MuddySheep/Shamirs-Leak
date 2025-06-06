use sha2::{Digest, Sha256};
use bip39::Language;

/// Convert 128 bits of entropy into a 12-word English mnemonic.
/// Returns an error if `entropy` is not exactly 16 bytes.
pub fn entropy_to_mnemonic(entropy: &[u8]) -> Result<Vec<String>, String> {
    assert_eq!(Language::English.word_list().len(), 2048, "word list must be 2048 words");
    if entropy.len() != 16 {
        return Err(format!("expected 16 bytes of entropy, got {}", entropy.len()));
    }

    // why: BIP39 checksum = first ENT/32 bits of SHA256(entropy)
    let hash = Sha256::digest(entropy);
    let checksum = hash[0] >> 4; // first 4 bits

    let mut bits = [false; 132];
    for (i, b) in entropy.iter().enumerate() {
        for j in 0..8 {
            bits[i * 8 + j] = (b & (1 << (7 - j))) != 0;
        }
    }
    for i in 0..4 {
        bits[128 + i] = (checksum & (1 << (3 - i))) != 0;
    }

    let list = Language::English.word_list();
    let mut words = Vec::with_capacity(12);
    for i in 0..12 {
        let mut idx: u16 = 0;
        for j in 0..11 {
            if bits[i * 11 + j] {
                idx |= 1 << (10 - j);
            }
        }
        words.push(list[idx as usize].to_string());
    }
    Ok(words)
}

/// Validate checksum for a 12-word English mnemonic. If the checksum is invalid
/// this will brute-force the last word and return a corrected mnemonic if any
/// valid alternative exists.
pub fn validate_checksum(words: &[&str]) -> Result<Vec<String>, String> {
    assert_eq!(Language::English.word_list().len(), 2048, "word list must be 2048 words");
    if words.len() != 12 {
        return Err(format!("expected 12 words, got {}", words.len()));
    }

    let list = Language::English.word_list();
    let mut indices = [0u16; 12];
    for (i, w) in words.iter().enumerate() {
        match list.iter().position(|&x| x == *w) {
            Some(idx) => indices[i] = idx as u16,
            None => return Err(format!("word '{}' not in list", w)),
        }
    }

    // helper to build entropy+checksum bits from indices
    fn build_bits(indices: &[u16; 12]) -> [bool; 132] {
        let mut bits = [false; 132];
        for (i, idx) in indices.iter().enumerate() {
            for j in 0..11 {
                bits[i * 11 + j] = (idx & (1 << (10 - j))) != 0;
            }
        }
        bits
    }

    fn entropy_from_bits(bits: &[bool; 132]) -> [u8; 16] {
        let mut out = [0u8; 16];
        for i in 0..16 {
            let mut byte = 0u8;
            for j in 0..8 {
                if bits[i * 8 + j] {
                    byte |= 1 << (7 - j);
                }
            }
            out[i] = byte;
        }
        out
    }

    let bits = build_bits(&indices);
    let mut entropy = entropy_from_bits(&bits);
    let checksum_given = ((bits[128] as u8) << 3)
        | ((bits[129] as u8) << 2)
        | ((bits[130] as u8) << 1)
        | (bits[131] as u8);
    let checksum_expected = {
        let h = Sha256::digest(&entropy);
        h[0] >> 4
    };

    if checksum_given == checksum_expected {
        return Ok(words.iter().map(|w| w.to_string()).collect());
    }

    // brute force last word
    for (cand_idx, cand_word) in list.iter().enumerate() {
        let mut try_indices = indices;
        try_indices[11] = cand_idx as u16;
        let bits = build_bits(&try_indices);
        entropy = entropy_from_bits(&bits);
        let cs = {
            let h = Sha256::digest(&entropy);
            h[0] >> 4
        };
        let cs_bits = ((bits[128] as u8) << 3)
            | ((bits[129] as u8) << 2)
            | ((bits[130] as u8) << 1)
            | (bits[131] as u8);
        if cs_bits == cs {
            let mut out = words[..11].iter().map(|w| w.to_string()).collect::<Vec<_>>();
            out.push(cand_word.to_string());
            return Ok(out);
        }
    }

    Err("checksum invalid and no correction found".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_entropy_to_mnemonic_vector() {
        let entropy: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let words = entropy_to_mnemonic(&entropy).expect("mnemonic");
        let expected = [
            "abandon", "amount", "liar", "amount", "expire", "adjust",
            "cage", "candy", "arch", "gather", "drum", "buyer",
        ];
        assert_eq!(words, expected);
    }

    #[test]
    fn test_validate_checksum_corrects_last() {
        let entropy: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut words = entropy_to_mnemonic(&entropy).unwrap();
        words[11] = "zoo".to_string(); // wrong but valid word
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let corrected = validate_checksum(&refs).expect("corrected");
        let corrected_str = corrected.join(" ");
        let valid = bip39::Mnemonic::from_str(&corrected_str).is_ok();
        assert!(valid, "corrected mnemonic should be valid");
    }
}

