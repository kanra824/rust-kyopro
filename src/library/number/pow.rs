pub fn pow(n: i64, mut k: i64) -> i64 {
    let mut res = 1;
    let mut mul = n;
    while k > 0 {
        if k & 1 == 1 {
            res *= mul;
        }
        mul *= mul;
        k /= 2;
    }
    res
}
