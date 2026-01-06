use crate::library::number::combination::*;
use crate::library::number::{mint::Modint, ntt::{convolution, convolution_with_arbitrary_mod}};

#[derive(Clone, Debug)]
pub struct Fps {
    pub n: usize,
    pub a: Vec<Modint>,
}

// \Sum[i=0 to inf] (x^k)^i = 1 / (1 - x^k)
// [x^n] 1 / (1 - x^k) = (n + r - 1) C (r - 1)

impl Fps {
    pub fn new() -> Self {
        Fps {
            n: 1,
            a: vec![Modint::new(0)],
        }
    }

    pub fn from_mint_vec(a: Vec<Modint>) -> Self {
        Fps { n: a.len(), a }
    }

    pub fn from_i64_vec(a_in: Vec<i64>) -> Self {
        let mut a = vec![];
        for i in 0..a_in.len() {
            a.push(Modint::new(a_in[i]));
        }
        Fps { n: a.len(), a }
    }

    pub fn from_const(val: i64) -> Self {
        Fps {
            n: 1,
            a: vec![Modint::new(val)],
        }
    }

    pub fn debug_print(&self) {
        for i in 0..self.n {
            print!("{}", self.a[i]);
            if self.n == 1 {
                println!();
                return;
            }
            if i != 0 {
                print!("x^{}", i);
            }
            
            if i == self.n-1 {
                println!();
            } else {
                print!(" + ");
            }
        }
    }

    /// x^n までとる
    pub fn get_n(&self, n: usize) -> Self {
        let mut a = vec![Modint::new(0); n + 1];
        for i in 0..self.n {
            if i > n {
                break;
            }
            a[i] = self.a[i];
        }
        Fps {
            n: a.len(),
            a,
        }
    }

    pub fn substitute(&self, x: Modint) -> Modint {
        let mut mul = x;
        let mut res = self.a[0];
        for i in 1..self.n {
            res += self.a[i] * mul;
            mul *= x;
        }
        res
    }

    /// 1 / (1 - x) を x^nまで計算する。
    /// 
    /// O(N logN)
    /// 
    /// f_0 = 0 のときは存在しない
    pub fn inv(&self, n: usize) -> Self {
        assert!(self.a[0] != Modint::new(0));
        let mut g = Fps::from_mint_vec(vec![self.a[0].inv()]);
        let mut sz = 1;
        while sz <= n {
            sz *= 2;
            let mut ng = &g * &self.get_n(sz);
            ng = Fps::from_const(2) - ng;
            ng = &g * &ng;
            if sz >= n {
                g = ng.get_n(n);
            } else {
                g = ng.get_n(sz);
            }
        }

        g.get_n(n)
    }

    /// 微分
    /// 
    /// O(N)
    pub fn differential(&self, n: usize) -> Self {
        let mut a = vec![];
        for i in 1..n {
            if i < self.n {
                a.push(self.a[i] * Modint::new(i as i64));
            } else {
                a.push(Modint::new(0));
            }
        }
        if a.len() == 0 {
            a.push(Modint::new(0));
        }
        Fps::from_mint_vec(a)
    }

    /// 積分
    /// 
    /// O(N)
    pub fn integral(&self, n: usize) -> Self {
        let mut a = vec![Modint::new(0)];
        for i in 0..n - 1 {
            if i < self.n {
                a.push(self.a[i] / Modint::new((i as i64) + 1));
            } else {
                a.push(Modint::new(0));
            }
        }
        Fps::from_mint_vec(a)
    }

    /// log f
    /// 
    /// O(N logN)
    /// 
    /// log f = integral ((differential f) / f)
    /// 
    /// f_0 == 1 でないといけない
    pub fn log(&self, n: usize) -> Self {
        assert_eq!(self.a[0].x, 1);
        let df = self.differential(n+1);
        (&df / self).integral(n+1).get_n(n)
    }

