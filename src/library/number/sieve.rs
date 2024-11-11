/// エラトステネスの篩
/// O(loglogn) で素数判定を行う
pub fn sieve(n: usize) -> Vec<bool> {
    let mut res = vec![true; n+1];
    res[0] = false;
    res[1] = false;
    for i in 2..n+1 {
        if !res[i] {
            continue;
        }
        let mut j = 2 * i;
        while j <= n {
            res[j] = false;
            j += i;
        }
    }
    res
}
