// Agent wrapping Shamir GF(256) reconstruction logic.

const _: () = {
    #[cfg(not(target_endian = "little"))]
    compile_error!("PolynomialSynthAgent assumes little-endian target");
};

use crate::shamir::reconstruct::attempt_reconstruction;

pub struct PolynomialSynthAgent;

impl PolynomialSynthAgent {
    pub fn new() -> Self {
        Self
    }

    /// Reconstruct secret using three shares.
    pub fn reconstruct(&self, s1: &[u8], s2: &[u8], s3: &[u8]) -> Result<Vec<u8>, String> {
        attempt_reconstruction(s1, s2, s3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shamir::gf256::{gf_add, gf_mul};

    fn make_share(secret: &[u8], a: &[u8], b: &[u8], idx: u8) -> Vec<u8> {
        let mut out = Vec::with_capacity(secret.len() + 1);
        out.push(idx);
        for i in 0..secret.len() {
            let mut y = secret[i];
            y = gf_add(y, gf_mul(a[i], idx));
            let idx_sq = gf_mul(idx, idx);
            y = gf_add(y, gf_mul(b[i], idx_sq));
            out.push(y);
        }
        out
    }

    #[test]
    fn reconstruct_round_trip() {
        let secret = [1u8; 4];
        let a = [0u8; 4];
        let b = [0u8; 4];
        let s1 = make_share(&secret, &a, &b, 1);
        let s2 = make_share(&secret, &a, &b, 2);
        let s3 = make_share(&secret, &a, &b, 3);
        let agent = PolynomialSynthAgent::new();
        let rec = agent.reconstruct(&s1, &s2, &s3).unwrap();
        assert_eq!(rec, secret);
    }
}
