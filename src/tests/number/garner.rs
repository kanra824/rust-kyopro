use crate::library::number::garner::*;
use rand::{rng, Rng};

#[test]
fn test_garner() {
    let mut rng = rng();
    let mut n = 1000;
    let mut m = 10000000000i64;
    let mut a = vec![];
    for i in 0..n {
        a.push(rng.random_range(0..=m));
    }

    let mut pv = vec![1000000007, 998244353];
    let mut b = vec![];
    let p = 1000000009;
    for i in 0..n {
        let mut valv = vec![];
        for j in 0..2 {
            valv.push(a[i] % pv[j]);
        }
        let mut res = garner(pv.clone(), valv.clone(), p);
        b.push(res);
    }

    for i in 0..n {
        assert_eq!(a[i] % p, b[i]);
    }
}