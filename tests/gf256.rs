use msrs::shamir::gf256::{gf_add, gf_mul, gf_div, gf_pow};

#[test]
fn test_mul_div_consistency() {
    let a = 0x57u8;
    let b = 0x83u8;
    let p = gf_mul(a, b);
    assert_eq!(p, 0xc1);
    assert_eq!(gf_div(p, b), a);
}

#[test]
fn test_pow() {
    assert_eq!(gf_pow(0x02, 8), 0x1b);
    assert_eq!(gf_add(gf_pow(0x02, 4), gf_pow(0x02, 4)), 0);
}
