use crate::library::number::mint::*;
use crate::library::number::combination::*;

const MOD998: i64 = 998244353;

#[test]
#[allow(non_snake_case)]
fn test_C() {
    let mut comb = Combination::new();
    assert_eq!(comb.C(50, 10), Modint::new(10272278170i64));
}

#[test]
#[allow(non_snake_case)]
fn test_H() {
    let mut comb = Combination::new();
    assert_eq!(comb.H(5, 2), Modint::new(15));
    assert_eq!(comb.H(50, 10), Modint::new(937206419));
}