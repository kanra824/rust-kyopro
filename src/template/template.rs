#![allow(unused)]

fn main() {
    // // AOJ, codeforces, etc...
    // let mut s = String::new();
    // let stdin = stdin();
    // let mut reader = Reader::new(&mut s, stdin);

    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
    }
    
}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, Stdin, BufReader, Read, Write};
use std::str::FromStr;
use std::{fmt, ops};

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

fn pr_vec<T>(v: &Vec<T>)
where
    T: std::fmt::Display,
{
    for i in 0..v.len() {
        print!("{}", v[i]);
        if i == v.len()-1 {
            println!();
        } else {
            print!(" ");
        }
    }
}

fn pr_yesno(x: bool) {
    if x {
        pr("Yes");
    } else {
        pr("No");
    }
}

/// 単一の値をデバッグプリントするための関数
fn pd<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}

struct Reader<'a> {
    stdin: Stdin,
    tokens: Vec<VecDeque<&'a str>>,
    idx: usize,
}

impl<'a> Reader<'a> {
    fn new(str: &'a mut String, mut stdin: Stdin) -> Self {
        stdin.read_to_string(str).unwrap();
        let tokens: Vec<VecDeque<&str>> = str
            .trim()
            .split('\n')
            .map(|v| v.split_whitespace().collect())
            .collect();
        Reader {
            stdin,
            tokens,
            idx: 0,
        }
    }

    fn r<T: FromStr>(&mut self) -> T {
        let str = self.tokens[self.idx].pop_front().unwrap();
        let res = str.parse().ok().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        res
    }

    fn rv<T: FromStr>(&mut self) -> Vec<T> {
        let deque = &mut self.tokens[self.idx];
        let mut res = vec![];
        while !deque.is_empty() {
            let str = deque.pop_front().unwrap();
            res.push(str.parse().ok().unwrap());
        }
        self.idx += 1;
        res
    }

    fn rl<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut res = vec![];
        let len = self.tokens.len();
        assert!(self.idx + n <= len);
        for i in 0..n {
            let str = self.tokens[self.idx].pop_front().unwrap();
            res.push(str.parse().ok().unwrap());
            assert!(self.tokens[self.idx].is_empty());
            self.idx += 1;
        }
        res
    }

    fn as_chars(&mut self) -> Vec<char> {
        let str = self.tokens[self.idx].pop_front().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        str.chars().collect()
    }
}

// グリッドの範囲を見てすすめるマスを列挙(壁がある場合は呼び出し側でチェック)
fn adj_pos(w: usize, h: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (dr, dc) in dir {
        let nr = r as i64 + dr;
        let nc = c as i64 + dc;
        if !(0 <= nr && nr < h as i64 && 0 <= nc && nc < w as i64) {
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        res.push((nr, nc))
    }
    res
}

// ---------------------------------------------------------------------------------------
// modint
pub const MOD: i64 = MOD998;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint {
    x: i64,
}

impl std::fmt::Display for Modint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl Modint {
    pub fn new(x: i64) -> Self {
        Modint { x : (x % MOD) as i64 }
    }

    pub fn zero() -> Self {
        Modint::new(0)
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
        self.pow((MOD - 2) as i64)
    }
}

impl ops::Neg for Modint {
    type Output = Modint;

    fn neg(mut self) -> Modint {
        self.x = (MOD - self.x);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl ops::Add<Self> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: Self) -> Modint {
        self + rhs.x
    }
}

impl ops::Add<i64> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: i64) -> Modint {
        self.x = (self.x + rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl ops::Sub<Self> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: Self) -> Modint {
        self - rhs.x
    }
}

impl ops::Sub<i64> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: i64) -> Modint {
        self.x = (self.x + MOD - rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl ops::Mul<Self> for Modint {
    type Output = Modint;
    fn mul(mut self, rhs: Self) -> Modint {
        self * rhs.x
    }
}

impl ops::Mul<i64> for Modint {
    type Output = Modint;
    fn mul(mut self, rhs: i64) -> Modint {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl ops::Div<Self> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: Self) -> Modint {
        self / rhs.x
    }
}

impl ops::Div<i64> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: i64) -> Modint {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::new(rhs).inv()
    }
}

impl ops::Rem<Self> for Modint {
    // implement only for num_traits::NumOps
    type Output = Modint;
    fn rem(mut self, rhs: Self) -> Modint {
        panic!("cannot rem");
    }
}


impl ops::AddAssign<Self> for Modint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<i64> for Modint {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl ops::SubAssign<Self> for Modint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::SubAssign<i64> for Modint {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl ops::MulAssign<Self> for Modint {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<i64> for Modint {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl ops::DivAssign<Self> for Modint {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::DivAssign<i64> for Modint {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

// ----------------------------------------------------------------------------------
// Graph
pub type Cost = i64;

pub struct Graph {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: Cost) {
        self.g.get_mut(a).unwrap().push((b, c))
    }

    pub fn edges(&mut self) -> Vec<(usize, usize, Cost)> {
        let mut res = vec![];
        for i in 0..self.n {
            for &(j, cost) in self.g[i].iter() {
                res.push((i, j, cost));
            }
        }
        res
    }
}