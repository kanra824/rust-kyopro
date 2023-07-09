use proconio::input;
use proconio::marker::{Usize1, Isize1};

const MOD:i64 = 998244353;

fn main() {
    input! {
        n: i64,
        m: i64,
    }

    let mut a = 1;
    let mut b = 0;

    for i in 0..n-1 {
        let preva = a;
        let prevb = b;
        a = prevb * (m - 1) % MOD;
        b = (preva + prevb * (m - 2) % MOD) % MOD;
    }

    println!("{}", m * b % MOD * (m - 1) % MOD);
}
