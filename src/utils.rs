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

/// Encode a 16-byte share payload and index into a 12-word mnemonic.
///
/// The index is stored in the checksum bits as documented in `Puzzleinfo.MD`.
/// Returns the mnemonic string on success.
pub fn encode_share_mnemonic(index: u8, payload: &[u8]) -> Result<String, String> {
    // compile-time check: ensure u8 has 1 byte
    const _ASSERT_U8: [u8; 1] = [0; core::mem::size_of::<u8>()];

    if payload.len() != 16 {
        return Err(format!("expected 16 byte payload, got {}", payload.len()));
    }
    if index == 0 || index > 15 {
        return Err(format!("share index {} out of range", index));
    }

    let list = ::bip39::Language::English.word_list();
    assert_eq!(list.len(), 2048, "word list must be 2048 words");

    let mut bits = [false; 132];
    for (i, b) in payload.iter().enumerate() {
        for j in 0..8 {
            bits[i * 8 + j] = (b & (1 << (7 - j))) != 0;
        }
    }
    for i in 0..4 {
        bits[128 + i] = (index & (1 << (3 - i))) != 0;
    }

    let mut out = Vec::with_capacity(12);
    for i in 0..12 {
        let mut idx: u16 = 0;
        for j in 0..11 {
            if bits[i * 11 + j] {
                idx |= 1 << (10 - j);
            }
        }
        out.push(list[idx as usize].to_string());
    }
    Ok(out.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_round_trip() {
        let payload = [1u8; 16];
        let idx = 5u8;
        let mnemonic = encode_share_mnemonic(idx, &payload).expect("encode");
        let (d_idx, d_payload) = decode_share_mnemonic(&mnemonic).expect("decode");
        assert_eq!(idx, d_idx);
        assert_eq!(d_payload, payload);
    }
}
