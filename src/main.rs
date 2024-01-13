#![allow(unused)]

fn ctoi(c: char) -> usize {
    c as usize - 'A' as usize
}

fn calc_score(r: usize, c: usize, nr: usize, nc: usize) -> i64 {
    let r = r as i64;
    let c = c as i64;
    let nr = nr as i64;
    let nc = nc as i64;
    (r - nr).abs() + (c - nc).abs() + 1
}

fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    let start = Instant::now();

    input! {
        // from &mut source,
        n: usize,
        m: usize,
        (sr, sc): (usize, usize),
        g: [Chars; n],
        mut t: [Chars; m],
    }

    let mut rng = ChaCha20Rng::seed_from_u64(0);
    t.shuffle(&mut rng);

    let sz = 5;

    let mut v = vec![vec![]; 26];
    for i in 0..n {
        for j in 0..n {
            v[ctoi(g[i][j])].push((i, j));
        }
    }

    let mut min_score = i64::MAX;
    let mut best_ans = vec![(0, 0)];

    loop {
        let now = Instant::now();
        if now - start >= Duration::from_millis(1900) {
            break;
        }

        let (mut r, mut c) = (sr, sc);
        let mut dp = vec![vec![(0, (sr, sc), usize::MAX)]];
        let mut idx = 0;
        for i in 0..m {
            let mut overlap = 0;
            if i > 0 {
                // かぶりチェック
                for j in (1..=5).rev() {
                    let mut ok = true;
                    for k in 0..j {
                        ok = ok && (t[i-1][5-j+k] == t[i][k]);
                    }
                    if ok {
                        overlap = j;
                        break;
                    }
                }
            }
            for j in overlap..sz {
                let mut ndp = vec![(i64::MAX, (usize::MAX, usize::MAX), usize::MAX); v[ctoi(t[i][j])].len()];
                for (k, &(now, (r, c), _)) in dp[idx].iter().enumerate() {
                    for (l, &(nr, nc)) in v[ctoi(t[i][j])].iter().enumerate() {
                        let nxt = now + calc_score(r, c, nr, nc);
                        if nxt < ndp[l].0 {
                            ndp[l].0 = nxt;
                            ndp[l].1 = (nr, nc);
                            ndp[l].2 = k
                        }
                    }
                }
                dp.push(ndp);
                idx += 1;
            }
        }

        let mut ans = vec![];
        let mut prev = 0;
        let dpsz = dp.len();
        let mut score = dp[dpsz-1][0].0;
        for i in 1..dp[dpsz-1].len() {
            if dp[dpsz-1][i].0 < score {
                score = dp[dpsz-1][i].0;
                prev = i;
            }
        }

        if score > min_score {
            continue;
        }

        for i in (1..=dpsz-1).rev() {
            ans.push(dp[i][prev].1);
            prev = dp[i][prev].2;
        }

        ans.reverse();
        best_ans = ans;
    }

    for (r, c) in best_ans {
        println!("{} {}", r, c);
    }

}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use rand::seq::SliceRandom;
use rand::prelude::*;
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha20Rng;
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Write};
use std::str::FromStr;
use std::time::{Duration, Instant};

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

// dir の方向にすすむ
fn next_pos(w: usize, h: usize, now: (usize, usize), dir: (i64, i64)) -> Option<(usize, usize)> {
    let nr = now.0 as i64 + dir.0;
    let nc = now.1 as i64 + dir.1;
    if !(0 <= nr && nr < h as i64 && 0 <= nc && nc < w as i64) {
        return None;
    }
    let nr = nr as usize;
    let nc = nc as usize;
    Some((nr, nc))
}
