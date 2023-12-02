#![allow(unused)]

use std::ops::{Add, Sub};

pub struct TwoDemensionSum<T>
{
    su: Vec<Vec<T>>,
    zero: T,
}

impl<T> TwoDemensionSum<T> 
where
    T: Clone + Copy + Add<Output = T> + Sub<Output = T>,
{
    /// 高さ h, 幅 w, 初期値 zero で初期化
    pub fn new(h: usize, w: usize, zero: T, v: &Vec<Vec<T>>) -> Self {
        let mut su = vec![vec![zero; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                su[i + 1][j + 1] = su[i + 1][j] + su[i][j + 1] - su[i][j] + v[i][j];
            }
        }
        TwoDemensionSum { su, zero, }
    }

    /// [x1, x2), [y1, y2) の和
    pub fn sum(&self, cor1: (usize, usize), cor2: (usize, usize)) -> T {
        let (x1, y1) = cor1;
        let (x2, y2) = (cor2.0, cor2.1);
        if x1 >= x2 || y1 >= y2 {
            return self.zero
        }
        self.su[x2][y2] - self.su[x1][y2] - self.su[x2][y1] + self.su[x1][y1]
    }
}
fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: usize,
        q: usize,
        p: [Chars;n],
        queries: [(usize, usize, usize, usize); q],
    }

    let mut v = vec![vec![0i64;2*n];2*n];
    for i in 0..2*n {
        for j in 0..2*n {
            if p[i%n][j%n] == 'B' {
                v[i][j] = 1;
            } else {
                v[i][j] = 0;
            }
        }
    }
    let mut su = TwoDemensionSum::new(2*n, 2*n, 0, &v);

    for (a, b, c, d) in queries {
        let c = c + 1;
        let d = d + 1;
        let cntr = ((c - a) / n) as i64;
        let cntc = ((d - b) / n) as i64;
        let row = (c - a) % n;
        let col = (d - b) % n;

        let mut ans = cntr * cntc * su.sum((0, 0), (n, n));
        ans += cntc * su.sum((a%n, 0), (a%n+row, n));
        ans += cntr * su.sum((0, b%n), (n, b%n+col));
        ans += su.sum((a%n, b%n), (a%n+row, b%n+col));

        pr(ans);
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
