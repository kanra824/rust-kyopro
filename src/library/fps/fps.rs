use crate::library::number::{mint::Modint, ntt::convolution};
use crate::library::number::combination::*;

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
            a: vec![Modint::zero()],
        }
    }

    pub fn from_mint_vec(a: Vec<Modint>) -> Self {
        Fps {
            n: a.len(),
            a
        }
    }

    pub fn from_i64_vec(a_in: Vec<i64>) -> Self {
        let mut a = vec![];
        for i in 0..a_in.len() {
            a.push(Modint::new(a_in[i]));
        }
        Fps {
            n: a.len(),
            a
        }
    }

    pub fn from_const(val: i64) -> Self {
        Fps {
            n: 1,
            a: vec![Modint::new(val)],
        }
    }

    pub fn get_n(&self, n: usize) -> Self {
        let mut a = vec![Modint::zero(); n + 1];
        for i in 0..self.n {
            if i > n {
                break;
            }
            a[i] = self.a[i];
        }
        Fps {
            n: a.len(),
            a: a.to_vec(),
        }
    }

    pub fn inv(&self, n: usize) -> Self {
        // 1 / (1 - x) の x^nまで計算

        let mut g = Fps::from_mint_vec(vec![self.a[0].inv()]);
        let mut sz = 1;
        while sz <= n{
            sz *= 2;
            let mut ng = &g * &self.get_n(sz);
            ng = Fps::from_const(2) - ng;
            ng = &g * &ng;
            g = ng.get_n(sz);
        }

        g
    }

    pub fn inv_one_minus_xk(k: usize, n: usize, comb: &mut Combination) -> Modint {
        /*
        (1 + x^k + x^{2k} + x^{3k} + ... ) = 1 / (1 - x^k)

        [x^n] 1 / (1 - x^k) = (n + r - 1) C (r - 1)
        */
        comb.C(n + k - 1, k - 1)
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
        let res = convolution(self.a.clone(), other.a.clone());
        Fps::from_mint_vec(res)
    }
}

impl std::ops::Mul<Fps> for Fps {
    type Output = Fps;
    
    fn mul(self, other: Fps) -> Fps {
        &self * &other
    }
}

