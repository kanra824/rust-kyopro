fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
    }
}

#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, Write};
#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused)]
const MOD998: i64 = 998244353;
#[allow(unused)]
const MOD107: i64 = 1000000007;

#[allow(unused)]
fn pr<T>(val: T)
where
    T: std::fmt::Display,
{
    println!("{}", val);
}

#[allow(unused)]
fn pd<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}


fn input<T: FromStr>() -> T {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().ok().unwrap()
}

fn input_vec<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let v = buffer.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
    v
}

// 複数の型が入り得る場合を処理したい（どうやって？）

fn input_lines<T: FromStr>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    for i in 0..n {
        v.push(input());
    }
    v
}
