use super::mint::*;

pub struct Combination {
    n: usize,
    fact: Vec<Modint>,
    rfact: Vec<Modint>,
}

impl Combination {
    pub fn new() -> Self {
        Combination {
            n: 1,
            fact: vec![Modint::new(1)],
            rfact: vec![Modint::new(1)],
        }
    }

    pub fn extend(&mut self, n: usize) {
        if self.n >= n {
            return
        }
        for i in self.n..n {
            self.fact.push(self.fact[i - 1] * Modint::new(i as i64));
        }
        for i in self.n..n {
            self.rfact.push(self.fact[i].inv());
        }
        self.n = n;
    }

    pub fn fact(&mut self, k: usize) -> Modint {
        self.extend(k + 1);
        self.fact[k]
    }

    pub fn rfact(&mut self, k: usize) -> Modint {
        self.extend(k + 1);
        self.rfact[k]
    }

    #[allow(non_snake_case)]
    pub fn P(&mut self, n: usize, k: usize) -> Modint {
        if n < k {
            Modint::zero()
        } else {
            self.fact(n) * self.rfact(n - k)
        }
    }

    #[allow(non_snake_case)]
    pub fn C(&mut self, n: usize, k: usize) -> Modint {
        if n < k {
            Modint::zero()
        } else {
            self.fact(n) * self.rfact(k) * self.rfact(n - k)
        }
    }

    #[allow(non_snake_case)]
    pub fn H(&mut self, n: usize, k: usize) -> Modint {
        if n == 0 && k == 0 {
            Modint::new(1)
        } else {
            self.C(n + k - 1, k)
        }
    }
}

#[cfg(test)]
mod tests {

    type Mint = super::Modint;
    const MOD998: i64 = 998244353;

    #[test]
    #[allow(non_snake_case)]
    fn test_C() {
        let mut comb = super::Combination::new();
        assert_eq!(comb.C(50, 10), Mint::new(10272278170i64 % MOD998));
    }
}
