use crate::library::others::rolling_hash::*;
use rand::{self, Rng};

#[test]
fn random_test() {
    let charset = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();

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
        let l = rng.gen_range(0..n - 1);
        let r = rng.gen_range(l + 1..n);
        let mut s = vec![];
        for i in l..r {
            s.push(test_str[i]);
        }
        let rh2 = RollingHash::new(&s);
        assert_eq!(rh.hash(l, r), rh2.hash(0, s.len()));
    }
}