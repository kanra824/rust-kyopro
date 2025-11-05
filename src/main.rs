#![allow(unused)]

fn calc_matrix(a: &Vec<Vec<bool>>, i: usize) -> Vec<Vec<i32>> {
    let mut res = vec![vec![i32::MAX; 3]; 3];

    if a[i][0] {
        if a[i+1][0] {
            res[0][0] = 1;
        }

        if a[i+1][1] && (a[i+1][0] || a[i][1]) {
            res[0][1] = 2;
        }

        if a[i+1][2] {
            if a[i+1][0] && a[i+1][1] 
               || a[i][1] && a[i+1][1] 
               || a[i][1] && a[i][2] {
                res[0][2] = 3;
            }
        }
    }

    if a[i][1] {
        if a[i+1][0] && (a[i][0] || a[i+1][1]) {
            res[1][0] = 2;
        }

        if a[i+1][1] {
            res[1][1] = 1;
        }

        if a[i+1][2] && (a[i][2] || a[i+1][1]) {
            res[1][2] = 2;
        }
    }

    if a[i][2] {
        if a[i+1][0] {
            if a[i+1][2] && a[i+1][1]
              || a[i][1] && a[i+1][1]
              || a[i][1] && a[i][0] {
                res[2][0] = 3;
              }
        }

        if a[i+1][1] && (a[i+1][2] || a[i][1]) {
            res[2][1] = 2;
        }

        if a[i+1][2] {
            res[2][2] = 1;
        }
    }

    // pd(a[i].clone());
    // pd(a[i+1].clone());
    // pd(&res);

    res
}

fn op(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![i32::MAX; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut val = i32::MAX;
            for k in 0..3 {
                if a[i][k] != i32::MAX && b[k][j] != i32::MAX {
                    val = val.min(a[i][k] + b[k][j]);
                }
            }
            res[i][j] = val;
        }
    }
    // pd(&a);
    // pd(&b);
    // pd(&res);
    // println!();

    res
}

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
        n: usize,
        mut s: [Chars; 3],
        q: usize,
    }

    let mut a = vec![vec![false; 3]; n];
    for i in 0..3 {
        for j in 0..n {
            a[j][i] = s[i][j] == '.';
        }
    }

    let mut init = vec![];
    for i in 0..n-1 {
        init.push(calc_matrix(&a, i));
    }
    // for i in 0..n-1 {
    //     pd(&init[i]);
    // }
    // println!();

    let mut st = SegmentTree::new(
        n-1,
        init,
        |a, b| op(&a, &b),
        |a, b| b,
        vec![vec![0, i32::MAX, i32::MAX], vec![i32::MAX, 0, i32::MAX], vec![i32::MAX, i32::MAX, 0]],
    );
    // for i in 0..n-1 {
    //     pd(&st.query(i, i+1));
    // }
    // println!();

    for i in 0..q {
        input! {
            j: Usize1,
            i: Usize1,
        }

        a[i][j] = !a[i][j];
        // pd(a.clone());
        // for i in 0..n-1 {
        //     pd(st.query(i, i+1));
        // }

        if i != n-1 {
            let v = calc_matrix(&a, i);
            st.update(i, v);
        }

        if i != 0 {
            let v = calc_matrix(&a, i-1);
            st.update(i-1, v);
        }

        let res = st.query(0, n-1);
        // for i in 0..3 {
        //     for j in 0..n {
        //         print!("{}", s[i][j]);
        //     }
        //     println!();
        // }
        // pd(&res);
        if res[0][2] == i32::MAX {
            pr(-1);
        } else {
            pr(res[0][2]);
        }
    }

}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};
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
        if i == v.len() - 1 {
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

    // read a token
    fn r<T: FromStr>(&mut self) -> T {
        let str = self.tokens[self.idx].pop_front().unwrap();
        let res = str.parse().ok().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        res
    }

    // read vec
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

    // read n lines
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

    // read string as chars
    fn as_chars(&mut self) -> Vec<char> {
        let str = self.tokens[self.idx].pop_front().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        str.chars().collect()
    }

    fn end_input(&self) -> bool {
        self.idx > self.tokens.len()
    }
}

// グリッドの範囲を見てすすめるマスを列挙(壁がある場合は呼び出し側でチェック)
fn adj_pos(h: usize, w: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let dir = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
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

fn char_to_i64(c: char) -> i64 {
    c as u32 as i64 - '0' as u32 as i64
}

#[derive(Clone, Debug)]
pub struct SegmentTree<T, F, G>
{
    n: usize,
    pub v: Vec<T>,
    f: F,
    g: G,
    zero: T,
}

impl<T, F, G> SegmentTree<T, F, G>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
    G: Fn(T, T) -> T,
{
    pub fn new(n: usize, v: Vec<T>, f: F, g: G, zero: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }

        let mut v_ = vec![zero.clone(); 2 * n_];
        for i in 0..n {
            v_[n_ + i] = v[i].clone();
        }
        for i in (1..=n_ - 1).rev() {
            v_[i] = f(&v_[i * 2], &v_[i * 2 + 1]);
        }

        SegmentTree {
            n: n_,
            v: v_,
            f,
            g,
            zero,
        }
    }

    pub fn update(&mut self, i: usize, x: T) {
        self.v[self.n + i] = (self.g)(self.v[self.n + i].clone(), x);
        let mut now = (self.n + i) / 2;
        while now > 0 {
            self.v[now] = (self.f)(&self.v[now * 2], &self.v[now * 2 + 1]);
            now /= 2;
        }
    }

    fn query_(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
        if r <= a || b <= l {
            return self.zero.clone();
        }
        if a <= l && r <= b {
            return self.v[k].clone();
        }

        let val1 = self.query_(l, (l + r) / 2, 2 * k, a, b);
        let val2 = self.query_((l + r) / 2, r, 2 * k + 1, a, b);
        (self.f)(&val1, &val2)
    }

    pub fn query(&self, a: usize, b: usize) -> T {
        self.query_(0, self.n, 1, a, b)
    }
}
