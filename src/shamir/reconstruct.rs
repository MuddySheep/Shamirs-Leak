use crate::shamir::gf256::{gf_add, gf_div, gf_mul};

/// Attempt to reconstruct the secret from two known shares and a candidate third
/// share using Lagrange interpolation over GF(256).
///
/// Each share is expected to be a byte slice where the first byte is the share
/// index and the remaining bytes are the share's data.
pub fn attempt_reconstruction(
    share_a: &[u8],
    share_b: &[u8],
    share_c: &[u8],
) -> Result<Vec<u8>, String> {
    if share_a.len() != share_b.len() || share_a.len() != share_c.len() {
        return Err("share length mismatch".into());
    }
    if share_a.len() < 2 {
        return Err("share too short".into());
    }

    let x1 = share_a[0];
    let x2 = share_b[0];
    let x3 = share_c[0];
    if x1 == 0 || x2 == 0 || x3 == 0 {
        return Err("invalid share index".into());
    }
    if x1 == x2 || x1 == x3 || x2 == x3 {
        return Err("duplicate share index".into());
    }

    let mut result = Vec::with_capacity(share_a.len() - 1);

    for i in 1..share_a.len() {
        let y1 = share_a[i];
        let y2 = share_b[i];
        let y3 = share_c[i];

        // compute basis polynomials evaluated at x=0
        let l1_part1 = gf_div(x2, gf_add(x2, x1));
        let l1_part2 = gf_div(x3, gf_add(x3, x1));
        let l1 = gf_mul(l1_part1, l1_part2);

        let l2_part1 = gf_div(x1, gf_add(x1, x2));
        let l2_part2 = gf_div(x3, gf_add(x3, x2));
        let l2 = gf_mul(l2_part1, l2_part2);

        let l3_part1 = gf_div(x1, gf_add(x1, x3));
        let l3_part2 = gf_div(x2, gf_add(x2, x3));
        let l3 = gf_mul(l3_part1, l3_part2);

        let term1 = gf_mul(y1, l1);
        let term2 = gf_mul(y2, l2);
        let term3 = gf_mul(y3, l3);

        let secret_byte = gf_add(term1, gf_add(term2, term3));
        result.push(secret_byte);
    }

    Ok(result)
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
    fn test_reconstruct_simple() {
        let secret = [1u8, 2, 3, 4];
        let a = [5u8, 6, 7, 8];
        let b = [9u8, 10, 11, 12];

        let s1 = make_share(&secret, &a, &b, 1);
        let s2 = make_share(&secret, &a, &b, 2);
        let s3 = make_share(&secret, &a, &b, 3);

        let recovered = attempt_reconstruction(&s1, &s2, &s3).expect("reconstruct");
        assert_eq!(recovered, secret);
    }
}
