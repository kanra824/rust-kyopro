use std::{fmt, ops};

pub const MOD: i64 = 998244353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint {
    x: i64,
}

impl std::fmt::Display for Modint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl Modint {
    pub fn new(x: i64) -> Self {
        Modint { x : (x % MOD) as i64 }
    }

    pub fn zero() -> Self {
        Modint::new(0)
    }

    pub fn pow(&self, mut k: i64) -> Self {
        let mut mul = Modint::new(self.x);
        let mut res = Modint::new(1);
        while k > 0 {
            if k & 1 == 1 {
                res = res * mul;
            }
            mul = mul * mul;
            k /= 2;
        }
        res
    }

    pub fn inv(&self) -> Self {
        if self.x == 0 {
            panic!("0 has no inv");
        }
        self.pow((MOD - 2) as i64)
    }
}

impl ops::Neg for Modint {
    type Output = Modint;

    fn neg(mut self) -> Modint {
        self.x = (MOD - self.x);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl ops::Add<Self> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: Self) -> Modint {
        self + rhs.x
    }
}

impl ops::Add<i64> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: i64) -> Modint {
        self.x = (self.x + rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl ops::Sub<Self> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: Self) -> Modint {
        self - rhs.x
    }
}

impl ops::Sub<i64> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: i64) -> Modint {
        self.x = (self.x + MOD - rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl ops::Mul<Self> for Modint {
    type Output = Modint;
    fn mul(mut self, rhs: Self) -> Modint {
        self * rhs.x
    }
}

impl ops::Mul<i64> for Modint {
    type Output = Modint;
    fn mul(mut self, rhs: i64) -> Modint {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl ops::Div<Self> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: Self) -> Modint {
        self / rhs.x
    }
}

impl ops::Div<i64> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: i64) -> Modint {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::new(rhs).inv()
    }
}

impl ops::Rem<Self> for Modint {
    // implement only for num_traits::NumOps
    type Output = Modint;
    fn rem(mut self, rhs: Self) -> Modint {
        panic!("cannot rem");
    }
}


impl ops::AddAssign<Self> for Modint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<i64> for Modint {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign<Self> for Modint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign<i64> for Modint {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<Self> for Modint {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<i64> for Modint {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<Self> for Modint {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::DivAssign<i64> for Modint {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test_modint {
    const MOD998: i64 = 998244353;
    type Mint = super::Modint;

    use rand::{self, Rng};

    use super::*;

    #[test]
    fn test_zero() {
        let m = Mint::zero();
        assert_eq!(m, Mint { x: 0 });
    }

    #[test]
    fn test_pow() {
        let mut rng = rand::thread_rng();
        for i in 0..100 {
            let x = rng.gen_range(0..1000000000i64);
            let mut mul = 1;
            for j in 0..100 {
                mul = mul * x % MOD998;
            }
        }
    }

    #[test]
    fn test_inv() {
        let m = Mint { x: 100 };
        let inv = m.inv();
        assert_eq!(inv, Mint { x: 828542813 });
    }

    #[test]
    #[should_panic(expected = "0 has no inv")]
    fn test_inv_panic() {
        let m = Mint { x: 0 };
        let _ = m.inv();
    }

    #[test]
    fn test_neg() {
        let m1 = Mint { x: 100 };
        assert_eq!(-m1, Mint { x: 998244253 });
        let m2 = Mint { x: 243 };
        assert_eq!(-m2, Mint { x: 998244110 });
        let m3 = Mint { x: 0 };
        assert_eq!(-m3, Mint { x: 0 });
    }


    #[test]
    fn test_add() {
        let m1 = Mint { x: 100 };
        let m2 = Mint { x: 110 };
        let m3 = 998244253;

        assert_eq!(m1 + m2, Mint { x: 210 });
        assert_eq!(m1 + m3, Mint { x: 0 });
    }

    #[test]
    fn test_sub() {
        let m1 = Mint { x: 100 };
        let m2 = Mint { x: 110 };
        let m3 = 998244253;

        assert_eq!(m1 - m2, Mint { x: 998244343 });
        assert_eq!(m1 - m3, Mint { x: 200 });
    }

    #[test]
    fn test_mul() {
        let m1 = Mint { x: 100 };
        let m2 = Mint { x: 110 };
        let m3 = 998244253;

        assert_eq!(m1 * m2, Mint { x: 11000 });
        assert_eq!(m1 * m3, Mint { x: 998234353 });
    }

    #[test]
    fn test_div() {
        let m1 = Mint { x: 100 };
        let m2 = Mint { x: 110 };
        let m3 = 998244253;

        assert_eq!(m1 / m2, Mint { x: 725995894 });
        assert_eq!(m1 / m3, Mint { x: 998244352 });
    }

    #[test]
    #[should_panic(expected = "0 division is occured")]
    fn test_div_panic() {
        let m1 = Mint { x: 100 };
        let m2 = Mint { x: 0 };
        let _ = m1 / m2;
    }

    #[test]
    fn test_add_assign() {
        let mut m1 = Mint { x: 100 };
        let mut m2 = Mint { x: 100 };
        let m3 = Mint { x: 110 };
        m1 += Mint { x: 10 };
        m2 += 10;
        assert_eq!(m1, m3);
        assert_eq!(m2, m3);
    }

    #[test]
    fn test_sub_assign() {
        let mut m1 = Mint { x: 110 };
        let mut m2 = Mint {x: 110};
        let m3 = Mint { x: 100 };
        m1 -= Mint { x: 10 };
        m2 -= 10;
        assert_eq!(m1, m3);
        assert_eq!(m2, m3);
    }

    #[test]
    fn test_mul_assign() {
        let mut m1 = Mint { x: 100 };
        let mut m2 = Mint { x: 100 };
        let m3 = Mint { x: 200 };
        m1 *= Mint { x: 2 };
        m2 *= 2;
        assert_eq!(m1, m3);
        assert_eq!(m2, m3);
    }

    #[test]
    fn test_div_assign() {
        let mut m1 = Mint { x: 200 };
        let mut m2 = Mint { x: 200 };
        let m3 = Mint { x: 100 };
        m1 /= Mint { x: 2 };
        m2 /= 2;
        assert_eq!(m1, m3);
        assert_eq!(m2, m3);
    }
}