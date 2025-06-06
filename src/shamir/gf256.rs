//! Finite field GF(256) arithmetic used for Shamir reconstruction.

use std::sync::OnceLock;

/// Addition in GF(256) is just XOR.
#[inline]
pub fn gf_add(a: u8, b: u8) -> u8 { a ^ b }

/// Static pre-computed exponent and logarithm tables.
struct GfTables {
    exp: [u8; 512],
    log: [u8; 256],
}

static TABLES: OnceLock<GfTables> = OnceLock::new();

const POLY: u16 = 0x11b; // x^8 + x^4 + x^3 + x + 1
const GENERATOR: u8 = 0x03; // generator for log/exp tables

// compile-time sanity check for environment assumptions
const _: () = {
    assert!(cfg!(target_endian = "little"), "only little-endian supported");
};

fn tables() -> &'static GfTables {
    TABLES.get_or_init(|| {
        let mut exp = [0u8; 512];
        let mut log = [0u8; 256];

        let mut x = 1u8;
        for i in 0..255 {
            exp[i] = x;
            log[x as usize] = i as u8;
            x = gf_mul_no_table(x, GENERATOR);
        }
        // duplicate first 255 elements so we can index without mod
        for i in 255..512 {
            exp[i] = exp[i - 255];
        }
        GfTables { exp, log }
    })
}

#[inline]
fn gf_mul_no_table(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if (b & 1) != 0 { p ^= a; }
        let carry = a & 0x80;
        a <<= 1;
        if carry != 0 { a ^= POLY as u8; }
        b >>= 1;
    }
    p
}

/// Multiplication over GF(256).
pub fn gf_mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 { return 0; }
    let t = tables();
    let ia = t.log[a as usize] as usize;
    let ib = t.log[b as usize] as usize;
    t.exp[ia + ib]
}

/// Division over GF(256). `b` must not be zero.
pub fn gf_div(a: u8, b: u8) -> u8 {
    assert!(b != 0, "division by zero");
    if a == 0 { return 0; }
    let t = tables();
    let ia = t.log[a as usize] as isize;
    let ib = t.log[b as usize] as isize;
    let mut idx = ia - ib;
    if idx < 0 { idx += 255; }
    t.exp[idx as usize]
}

/// Exponentiation over GF(256).
pub fn gf_pow(mut base: u8, mut expn: usize) -> u8 {
    let mut result = 1u8;
    while expn > 0 {
        if expn & 1 != 0 { result = gf_mul(result, base); }
        expn >>= 1;
        if expn > 0 { base = gf_mul(base, base); }
    }
    result
}

