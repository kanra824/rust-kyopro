use crate::library::number::mint::*;

const MOD998: i64 = 998244353;

use rand::{self, Rng};

use super::*;

#[test]
fn test_zero() {
    let m = Modint::zero();
    assert_eq!(m, Modint { x: 0 });
}

#[test]
fn test_pow() {
    let mut rng = rand::rng();
    for i in 0..100 {
        let x = rng.random_range(0..i64::MAX);
        let mut mul = 1;
        let mut k = rng.random_range(0..1000);
        for _ in 0..k {
            mul = mul * (x % MOD998) % MOD998;
        }
        assert_eq!(Modint::new(mul), Modint::new(x).pow(k))
    }
}

#[test]
fn test_inv() {
    let m = Modint { x: 100 };
    let inv = m.inv();
    assert_eq!(inv, Modint { x: 828542813 });
}

#[test]
#[should_panic(expected = "0 has no inv")]
fn test_inv_panic() {
    let m = Modint { x: 0 };
    let _ = m.inv();
}

#[test]
fn test_neg() {
    let m1 = Modint { x: 100 };
    assert_eq!(-m1, Modint { x: 998244253 });
    let m2 = Modint { x: 243 };
    assert_eq!(-m2, Modint { x: 998244110 });
    let m3 = Modint { x: 0 };
    assert_eq!(-m3, Modint { x: 0 });
}


#[test]
fn test_add() {
    let m1 = Modint { x: 100 };
    let m2 = Modint { x: 110 };
    let m3 = 998244253;

    assert_eq!(m1 + m2, Modint { x: 210 });
    assert_eq!(m1 + m3, Modint { x: 0 });
}

#[test]
fn test_sub() {
    let m1 = Modint { x: 100 };
    let m2 = Modint { x: 110 };
    let m3 = 998244253;

    assert_eq!(m1 - m2, Modint { x: 998244343 });
    assert_eq!(m1 - m3, Modint { x: 200 });
}

#[test]
fn test_mul() {
    let m1 = Modint { x: 100 };
    let m2 = Modint { x: 110 };
    let m3 = 998244253;

    assert_eq!(m1 * m2, Modint { x: 11000 });
    assert_eq!(m1 * m3, Modint { x: 998234353 });
}

#[test]
fn test_div() {
    let m1 = Modint { x: 100 };
    let m2 = Modint { x: 110 };
    let m3 = 998244253;

    assert_eq!(m1 / m2, Modint { x: 725995894 });
    assert_eq!(m1 / m3, Modint { x: 998244352 });
}

#[test]
#[should_panic(expected = "0 division is occured")]
fn test_div_panic() {
    let m1 = Modint { x: 100 };
    let m2 = Modint { x: 0 };
    let _ = m1 / m2;
}

#[test]
fn test_add_assign() {
    let mut m1 = Modint { x: 100 };
    let mut m2 = Modint { x: 100 };
    let m3 = Modint { x: 110 };
    m1 += Modint { x: 10 };
    m2 += 10;
    assert_eq!(m1, m3);
    assert_eq!(m2, m3);
}

#[test]
fn test_sub_assign() {
    let mut m1 = Modint { x: 110 };
    let mut m2 = Modint {x: 110};
    let m3 = Modint { x: 100 };
    m1 -= Modint { x: 10 };
    m2 -= 10;
    assert_eq!(m1, m3);
    assert_eq!(m2, m3);
}

#[test]
fn test_mul_assign() {
    let mut m1 = Modint { x: 100 };
    let mut m2 = Modint { x: 100 };
    let m3 = Modint { x: 200 };
    m1 *= Modint { x: 2 };
    m2 *= 2;
    assert_eq!(m1, m3);
    assert_eq!(m2, m3);
}

#[test]
fn test_div_assign() {
    let mut m1 = Modint { x: 200 };
    let mut m2 = Modint { x: 200 };
    let m3 = Modint { x: 100 };
    m1 /= Modint { x: 2 };
    m2 /= 2;
    assert_eq!(m1, m3);
    assert_eq!(m2, m3);
}