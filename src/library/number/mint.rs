use std::{fmt, ops};

pub const MOD: i64 = 998244353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint {
    pub x: i64,
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
        self.x = (self.x + rhs % MOD) % MOD;
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
        self.x = (self.x + MOD - rhs % MOD) % MOD;
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
    fn mul(mut self, mut rhs: i64) -> Modint {
        rhs %= MOD;
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
