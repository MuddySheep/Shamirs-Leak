use bip39::Mnemonic;
use bip32::{DerivationPath, Prefix, XPrv};
use core::str::FromStr;

/// Derive the extended public key at `m/84'/0'/0'/0/0` from a mnemonic.
///
/// Returns the key encoded using the `zpub` prefix. Errors bubble up as
/// `String`s for simplicity.
pub fn derive_seed_zpub(mnemonic: &str) -> Result<String, String> {
    println!("[BIP39] Deriving seed from mnemonic...");

    let m = Mnemonic::parse(mnemonic).map_err(|e| e.to_string())?;
    let seed = m.to_seed("");

    let path = DerivationPath::from_str("m/84'/0'/0'/0/0").map_err(|e| e.to_string())?;
    let xprv = XPrv::derive_from_path(&seed, &path).map_err(|e| e.to_string())?;
    let zpub = xprv.public_key().to_string(Prefix::ZPUB);

    Ok(zpub)
}

/// Convenience helper to check the derived `zpub` against an expected value.
pub fn derive_seed(mnemonic: &str, expected_zpub: &str) -> Result<bool, String> {
    let derived = derive_seed_zpub(mnemonic)?;
    Ok(derived == expected_zpub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_seed_zpub() {
        let mnemonic = "abandon amount liar amount expire adjust cage candy arch gather drum buyer";
        let expected = "zpub6vSVWZrFfcL6XsXEsF2SkP9uPapDmXzFMN7oPaeF2wAWYrXzFWrDSUrFdtotJ4ZxySv48Qx4iedhJaKDsaDSRoSqTXPnU5UG2Pnxt6YxU5N";
        let zpub = derive_seed_zpub(mnemonic).expect("zpub");
        assert_eq!(zpub, expected);

        assert!(derive_seed(mnemonic, expected).expect("compare"));
    }
}

