#![allow(unused)]

mod library;

fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: usize,
        m: usize,
        a: [i64; n],
        queries: [(Usize1, Usize1, i64); m],
    }

    let mut data = vec![];
    for i in 0..n {
        data.push(Mint::new(a[i]));
    }

    let zero = Mint::zero();
    let one = Mint::new(1);
    let mut st = LazySegmentTree::new(
        n,
        |a, b| a + b,
        |a, b, len| {b.0 * a + b.1},
        |a, b| (a.0 * b.0, b.0 * a.1 + b.1),
        zero,
        (one, zero),
    );
    st.build(&data);

    for (l, r, x) in queries {
        let p = Mint::new((r + 1 - l) as i64).inv();
        st.update(l, r + 1, (one - p, p * Mint::new(x)));
    }

    for i in 0..n {
        pr(st.query(i, i+1));
    }
}

pub struct LazySegmentTree<T, OT, F, G, H>
where
    T: Clone + Copy,
    OT: Clone + Copy + PartialEq + Eq,
    F: Fn(T, T) -> T,
    G: Fn(T, OT, usize) -> T,
    H: Fn(OT, OT) -> OT,
{
    n: usize,
    sz: usize,
    data: Vec<T>,
    lazy: Vec<OT>,
    f: F,
    g: G,
    h: H,
    e: T,
    oe: OT,
}

impl<T, OT, F, G, H> LazySegmentTree<T, OT, F, G, H>
where
    T: Clone + Copy,
    OT: Clone + Copy + PartialEq + Eq,
    F: Fn(T, T) -> T,
    G: Fn(T, OT, usize) -> T,
    H: Fn(OT, OT) -> OT,
{
    pub fn new(n: usize, f: F, g: G, h: H, e: T, oe: OT) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }

        LazySegmentTree {
            n,
            sz,
            data: vec![e; 2 * sz],
            lazy: vec![oe; 2 * sz],
            f,
            g,
            h,
            e,
            oe,
        }
    }

    // 初期化する. O(n)
    // v: 初期値の配列
    // 使用例
    // let v = vec![1, 2, 3, 4, 5];
    // st.build(&v)
    pub fn build(&mut self, v: &Vec<T>) {
        assert!(v.len() == self.n);
        for i in 0..self.n {
            self.data[self.sz + i] = v[i];
        }
        for i in (1..self.sz).rev() {
            self.data[i] = (self.f)(self.data[i * 2], self.data[i * 2 + 1]);
        }
    }

    // 指定した区間に作用素 x を作用させる O(log n)
    // a, b: [a, b) に x を作用させる
    // x: 作用素モノイドの元
    // 使い方 (区間[a, b) を x に変更)
    // st.update(a, b, x);
    pub fn update(&mut self, a: usize, b: usize, x: OT) {
        self.update_range(a, b, x, 1, 0, self.sz);
    }

    // 指定した区間に取得クエリを実行する O(log n)
    // a, b: [a, b) のクエリを実行する
    // st.query(a, b)
    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.query_range(a, b, 1, 0, self.sz)
    }

    fn propagate(&mut self, k: usize, len: usize) {
        if self.lazy[k] == self.oe {
            return;
        }
        if k < self.sz {
            self.lazy[k * 2] = (self.h)(self.lazy[k * 2], self.lazy[k]);
            self.lazy[k * 2 + 1] = (self.h)(self.lazy[k * 2 + 1], self.lazy[k]);
        }
        self.data[k] = (self.g)(self.data[k], self.lazy[k], len);
        self.lazy[k] = self.oe;
    }

    fn update_range(&mut self, a: usize, b: usize, x: OT, k: usize, l: usize, r: usize) {
        self.propagate(k, r - l);
        if r <= a || b <= l {
            return;
        } else if a <= l && r <= b {
            self.lazy[k] = (self.h)(self.lazy[k], x);
            self.propagate(k, r - l);
        } else {
            self.update_range(a, b, x, k * 2, l, (l + r) / 2);
            self.update_range(a, b, x, k * 2 + 1, (l + r) / 2, r);
            self.data[k] = (self.f)(self.data[k * 2], self.data[k * 2 + 1]);
        }
    }

    fn query_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.propagate(k, r - l);
        if r <= a || b <= l {
            return self.e;
        } else if a <= l && r <= b {
            return self.data[k];
        } else {
            let vl = self.query_range(a, b, k * 2, l, (l + r) / 2);
            let vr = self.query_range(a, b, k * 2 + 1, (l + r) / 2, r);
            return (self.f)(vl, vr);
        }
    }
}


use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, Stdin, BufReader, Read, Write};
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
    let v = buffer.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
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

type Mint = Modint<MOD998>;

use std::{fmt, ops};
type ModintMod = i64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint<const MOD: ModintMod> {
    x: ModintMod,
}

impl<const MOD: ModintMod> std::fmt::Display for Modint<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl<const MOD: ModintMod> Modint<MOD> {
    pub fn zero() -> Self {
        Modint { x: 0 }
    }

    pub fn new(x: ModintMod) -> Self {
        Modint { x : (x % MOD) as ModintMod }
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

impl<const MOD: ModintMod> ops::Neg for Modint<MOD> {
    type Output = Modint<MOD>;

    fn neg(mut self) -> Modint<MOD> {
        self.x = (MOD - self.x);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Add<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: Self) -> Modint<MOD> {
        self + rhs.x
    }
}

impl<const MOD: ModintMod> ops::Add<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = (self.x + rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Sub<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: Self) -> Modint<MOD> {
        self - rhs.x
    }
}

impl<const MOD: ModintMod> ops::Sub<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Mul<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: Self) -> Modint<MOD> {
        self * rhs.x
    }
}

impl<const MOD: ModintMod> ops::Mul<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl<const MOD: ModintMod> ops::Div<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: Self) -> Modint<MOD> {
        self / rhs.x
    }
}

impl<const MOD: ModintMod> ops::Div<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: ModintMod) -> Modint<MOD> {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::<MOD>::new(rhs).inv()
    }
}

impl<const MOD: ModintMod> ops::AddAssign<Self> for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: ModintMod> ops::AddAssign<ModintMod> for Modint<MOD> {
    fn add_assign(&mut self, rhs: ModintMod) {
        *self = *self + rhs;
    }
}

impl<const MOD: ModintMod> ops::SubAssign<Self> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: ModintMod> ops::SubAssign<ModintMod> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: ModintMod) {
        *self = *self - rhs;
    }
}

impl<const MOD: ModintMod> ops::MulAssign<Self> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: ModintMod> ops::MulAssign<ModintMod> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: ModintMod) {
        *self = *self * rhs;
    }
}

impl<const MOD: ModintMod> ops::DivAssign<Self> for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: ModintMod> ops::DivAssign<ModintMod> for Modint<MOD> {
    fn div_assign(&mut self, rhs: ModintMod) {
        *self = *self / rhs;
    }
}