#![allow(unused)]
use std::collections::*;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Isize1;
use proconio::marker::Usize1;
use proconio::source::line::LineSource;
use std::cmp::max;
use std::cmp::min;
use std::fmt;
use std::io::BufReader;
use std::io::Read;
use std::io::Stdin;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::ops;
use std::str::FromStr;

pub type Cost = i64;

#[derive(Clone, Debug)]
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

    pub fn from_weighted_directed(n: usize, edges: Vec<(usize, usize, Cost)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v, c) in edges {
            graph.add_edge(u, v, c);
        }
        graph
    }

    pub fn from_directed(n: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v) in edges {
            graph.add_edge(u, v, 1);
        }
        graph
    }

    pub fn from_weighted(n: usize, edges: Vec<(usize, usize, Cost)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v, c) in edges {
            graph.add_edge(u, v, c);
            graph.add_edge(v, u, c);
        }
        graph
    }

    pub fn from_edges(n: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v) in edges {
            graph.add_edge(u, v, 1);
            graph.add_edge(v, u, 1);
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



pub trait Dijkstra {
    fn dijkstra(&self, start: usize) -> Vec<Option<Cost>>;
}

impl Dijkstra for Graph {
    fn dijkstra(&self, start: usize) -> Vec<Option<Cost>> {
        let mut res = vec![None; self.n];
        let mut pq = BinaryHeap::new();
        res[start] = Some(0);
        pq.push((0, start));
        while !pq.is_empty() {
            let (mut val, now) = pq.pop().unwrap();
            val = -val;
            if let Some(now_cost) = res[now] {
                if val > now_cost {
                    continue;
                }
            }
            for &(nxt, cost) in self.g[now].iter() {
                let nxt_cost = val + cost;
                match res[nxt] {
                    None => {
                        pq.push((-nxt_cost, nxt));
                        res[nxt] = Some(nxt_cost);
                    },
                    Some(val) => {
                        if val > nxt_cost {
                            pq.push((-nxt_cost, nxt));
                            res[nxt] = Some(nxt_cost);
                        }
                    }
                }
            }
        }
        res
    }
}


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

