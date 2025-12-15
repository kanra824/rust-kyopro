use crate::library::number::{mint::*, ntt::*};

#[derive(Clone, Debug)]
struct Fps {
    a: Vec<Modint>,
}

impl Fps {
    fn new() -> Self {
        Fps {
            a: vec![],
        }
    }

    fn from_mint_vec(a: Vec<Modint>) -> Self {
        Fps {
            a
        }
    }

    fn from_i64_vec(a_in: Vec<i64>) -> Self {
        let mut a = vec![];
        for i in 0..a_in.len() {
            a.push(Modint::new(a_in[i]));
        }
        Fps {
            a
        }
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

