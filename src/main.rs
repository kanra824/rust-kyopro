use proconio::input;
use proconio::marker::{Usize1, Isize1};

const MOD998: i64 = 998244353;

fn pow(mut a: i64, mut b: i64) -> i64 {
    let mut res = 1;
    while b > 0 {
        if b % 2 == 1 {
            res *= a % MOD998;
            res %= MOD998;
        }
        b /= 2;
        a *= a;
        a %= MOD998;
    }
    res
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![0;1<<11];n+1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..1<<11 {
            for k in 1..std::cmp::min(a[i]+1, 11) {
                let mut nxt = j;
                for l in 0..11 {
                    if ((j>>l)&1 == 1) && (k + l <= 10) {
                        nxt = nxt | (1<<(k+l));
                    }
                }
                nxt = nxt | (1<<k);
                dp[i+1][nxt] += dp[i][j];
                dp[i+1][nxt] %= MOD998;
            }
            if a[i] > 10 {
                dp[i+1][j] += dp[i][j] * (a[i] - 10) % MOD998;
                dp[i+1][j] %= MOD998;
            }
        }
    }

    let mut sum = 0;
    for i in 1<<10..1<<11 {
        sum += dp[n][i];
        sum %= MOD998;
    }

    let mut mul = 1;
    for i in 0..n {
        mul *= a[i];
        mul %= MOD998;
    }

    println!("{}", sum * (pow(mul, MOD998 - 2)) % MOD998);
    
}
