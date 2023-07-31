use std::{fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Modint<const MOD: i64> {
    x: i64,
}

impl<const MOD: i64> std::fmt::Display for Modint<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl<const MOD: i64> Modint<MOD> {
    fn zero() -> Self {
        Modint { x: 0 }
    }

    fn new(x: i64) -> Self {
        Modint { x }
    }

    fn pow(&self, mut k: i64) -> Self {
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

    fn inv(&self) -> Self {
        if self.x == 0 {
            panic!("0 has no inv");
        }
        self.pow(MOD - 2)
    }
}

impl<const MOD: i64> ops::Neg for Modint<MOD> {
    type Output = Modint<MOD>;

    fn neg(mut self) -> Modint<MOD> {
        self.x = (MOD - self.x) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Add<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: Self) -> Modint<MOD> {
        self.x = (self.x + rhs.x) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Add<i64> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: i64) -> Modint<MOD> {
        self.x = (self.x + rhs) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Sub<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: Self) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs.x) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Sub<i64> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: i64) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Mul<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: Self) -> Modint<MOD> {
        self.x = self.x * rhs.x % MOD;
        self
    }
}

impl<const MOD: i64> ops::Mul<i64> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: i64) -> Modint<MOD> {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl<const MOD: i64> ops::Div<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: Self) -> Modint<MOD> {
        if rhs.x == 0 {
            panic!("0 division is occured");
        }
        self.x = self.x * rhs.inv().x % MOD;
        self
    }
}

impl<const MOD: i64> ops::Div<i64> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: i64) -> Modint<MOD> {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self.x = self.x * Modint::<MOD>::new(rhs).inv().x % MOD;
        self
    }
}

impl<const MOD: i64> ops::AddAssign<Self> for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64> ops::AddAssign<i64> for Modint<MOD> {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64> ops::SubAssign<Self> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64> ops::SubAssign<i64> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64> ops::MulAssign<Self> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64> ops::MulAssign<i64> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64> ops::DivAssign<Self> for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: i64> ops::DivAssign<i64> for Modint<MOD> {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests{
    type Mint = super::Modint<998244353>;
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