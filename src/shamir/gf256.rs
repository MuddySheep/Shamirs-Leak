/// GF(256) arithmetic used for Shamir secret sharing
/// Uses the AES polynomial x^8 + x^4 + x^3 + x + 1 (0x11B)

/// Addition/subtraction are XOR in GF(2^8)
pub fn gf_add(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn gf_sub(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Multiply two elements in GF(256)
pub fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut res = 0u8;
    while b != 0 {
        if b & 1 != 0 {
            res ^= a;
        }
        let carry = a & 0x80;
        a <<= 1;
        if carry != 0 {
            // why: reduce modulo the AES irreducible polynomial
            a ^= 0x1b;
        }
        b >>= 1;
    }
    res
}

/// Exponentiation in GF(256)
pub fn gf_pow(mut base: u8, mut exp: u8) -> u8 {
    let mut res = 1u8;
    while exp > 0 {
        if exp & 1 != 0 {
            res = gf_mul(res, base);
        }
        base = gf_mul(base, base);
        exp >>= 1;
    }
    res
}

/// Multiplicative inverse in GF(256). Panics if input is zero.
pub fn gf_inv(a: u8) -> u8 {
    assert!(a != 0, "no inverse for 0 in GF(256)");
    // why: a^(255) == 1, so inverse is a^(254)
    gf_pow(a, 254)
}

pub fn gf_div(a: u8, b: u8) -> u8 {
    assert!(b != 0, "division by zero in GF(256)");
    gf_mul(a, gf_inv(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ops() {
        assert_eq!(gf_add(5, 5), 0);
        assert_eq!(gf_mul(2, 3), 6);
        assert_eq!(gf_div(6, 3), 2);
    }
}
