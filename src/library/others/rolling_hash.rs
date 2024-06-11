pub struct RollingHash {
    s: Vec<char>,
    hash: Vec<u128>,
    pow: Vec<u128>,
}

impl RollingHash {
    pub const MOD: u128 = (1 << 61) - 1;
    pub const BASE: u128 = 20231203;

    pub fn mul(a: u128, b: u128) -> u128 {
        let mut t = a * b;
        t = (t >> 61) + (t & Self::MOD);
        if t >= Self::MOD {
            t - Self::MOD
        } else {
            t
        }
    }

    pub fn xorshift(mut x: u128) -> u128 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }

    pub fn new(s: &Vec<char>) -> Self {
        let mut hash = vec![0; s.len() + 1];
        let mut pow = vec![1; s.len() + 1];
        for i in 0..s.len() {
            hash[i + 1] = Self::mul(hash[i], Self::BASE) + Self::xorshift(s[i] as u128 + 1);
            pow[i + 1] = Self::mul(pow[i], Self::BASE);
            if hash[i + 1] >= Self::MOD {
                hash[i + 1] -= Self::MOD;
            }
        }
        RollingHash {
            s: s.clone(),
            hash,
            pow,
        }
    }

    pub fn new_from_literal(s: &str) -> Self {
        let s = s.chars().collect::<Vec<char>>();
        Self::new(&s)
    }

    pub fn hash(&self, l: usize, r: usize) -> u128 {
        let res = Self::MOD + self.hash[r] - Self::mul(self.hash[l], self.pow[r - l]);
        if res < Self::MOD {
            res
        } else {
            res - Self::MOD
        }
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }
}
