#![allow(unused)]

fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: usize,
    }

    type Mint = Modint<MOD998>;

    let mut dp = vec![Mint::new(1)];
    let mut mint1 = Mint::new(1);
    let mut mint2 = Mint::new(2);
    let mut inv2 = mint1 / mint2;
    let mut r = inv2;
    let mut div = mint2;
    for i in 1..n {
        //pr(i);
        for j in 0..i {
            dp[j] = dp[j] / Mint::new(2);
        }

        let mut dp2 = vec![Mint::new(0); i + 1];
        r = r * inv2;
        div = div * mint2;
        let mir = mint1 - r;
        let mut val = Mint::new(0);

        let divr = mint1 / (mint1 - r);
        let invdiv = mint1 / div;
        for k in 0..i + 1 {
            val = val * inv2;
            if k == 0 {
                let j = 0;
                val = val + dp[j] * divr;
            } else if k != 1 {
                let j = k - 1;
                val = val + dp[j] * divr;
            }
            dp2[k] = dp2[k] + val;
        }
        for k in 0..i + 1 {
            val = val * inv2;
            if k == 0 {
                let j = 0;
                val = val - dp[j] * divr * invdiv;
            } else if k != 1 {
                let j = k - 1;
                val = val - dp[j] * divr * invdiv;
            }
            dp2[k] = dp2[k] + val;
        }

        // 初項ごとに
        // for j in 0..i {
        //     let ij = idx[j];
        //     for k in 0..i+1 {
        //         let ik = (ij + k) % (i + 1);
        //         dp2[ik] = dp2[ik] + dp[j] / (Mint::new(1) - r) / v[k];
        //     }
        // }
        // pd(dp.clone());
        // pd(dp2.clone());
        // pd(dp2[0] + dp2[1]);
        dp = dp2;
    }
    for i in 1..n {
        print!("{} ", dp[i]);
    }
    println!("{}", dp[0]);
}

use std::{fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint<const MOD: i64> {
    x: i64,
}

impl<const MOD: i64> std::fmt::Display for Modint<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl<const MOD: i64> Modint<MOD> {
    pub fn zero() -> Self {
        Modint { x: 0 }
    }

    pub fn new(x: i64) -> Self {
        Modint { x: x % MOD }
    }

    pub fn pow(&self, mut k: i64) -> Self {
        let mut mul = Modint::new(self.x);
        let mut res = Modint::new(1);
        while k > 0 {
            if k & 1 == 1 {
                res = res * mul;
            }
            mul = mul * mul;
            k /= 2;
        }
        res
    }

    pub fn inv(&self) -> Self {
        if self.x == 0 {
            panic!("0 has no inv");
        }
        self.pow(MOD - 2)
    }
}

impl<const MOD: i64> ops::Neg for Modint<MOD> {
    type Output = Modint<MOD>;

    fn neg(mut self) -> Modint<MOD> {
        self.x = (MOD - self.x) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Add<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: Self) -> Modint<MOD> {
        self.x = (self.x + rhs.x) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Add<i64> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: i64) -> Modint<MOD> {
        self.x = (self.x + rhs) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Sub<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: Self) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs.x) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Sub<i64> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: i64) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs) % MOD;
        self
    }
}

impl<const MOD: i64> ops::Mul<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: Self) -> Modint<MOD> {
        self.x = self.x * rhs.x % MOD;
        self
    }
}

impl<const MOD: i64> ops::Mul<i64> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: i64) -> Modint<MOD> {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl<const MOD: i64> ops::Div<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: Self) -> Modint<MOD> {
        if rhs.x == 0 {
            panic!("0 division is occured");
        }
        self.x = self.x * rhs.inv().x % MOD;
        self
    }
}

impl<const MOD: i64> ops::Div<i64> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: i64) -> Modint<MOD> {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self.x = self.x * Modint::<MOD>::new(rhs).inv().x % MOD;
        self
    }
}

impl<const MOD: i64> ops::AddAssign<Self> for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64> ops::AddAssign<i64> for Modint<MOD> {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl<const MOD: i64> ops::SubAssign<Self> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64> ops::SubAssign<i64> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl<const MOD: i64> ops::MulAssign<Self> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64> ops::MulAssign<i64> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl<const MOD: i64> ops::DivAssign<Self> for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: i64> ops::DivAssign<i64> for Modint<MOD> {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Write};
use std::str::FromStr;

/// 有名MODその1
const MOD998: i64 = 998244353;
/// 有名MODその2
const MOD107: i64 = 1000000007;

/// 単一の値をプリントするための関数
fn pr<T>(val: T)
where
    T: std::fmt::Display,
{
    println!("{}", val);
}

/// 単一の値をデバッグプリントするための関数
fn pd<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}

/// 単一の値を入力する
fn input<T: FromStr>() -> T {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().ok().unwrap()
}

/// 一行の複数の値を入力する
fn input_vec<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let v = buffer
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    v
}

// TODO: 複数の型が入り得る場合を処理したい（どうやって？）
/// 複数行を入力する
fn input_lines<T: FromStr>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    for i in 0..n {
        v.push(input());
    }
    v
}
