#![allow(unused)]

fn itov(mut n: usize, k: usize) -> Vec<usize> {
    let mut v = vec![];
    for i in 0..k {
        v.push(n % 2);
        n /= 2;
    }
    v.reverse();
    v
}

fn vtoi(v: &Vec<usize>, l: usize, r: usize) -> usize {
    let mut res = 0;
    for i in l..r {
        res *= 2;
        res += v[i];
    }
    res
}

fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        x: [i64; 16],
    }

    let n = 8;
    let mut g = vec![vec![]; n];
    let mut din = vec![0; n];
    let mut dout = vec![0; n];
    for i in 0..n {
        let mut iv = itov(i, 3);
        // i, 0
        iv.push(0);
        let res = vtoi(&iv, 0, 4);
        let val = x[res];
        println!("res: {:04b}, val: {}", res, val);
        let to = vtoi(&iv, 1, 4);
        for j in 0..val {
            if j == 0 {
                println!("{:03b} -> {:03b}", i, to);
            }
            din[to] += 1;
            dout[i] += 1;
            g[i].push(to);
        }

        // i, 1
        iv.pop();
        iv.push(1);
        let res = vtoi(&iv, 0, 4);
        let val = x[res];
        println!("res: {:04b}, val: {}", res, val);
        let to = vtoi(&iv, 1, 4);
        for j in 0..val {
            if j == 0 {
                println!("{:03b} -> {:03b}", i, to);
            }
            g[i].push(to);
        }
    }

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for &to in g[i].iter() {
            uf.unite(i, to);
        }
    }
    if uf.count() != 1 {
        println!("0");
        return;
    }

    let mut ok = true;
    for i in 0..n {
        ok = ok && (din[i] == dout[i]);
    }
    if !ok {
        println!("0");
        return;
    }

    // オイラーグラフをチェック済み
    // BEST theorem: 始点、終点を決めると、（始点を根とする有向木の数) * （総積）｛outdeg - 1}
    

}

//use proconio::input;
//use proconio::marker::{Usize1, Isize1};
pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut par = Vec::new();
        for i in 0..n {
            par.push(i);
        }
        let sz = vec![1; n];
        UnionFind { n: n, par, sz }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    pub fn get_sz(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.sz[x]
    }

     pub fn count(&mut self) -> usize {
         let mut st = std::collections::HashSet::new(); 
         for i in 0..self.n {
            st.insert(self.root(i));
         }
         st.len()
     }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return;
        }

        if self.sz[x] > self.sz[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.sz[y] = self.sz[x] + self.sz[y];
        self.sz[x] = self.sz[y];
        self.par[x] = y;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

// 正方行列
#[derive(Debug, Clone)]
struct Matrix {
    h: usize,
    w: usize,
    a: Vec<Vec<Mint>>,
}

impl Matrix {
    fn new(a: Vec<Vec<Mint>>) -> Self {
        assert!(!a.is_empty());
        assert!(a.iter().all(|v| v.len() == a[0].len()));
        Matrix {
            h: a.len(),
            w: a[0].len(),
            a,
        }
    }

    fn zero(h: usize, w: usize) -> Self {
        Matrix {
            h,
            w,
            a: vec![vec![Mint::zero(); w]; h],
        }
    }

    fn e(n: usize) -> Self {
        let mut a = vec![vec![Mint::zero();n];n];
        for i in 0..n {
            a[i][i] = Mint::new(1);
        }
        Matrix {
            h: n,
            w: n,
            a,
        }
    }

    fn add(&self, b: &Matrix) -> Matrix {
        debug_assert_eq!(self.h, b.h);
        debug_assert_eq!(self.w, b.w);
        let mut res = vec![vec![Mint::zero(); b.w]; self.h];
        for i in 0..self.h {
            for j in 0..self.w {
                res[i][j] = self.a[i][j] + b.a[i][j];
            }
        }
        Matrix {
            h: res.len(),
            w: res[0].len(),
            a: res,
        }
    }

    fn mul(&self, b: &Matrix) -> Matrix {
        debug_assert_eq!(self.w, b.h);
        let mut res = vec![vec![Mint::zero(); b.w]; self.h];
        for i in 0..self.h {
            for j in 0..b.w {
                for k in 0..self.w {
                    res[i][j] = res[i][j] + self.a[i][k] * b.a[k][j];
                }
            }
        }
        Matrix {
            h: res.len(),
            w: res[0].len(),
            a: res,
        }
    }

    fn pow(&self, mut k: i64) -> Matrix {
        assert_eq!(self.h, self.w);
        let mut res = Matrix::e(self.h);
        let mut now = self.clone();
        while k > 0 {
            if k & 1 == 1 {
                res = res.mul(&now);
            }
            now = now.mul(&now);
            k >>= 1;
        }
        res
    }

    fn pivot(a: &Matrix, rank: usize, col: usize) -> usize {
        for i in rank..a.h {
            if a.a[i][col] != Mint::zero() {
                return i;
            }
        }
        return usize::MAX;
    }

    fn sweep(mut a: Matrix, rank: usize, col: usize, pivot: usize) -> Matrix {
        let tmp = a.a[pivot].clone();
        a.a[pivot] = a.a[rank].clone();
        a.a[rank] = tmp;
        
        let div = a.a[rank][col].inv();
        for j in 0..col {
            assert_eq!(a.a[rank][j], Mint::zero());
        }
        for j in col..a.w {
            a.a[rank][j] *= div;
        }

        for i in 0..a.h {
            if i == rank {
                continue;
            }
            for j in 0..col {
                assert_eq!(a.a[rank][j], Mint::zero());
            }
            let tmp = a.a[i][col];
            for j in col..a.w {
                let sub = a.a[rank][j] * tmp;
                a.a[i][j] -= sub;
            }
        }
        a
    }

    fn determinant(&self) -> Mint {
        let mut a = self.clone();
        let mut rank = 0;
        let mut res = Mint::new(1);
        for col in 0..self.w {
            let pivot = Self::pivot(&a, rank, col);
            if pivot == usize::MAX {
                return Mint::zero();
            }
            res *= a.a[pivot][rank];
            a = Self::sweep(a, rank, col, pivot);
            rank += 1;
        }
        res
    }

}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, Read, BufReader, Write, Stdin};
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