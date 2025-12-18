use super::gcd::*;

// ax + by = gcd(a, b) となる整数解 (x, y) を求める
pub fn ext_gcd(a: i64, b: i64, x: i64, y: i64) -> (i64, i64, i64) {
    if b == 0 {
        (1, 0, a)
    } else {
        let (y, x, g) = ext_gcd(b, a % b, y, x);
        (x, y - a / b * x, g)
    }
}