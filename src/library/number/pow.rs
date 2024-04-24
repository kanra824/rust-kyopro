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

#[test]
fn pow_test() {
    assert_eq!(pow(2, 3), 8);
    assert_eq!(pow(3, 4), 81);
    assert_eq!(pow(10, 4), 10000);
    assert_eq!(pow(5, 3), 125);
}