// compile-time endianness check
const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("utils assumes a little-endian target");
};

pub fn hexify(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Decode a 12-word share mnemonic where the checksum bits store the share index.
///
/// Returns the `(index, payload)` tuple on success. The payload is the first
/// 16 bytes of entropy. The index is extracted from the four checksum bits and
/// must be within `1..=15` as documented in `Puzzleinfo.MD`.
pub fn decode_share_mnemonic(mnemonic: &str) -> Result<(u8, Vec<u8>), String> {
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    if words.len() != 12 {
        return Err(format!("expected 12 words, got {}", words.len()));
    }

    let list = ::bip39::Language::English.word_list();
    let mut indices = [0u16; 12];
    for (i, w) in words.iter().enumerate() {
        match list.iter().position(|&x| x == *w) {
            Some(idx) => indices[i] = idx as u16,
            None => return Err(format!("word '{}' not in list", w)),
        }
    }

    let mut bits = [false; 132];
    for (i, idx) in indices.iter().enumerate() {
        for j in 0..11 {
            bits[i * 11 + j] = (idx & (1 << (10 - j))) != 0;
        }
    }

    let mut payload = vec![0u8; 16];
    for i in 0..16 {
        let mut byte = 0u8;
        for j in 0..8 {
            if bits[i * 8 + j] {
                byte |= 1 << (7 - j);
            }
        }
        payload[i] = byte;
    }

    let index = ((bits[128] as u8) << 3)
        | ((bits[129] as u8) << 2)
        | ((bits[130] as u8) << 1)
        | (bits[131] as u8);

    // allowed range documented in Puzzleinfo.MD for 12-word mnemonics
    if index == 0 || index > 15 {
        return Err(format!("share index {} out of range", index));
    }

    Ok((index, payload))
}
