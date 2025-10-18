#![allow(unused)]

fn main() {
    // // AOJ, codeforces, etc...
    let mut s = String::new();
    let stdin = stdin();
    let mut reader = Reader::new(&mut s, stdin);

    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    let n: usize = reader.r();
    let mut g = vec![vec![]; n];
    for i in 0..n {
        let k: usize = reader.r();
        for j in 0..k {
            let mut child: usize = reader.r();
            g[i].push(child);
            g[child].push(i);
        }
    }

    let mut hld = HeavyLightDecomposition::new(n, g);
    let res = hld.hld(0);
    // pd(res);


    let q: usize = reader.r();
    for i in 0..q {
        let mut u: usize = reader.r();
        let mut v: usize = reader.r();
        // println!("{} {}", u, v);
        // pd(hld.query(u, v));
        pr(hld.lca(u, v));
    }
    
}

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

    fn end_input(&self) -> bool {
        self.idx > self.tokens.len()
    }
}

// グリッドの範囲を見てすすめるマスを列挙(壁がある場合は呼び出し側でチェック)
fn adj_pos(w: usize, h: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
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

// https://qiita.com/recuraki/items/cb888afdc107b64a4a6e
// verify: https://atcoder.jp/contests/abc294/submissions/70278032

pub struct HeavyLightDecomposition {
    n: usize,
    g: Vec<Vec<usize>>,
    prev: Vec<usize>,
    depth: Vec<i64>,
    child_cnt: Vec<i64>,
    node_to_hld: Vec<usize>,
    hld_to_node: Vec<usize>,
    shallow: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new(n: usize, g: Vec<Vec<usize>>) -> Self {
        HeavyLightDecomposition {
            n,
            g,
            prev: vec![usize::MAX; n],
            depth: vec![i64::MAX; n],
            child_cnt: vec![i64::MAX; n],
            node_to_hld: vec![usize::MAX; n],
            hld_to_node: vec![],
            shallow: vec![usize::MAX; n],
        }
    }

    pub fn hld(&mut self, root: usize) -> Vec<usize> {
        self.dfs(root, usize::MAX, 0);
        self.hld_rec(root, root);
        self.node_to_hld.clone()
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        while self.shallow[u] != self.shallow[v] {
            // 浅いほうを u
            if self.depth[self.shallow[u]] > self.depth[self.shallow[v]] {
                std::mem::swap(&mut u, &mut v);
            }

            // v は shallow の前
            v = self.prev[self.shallow[v]];
        }
        if self.node_to_hld[u] < self.node_to_hld[v] {
            u
        } else {
            v
        }
    }

    pub fn query(&self, mut u: usize, mut v: usize) -> Vec<(usize, usize)> {
        // 同じ列に含まれる閉区間の列
        // 同じ列の中では (浅いほう、深いほう) の順で返す

        let mut res = vec![];
        while self.shallow[u] != self.shallow[v] {
            // 浅いほうを u
            if self.depth[self.shallow[u]] > self.depth[self.shallow[v]] {
                std::mem::swap(&mut u, &mut v);
            }

            // 深いほうを push
            res.push((self.node_to_hld[self.shallow[v]], self.node_to_hld[v]));
            v = self.prev[self.shallow[v]];
        }
        let mut val = (self.node_to_hld[u], self.node_to_hld[v]);
        if val.0 > val.1 {
            val = (val.1, val.0);
        }
        res.push(val);
        res
    }

    fn dfs(&mut self, now: usize, prev: usize, nowd: i64) {
        self.prev[now] = prev;
        self.depth[now] = nowd;
        self.child_cnt[now] = 1;
        for nxt in self.g[now].clone() {
            if nxt == prev {
                continue;
            }
            self.dfs(nxt, now, nowd + 1);
            self.child_cnt[now] += self.child_cnt[nxt];
        }
    }

    fn hld_rec(&mut self, now: usize, top: usize) {
        self.node_to_hld[now] = self.hld_to_node.len();
        self.hld_to_node.push(now);
        self.shallow[now] = top;
        if self.child_cnt[now] == 1 {
            return;
        }

        let mut ma = 0;
        let mut maidx = usize::MAX;
        for i in 0..self.g[now].len() {
            let nxt = self.g[now][i];
            if nxt == self.prev[now] {
                continue;
            }
            if self.child_cnt[nxt] > ma {
                ma = self.child_cnt[nxt];
                maidx = nxt;
            }
        }

        self.hld_rec(maidx, top);

        for i in 0..self.g[now].len() {
            let nxt = self.g[now][i];
            if nxt == self.prev[now] {
                continue;
            }
            if nxt == maidx {
                continue;
            }
            self.hld_rec(nxt, nxt);
        }
    }
}