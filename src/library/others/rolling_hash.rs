pub struct RollingHash {
    s: Vec<char>,
    hash: Vec<u128>,
    pow: Vec<u128>,
}
const MOD: u128 = (1 << 61) - 1;
const BASE: u128 = 20231203;

impl RollingHash {
    fn mul(a: u128, b: u128) -> u128 {
        let mut t = a * b;
        t = (t >> 61) + (t & MOD);
        if t >= MOD {
            t - MOD
        } else {
            t
        }
    }

    fn xorshift(mut x: u128) -> u128 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }

    pub fn new(s: &Vec<char>) -> Self {
        let mut hash = vec![0; s.len() + 1];
        let mut pow = vec![1; s.len() + 1];
        for i in 0..s.len() {
            hash[i+1] = Self::mul(hash[i], BASE) + Self::xorshift(s[i] as u128 + 1);
            pow[i+1] = Self::mul(pow[i], BASE);
            if hash[i+1] >= MOD {
                hash[i+1] -= MOD;
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
        let res = MOD + self.hash[r] - Self::mul(self.hash[l], self.pow[r - l]);
        if res < MOD {
            res
        } else {
            res - MOD
        }
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }
}

#[cfg(test)]
mod test {
    use super::RollingHash;
    use rand::{self, Rng};

    #[test]
    fn random_test() {
        let charset = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();


        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let n: usize = rng.gen_range(2..100000);
            let test_str: Vec<char> = (0..n)
                .map(|_| {
                    let idx = rng.gen_range(0..charset.len());
                    charset[idx]
                })
                .collect();

            let rh = RollingHash::new(&test_str);
            
            for i in 0..100 {
                let l = rng.gen_range(0..n-1);
                let r = rng.gen_range(l+1..n);
                let mut s = vec![];
                for i in l..r {
                    s.push(test_str[i]);
                }
                let rh2 = RollingHash::new(&s);
                assert_eq!(rh.hash(l, r), rh2.hash(0, s.len()));
            }
        }
    }
}