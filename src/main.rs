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
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut g = Graph::from_unweighted_edges(n, edges);
    let mut scc = g.strongly_connected_components();

    pr(scc.len());
    for v in scc {
        print!("{}", v.len());
        for val in v {
            print!(" {}", val);
        }
        println!();
    }
    
}

mod library;
mod tests;

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

pub type Cost = i64;

pub struct Graph {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
    pub edges: Vec<(usize, usize, Cost)>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            g: vec![Vec::new(); n],
            edges: vec![],
        }
    }

    pub fn from_edges(n: usize, edges: Vec<(usize, usize, Cost)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v, c) in edges {
            graph.add_edge(u, v, c);
        }
        graph
    }

    pub fn from_unweighted_edges(n: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v) in edges {
            graph.add_edge(u, v, 1);
        }
        graph
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: Cost) {
        self.g.get_mut(a).unwrap().push((b, c));
        self.edges.push((a, b, c));
    }

    pub fn edges(&self) -> Vec<(usize, usize, Cost)> {
        self.edges.clone()
    }

    pub fn rev(&self) -> Self {
        let mut revg = Graph::new(self.n);
        for &(u, v, c) in &self.edges {
            revg.add_edge(v, u, c);
        }
        revg
    }
}

pub trait StronglyConnectedComponents {
    fn strongly_connected_components(&self) -> Vec<Vec<usize>>;
}

impl StronglyConnectedComponents for Graph {
    // 強連結成分の Vec をトポロジカルソート順に格納
    fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let mut sel = vec![false; self.n];
        let mut num = vec![usize::MAX; self.n];
        let mut id = 0;
        for i in 0..self.n {
            if !sel[i] {
                dfs_scc1(i, usize::MAX, &self.g, &mut sel, &mut num, &mut id);
            }
        }

        let mut v = vec![];
        for i in 0..self.n {
            v.push((num[i], i));
        }
        v.sort();
        v.reverse();

        let mut revg = self.rev();
        let mut res = vec![];
        sel = vec![false; self.n];
        for i in 0..self.n {
            let idx = v[i].1;
            if sel[idx] {
                continue;
            }
            let mut resv = vec![];
            dfs_scc2(idx, usize::MAX, &revg.g, &mut sel, &mut resv);
            res.push(resv);
        }

        res
    }

}

fn dfs_scc1(now: usize, prev: usize, g: &Vec<Vec<(usize, Cost)>>, sel: &mut Vec<bool>, num: &mut Vec<usize>, id: &mut usize) {
    sel[now] = true;

    for &(nxt, _) in &g[now] {
        if nxt == prev {
            continue;
        }
        if sel[nxt] {
            continue;
        }

        dfs_scc1(nxt, now, g, sel, num, id);
    }

    num[now] = *id;
    *id += 1;
}

fn dfs_scc2(now: usize, prev: usize, g: &Vec<Vec<(usize, Cost)>>, sel: &mut Vec<bool>, res: &mut Vec<usize>) {
    sel[now] = true;
    res.push(now);

    for &(nxt, _) in &g[now] {
        if nxt == prev {
            continue;
        }

        if sel[nxt] {
            continue;
        }

        dfs_scc2(nxt, now, g, sel, res);
    }
}