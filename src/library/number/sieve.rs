/// エラトステネスの篩
/// O(loglogn) で素数判定を行う
pub fn sieve(n: usize) -> Vec<bool> {
    let mut res = vec![true; n+1];
    res[0] = false;
    res[1] = false;
    for i in 2..n+1 {
        let mut j = 2 * i;
        while j <= n {
            res[j] = false;
            j += i;
        }
    }
    res
}

#[test]
fn test_sieve() {
    let res = sieve(100);
    for (i, &val) in res.iter().enumerate() {
        if val {
            println!("{}", i);
        }
    }
    assert!(!res[0]);
    assert!(!res[1]);
    assert!(res[2]);
    assert!(res[3]);
    assert!(res[5]);
    assert!(!res[100]);
}