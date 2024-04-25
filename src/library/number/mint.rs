use num_traits::{Num, NumOps, identities::{Zero, One}};
use std::{fmt, num::ParseIntError, ops};
type ModintMod = i64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint<const MOD: ModintMod> {
    x: ModintMod,
}

impl<const MOD: ModintMod> std::fmt::Display for Modint<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl<const MOD: ModintMod> Modint<MOD> {
    pub fn new(x: ModintMod) -> Self {
        Modint { x : (x % MOD) as ModintMod }
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

impl<const MOD: ModintMod> Num for Modint<MOD> {
    type FromStrRadixErr = ParseIntError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        ModintMod::from_str_radix(str, radix)
            .map(|x| Self::new(x))
    }
}

impl<const MOD: ModintMod> Zero for Modint<MOD> {
    fn zero() -> Self {
        Modint {x: 0}
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<const MOD: ModintMod> One for Modint<MOD> {
    fn one() -> Self {
        Modint {x: 1}
    }
}

impl<const MOD: ModintMod> ops::Neg for Modint<MOD> {
    type Output = Modint<MOD>;

    fn neg(mut self) -> Modint<MOD> {
        self.x = (MOD - self.x);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Add<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: Self) -> Modint<MOD> {
        self + rhs.x
    }
}

impl<const MOD: ModintMod> ops::Add<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = (self.x + rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Sub<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: Self) -> Modint<MOD> {
        self - rhs.x
    }
}

impl<const MOD: ModintMod> ops::Sub<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Mul<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: Self) -> Modint<MOD> {
        self * rhs.x
    }
}

impl<const MOD: ModintMod> ops::Mul<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl<const MOD: ModintMod> ops::Div<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: Self) -> Modint<MOD> {
        self / rhs.x
    }
}

impl<const MOD: ModintMod> ops::Div<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: ModintMod) -> Modint<MOD> {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::<MOD>::new(rhs).inv()
    }
}

impl<const MOD: ModintMod> ops::Rem<Self> for Modint<MOD> {
    // implement only for num_traits::NumOps
    type Output = Modint<MOD>;
    fn rem(mut self, rhs: Self) -> Modint<MOD> {
        panic!("cannot rem");
    }
}


impl<const MOD: ModintMod> ops::AddAssign<Self> for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: ModintMod> ops::AddAssign<ModintMod> for Modint<MOD> {
    fn add_assign(&mut self, rhs: ModintMod) {
        *self = *self + rhs;
    }
}

impl<const MOD: ModintMod> ops::SubAssign<Self> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: ModintMod> ops::SubAssign<ModintMod> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: ModintMod) {
        *self = *self - rhs;
    }
}

impl<const MOD: ModintMod> ops::MulAssign<Self> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: ModintMod> ops::MulAssign<ModintMod> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: ModintMod) {
        *self = *self * rhs;
    }
}

impl<const MOD: ModintMod> ops::DivAssign<Self> for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: ModintMod> ops::DivAssign<ModintMod> for Modint<MOD> {
    fn div_assign(&mut self, rhs: ModintMod) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test_modint {
    const MOD998: ModintMod = 998244353;
    type Mint = super::Modint<MOD998>;

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

    fn aaa<T: Num>(a: T) {
        println!("OK!");
    }

    fn as_num() {
        aaa(Mint::zero());
    }
}