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
        s: Chars,
        m: usize,
        patterns_s: [Chars; m],
    }

    let mut patterns = vec![];
    for i in 0..m {
        let mut v = vec![];
        for &c in patterns_s[i].iter() {
            v.push(c as usize);
        }
        patterns.push(v);
    }
    let mut ahocora = AhoCorasick::new(&patterns);

    let mut query_s = vec![];
    for c in s {
        query_s.push(c as usize);
    }
    let res = ahocora.query(query_s);
    pr(res);
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

use std::collections::{HashMap, VecDeque};
// https://naoya-2.hatenadiary.org/entry/20090405/aho_corasick

#[derive(Clone, Debug)]
struct State {
    id: usize,
    next: HashMap<usize, usize>,
    is_pattern: bool,
}

impl State {
    fn new(id: usize) -> Self {
        State {
            id,
            next: HashMap::new(),
            is_pattern: false,
        }
    }

    fn has_key(&self, x: usize) -> bool {
        self.next.contains_key(&x)
    }
}

pub struct AhoCorasick {
    node: Vec<State>,
    failure: Vec<usize>,
}

impl AhoCorasick {
    pub fn new(patterns: &Vec<Vec<usize>>) -> Self {
        let node = vec![State::new(0)];
        let mut ahocora = AhoCorasick {
            node,
            failure: vec![],
        };
        ahocora.make_goto(patterns);
        ahocora.make_failure();
        ahocora
    }

    fn make_goto(&mut self, patterns: &Vec<Vec<usize>>) {
        // Trie 木をつくる
        for pattern in patterns {
            let mut cur = self.node[0].id;
            for &x in pattern {
                if !self.node[cur].has_key(x) {
                    let mut new_node = State::new(self.node.len());
                    self.node[cur].next.insert(x, new_node.id);
                    self.node.push(new_node);
                }
                cur = self.node[cur].next[&x];
            }
            self.node[cur].is_pattern = true;
        }
    }

    fn make_failure(&mut self) {
        let mut failure = vec![0; self.node.len()];
        let mut que = VecDeque::new();
        que.push_back(0);
        while !que.is_empty() {
            let s = que.pop_front().unwrap();
            for &x in self.node[s].next.keys() {
                let nxt = self.goto(s, x);
                if let Some(nxt) = nxt {
                    que.push_back(nxt);
                    if s != 0 {
                        let mut f = failure[s];
                        while self.goto(f, x).is_none() {
                            f = failure[f];
                        }
                        let res = self.goto(f, x);
                        if let Some(val) = res {
                            failure[nxt] = val;
                        }
                    }
                }
            }
        }
        self.failure = failure;
    }

    fn goto(&self, s: usize, x: usize) -> Option<usize> {
        if self.node[s].next.contains_key(&x) {
            Some(self.node[s].next[&x])
        } else {
            if s == 0 {
                Some(0)
            } else {
                None
            }
        }
    }

    fn query(&self, query: Vec<usize>) -> i64{
        pd(self.node.clone());
        pd(self.failure.clone());
        let mut now = 0;
        let mut cnt = 0;
        for i in 0..query.len() {
            while self.goto(now, query[i]).is_none() {
                now = self.failure[now];
                pd(now.clone());
                if self.node[now].is_pattern {
                    cnt += 1;
                }
            }

            now = self.goto(now, query[i]).unwrap();
            pd(now.clone());
            if self.node[now].is_pattern {
                cnt += 1;
            }
        }
        cnt
    }
}