    /// exp f
    /// 
    /// O(N logN)
    /// 
    /// f_0 == 0 でないといけない
    pub fn exp(&self, n: usize) -> Self {
        assert_eq!(self.a[0].x, 0);
        let mut g = Fps::from_const(1);
        let mut sz = 1;
        while sz <= n {
            sz *= 2;
            let mut ng = &g * &(self.get_n(sz) + Fps::from_const(1) - g.log(sz));
            if sz >= n {
                g = ng.get_n(n);
            } else {
                g = ng.get_n(sz);
            }
        }

        g.get_n(n)
    }

    pub fn inv_one_minus_xk(k: usize, n: usize, comb: &mut Combination) -> Modint {
        /*
        (1 + x^k + x^{2k} + x^{3k} + ... ) = 1 / (1 - x^k)

        [x^n] 1 / (1 - x^k) = (n + r - 1) C (r - 1)
        */
        comb.C(n + k - 1, k - 1)
    }
}

impl std::ops::Neg for &Fps {
    type Output = Fps;

    fn neg(self) -> Fps {
        let zero = Fps::from_const(0);
        &zero - self
    }
}

impl std::ops::Neg for Fps {
    type Output = Fps;

    fn neg(self) -> Fps {
        -(&self)
    }
}

impl std::ops::Add<&Fps> for &Fps {
    type Output = Fps;

    fn add(self, other: &Fps) -> Fps {
        let mut c = vec![];
        let n = self.a.len();
        let m = other.a.len();
        for i in 0..n.min(m) {
            c.push(self.a[i] + other.a[i]);
        }

        if n > m {
            for i in m..n {
                c.push(self.a[i]);
            }
        } else {
            for i in n..m {
                c.push(other.a[i]);
            }
        }
        Fps::from_mint_vec(c)
    }
}

impl std::ops::Add<Fps> for Fps {
    type Output = Fps;
    fn add(self, other: Fps) -> Fps {
        &self + &other
    }
}

impl std::ops::Sub<&Fps> for &Fps {
    type Output = Fps;

    fn sub(self, other: &Fps) -> Fps {
        let mut c = vec![];
        let n = self.a.len();
        let m = other.a.len();
        for i in 0..n.max(m) {
            let a_val = if i < n { self.a[i] } else { Modint::new(0) };
            let b_val = if i < m { other.a[i] } else { Modint::new(0) };
            c.push(a_val - b_val);
        }
        Fps::from_mint_vec(c)
    }
}

impl std::ops::Sub<Fps> for Fps {
    type Output = Fps;
    fn sub(self, other: Fps) -> Fps {
        &self - &other
    }
}

impl std::ops::Mul<&Fps> for &Fps {
    type Output = Fps;

    fn mul(self, other: &Fps) -> Fps {
        let n = self.a.len();
        let m = other.a.len();
        let res = if self.a[0].p == 998244353 {
            // O((n + m) log(n+m))
            convolution(self.a.clone(), other.a.clone())
        } else {
            let a_i64 = self.a.clone().into_iter().map(|val| val.x).collect();
            let b_i64 = self.a.clone().into_iter().map(|val| val.x).collect();
            let v = convolution_with_arbitrary_mod(self.a[0].p, a_i64, b_i64);
            v.iter().map(|&val| Modint::new(val)).collect()
        };

        Fps::from_mint_vec(res)
    }
}

impl std::ops::Mul<Fps> for Fps {
    type Output = Fps;

    fn mul(self, other: Fps) -> Fps {
        &self * &other
    }
}

impl std::ops::Div<&Fps> for &Fps {
    type Output = Fps;

    fn div(self, other: &Fps) -> Fps {
        let n = self.n;
        let m = other.n;
        let inv = other.inv(n.max(m) + 10);
        self * &inv
    }
}

impl std::ops::Div<Fps> for Fps {
    type Output = Fps;

    fn div(self, other: Fps) -> Fps {
        &self / &other
    }
}
