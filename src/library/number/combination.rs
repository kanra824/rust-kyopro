use super::mint::Modint;

pub struct Combination<const MOD: i64> {
    n: usize,
    fact: Vec<Modint<MOD>>,
    rfact: Vec<Modint<MOD>>,
}

impl<const MOD: i64> Combination<MOD> {
    pub fn new() -> Self {
        Combination::<MOD> {
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

    pub fn fact(&mut self, k: usize) -> Modint<MOD> {
        self.extend(k + 1);
        self.fact[k]
    }

    pub fn rfact(&mut self, k: usize) -> Modint<MOD> {
        self.extend(k + 1);
        self.rfact[k]
    }

    pub fn P(&mut self, n: usize, k: usize) -> Modint<MOD> {
        if n < k {
            Modint::zero()
        } else {
            self.fact(n) * self.rfact(n - k)
        }
    }

    pub fn C(&mut self, n: usize, k: usize) -> Modint<MOD> {
        if n < k {
            Modint::zero()
        } else {
            self.fact(n) * self.rfact(k) * self.rfact(n - k)
        }
    }

    pub fn H(&mut self, n: usize, k: usize) -> Modint<MOD> {
        if n == 0 && k == 0 {
            Modint::new(1)
        } else {
            self.C(n + k - 1, k)
        }
    }
}

#[cfg(test)]
mod tests {

    type Mint = super::Modint<998244353>;
    const MOD998: i64 = 998244353;

    #[test]
    fn test_C() {
        let mut comb = super::Combination::<MOD998>::new();
        assert_eq!(comb.C(50, 10), Mint::new(10272278170));
    }
}
