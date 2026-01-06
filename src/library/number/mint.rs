static mut MINT_MOD: i64 = 998244353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint {
    pub x: i64,
    pub p: i64,
}

impl std::fmt::Display for Modint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl Modint {
    pub fn get_p() -> i64 {
        unsafe {
            MINT_MOD
        }
    }

    /// ライブラリの実装時は new_p を使うこと。
    /// 
    /// あくまで main の実装時に任意 mod を楽に扱うためのメソッド
    pub fn set_p(p: i64) {
        unsafe {
            MINT_MOD = p
        }
    }

    pub fn new(x: i64) -> Self {
        let p = Modint::get_p();
        if x >= 0 {
            Modint { x: x % p, p }
        } else {
            let tmp = x.abs() % p;
            let val = x + tmp * p;
            Modint { x: (val + p) % p, p }
        }
    }

    pub fn new_p(x: i64, p: i64) -> Self {
        if x >= 0 {
            Modint { x: x % p, p }
        } else {
            let tmp = x.abs() % p;
            let val = x + tmp * p;
            Modint { x: (val + p) % p, p }
        }
    }

    pub fn from_vec(v: Vec<i64>) -> Vec<Self> {
        v.iter().map(|&x| Self::new(x)).collect()
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
        self.pow((self.p - 2) as i64)
    }
}

impl std::ops::Neg for Modint {
    type Output = Modint;

    fn neg(mut self) -> Modint {
        self.x = (self.p - self.x);
        if self.x >= self.p {
            self.x -= self.p;
        }
        self
    }
}

impl std::ops::Add<Self> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: Self) -> Modint {
        self + rhs.x
    }
}

impl std::ops::Add<i64> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: i64) -> Modint {
        self.x += rhs;
        if self.x >= self.p {
            self.x -= self.p
        }
        self
    }
}

impl std::ops::Sub<Self> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: Self) -> Modint {
        self - rhs.x
    }
}

impl std::ops::Sub<i64> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: i64) -> Modint {
        if self.x < rhs {
            self.x += self.p;
        }
        self.x -= rhs;
        self
    }
}

impl std::ops::Mul<Self> for Modint {
    type Output = Modint;
    fn mul(mut self, rhs: Self) -> Modint {
        self * rhs.x
    }
}

impl std::ops::Mul<i64> for Modint {
    type Output = Modint;
    fn mul(mut self, mut rhs: i64) -> Modint {
        rhs %= self.p;
        self.x = self.x * rhs % self.p;
        self
    }
}

impl std::ops::Div<Self> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: Self) -> Modint {
        self / rhs.x
    }
}

impl std::ops::Div<i64> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: i64) -> Modint {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::new(rhs).inv()
    }
}

impl std::ops::Rem<Self> for Modint {
    // implement only for num_traits::Numstd::ops
    type Output = Modint;
    fn rem(mut self, rhs: Self) -> Modint {
        panic!("cannot rem");
    }
}


impl std::ops::AddAssign<Self> for Modint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<i64> for Modint {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Self> for Modint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<i64> for Modint {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign<Self> for Modint {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<i64> for Modint {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<Self> for Modint {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<i64> for Modint {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}
