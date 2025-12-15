#![allow(unused)]

mod library;
use library::graph::graph::*;
use library::graph::dijkstra::*;

fn main() {
    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut g = Graph::new(h * w + 26);
    let ofs = h * w;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '#' {
                continue;
            }
            for (nr, nc) in adj_pos(h, w, r, c) {
                if s[nr][nc] == '#' {
                    continue;
                }
                g.add_edge(r * w + c, nr * w + nc, 2);
            }

            if 'a' <= s[r][c] && s[r][c] <= 'z' {
                let val = s[r][c] as usize - 'a' as usize;
                g.add_edge(r * w + c, ofs + val, 1);
                g.add_edge(ofs + val, r * w + c, 1);
            }
        }
    }

    let d = g.dijkstra(0);

    match d[h * w - 1] {
        None => {
            pr(-1);
        },
        Some(val) => {
            pr(val / 2);
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
