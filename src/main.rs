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
        edges: [(Usize1, Usize1); m],
        q: usize,
    }

    let mut graph = Graph::from_edges(n, edges);
    let mut lowlink = graph.lowlink();
    let mut tec = graph.two_edge_connected_components(&lowlink);
    for _ in 0..q {
        input! {
            (a, b, c): (Usize1, Usize1, Usize1),
        }

        if tec.comp[a] == tec.comp[b] && tec.comp[b] == tec.comp[c] {
            pr("OK");
        } else {
            pr("NG");
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

#[derive(Clone, Debug)]
pub struct LowLinkData {
    ord: Vec<usize>,
    low: Vec<usize>,
    bridges: Vec<(usize, usize)>,
    articulations: Vec<usize>,
}

pub trait LowLink {
    fn lowlink(&self) -> LowLinkData;
}

impl LowLink for Graph {
    fn lowlink(&self) -> LowLinkData {
        let n = self.n;
        let mut visited = vec![false; n];
        let mut ord = vec![usize::MAX; n];
        let mut low = vec![usize::MAX; n];
        let mut order = 0;

        let mut lowlink = LowLinkData {
            ord,
            low,
            bridges: vec![],
            articulations: vec![]
        };

        for i in 0..n {
            if !visited[i] {
                build_lowlink(i, usize::MAX, 0, &self, &mut visited, &mut order, &mut lowlink);
            }
        }
        lowlink
    }
}

fn build_lowlink(now: usize, prev: usize, prev_cnt: usize, graph: &Graph, visited: &mut Vec<bool>, order: &mut usize, lowlink: &mut LowLinkData) {
    if visited[now] {
        return;
    }
    visited[now] = true;
    lowlink.ord[now] = *order;
    lowlink.low[now] = *order;
    *order += 1;
    let mut is_articulation = false;
    let mut cnt = 0;

    let mut prev_cnt_mp = BTreeMap::new();
    for &(nxt, _) in &graph.g[now] {
        let val = prev_cnt_mp.entry(nxt).or_insert(0);
        *val += 1;
    }

    for &(nxt, _) in &graph.g[now] {
        if !visited[nxt] {
            cnt += 1;
            build_lowlink(nxt, now, prev_cnt_mp[&nxt], graph, visited, order, lowlink);
            lowlink.low[now] = lowlink.low[now].min(lowlink.low[nxt]);
            is_articulation = is_articulation || prev != usize::MAX && lowlink.low[nxt] >= lowlink.ord[now];
            if lowlink.ord[now] < lowlink.low[nxt] {
                lowlink.bridges.push((now.min(nxt), now.max(nxt)));
            }
        } else if prev_cnt >= 2 || nxt != prev {
            lowlink.low[now] = lowlink.low[now].min(lowlink.ord[nxt]);
        }
    }
    is_articulation = is_articulation || prev == usize::MAX && cnt >= 2;
    if is_articulation {
        lowlink.articulations.push(now);
    }
}

pub trait BiConnectedComponents: LowLink {
    fn biconnected_components(&self, lowlink: &LowLinkData) -> Vec<Vec<usize>>;
}

impl BiConnectedComponents for Graph {
    fn biconnected_components(&self, lowlink: &LowLinkData) -> Vec<Vec<usize>> {
        let n = self.n;
        let mut used = vec![false; n];
        let mut tmp = vec![];
        let mut bc = vec![];

        for i in 0..n {
            if used[i] {
                continue;
            }
            build_biconnected_components(i, usize::MAX, &self, &lowlink, &mut used, &mut tmp, &mut bc);
        }
        bc
    }
}

fn build_biconnected_components(now: usize, prev: usize, graph: &Graph, lowlink: &LowLinkData, used: &mut Vec<bool>, tmp: &mut Vec<usize>, bc: &mut Vec<Vec<usize>>) {
    used[now] = true;
    let mut tmp_bool = false;
    for &(nxt, _) in &graph.g[now] {
        if nxt == prev {
            let nowtmp = tmp_bool;
            tmp_bool = true;
            if !nowtmp {
                continue;
            }
        }
        if !used[nxt] || lowlink.ord[nxt] < lowlink.ord[now] {
            tmp.push((nxt))
        }

        if !used[nxt] {
            build_biconnected_components(nxt, now, graph, lowlink, used, tmp, bc);
            if lowlink.low[nxt] >= lowlink.ord[now] {
                bc.push(vec![]);
                loop {
                    let e = tmp.pop().unwrap();
                    let sz = bc.len();
                    bc[sz-1].push(e);
                    if e == nxt {
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct TwoEdgeConnectedComponentsData {
    comp: Vec<usize>, // 各頂点が属する二重辺連結成分の頂点番号
    tree: Graph, // 縮約後の頂点からなる森
    group: Vec<Vec<usize>>, // 各二重辺連結成分について、それに属する頂点
}

trait TwoEdgeConnectedComponents: LowLink {
    fn two_edge_connected_components(&self, lowlink: &LowLinkData) -> TwoEdgeConnectedComponentsData;
}

impl TwoEdgeConnectedComponents for Graph {
    fn two_edge_connected_components(&self, lowlink: &LowLinkData) -> TwoEdgeConnectedComponentsData {
        // verify: https://judge.yosupo.jp/submission/326904
        let n = self.n;
        let mut comp = vec![usize::MAX; n];
        let mut k = 0;
        let mut tec = TwoEdgeConnectedComponentsData {
            comp,
            tree: Graph::new(1),
            group: vec![],
        };
        for i in 0..n {
            if tec.comp[i] == usize::MAX {
                build_two_edge_connected_components(i, usize::MAX, &mut k, &self, lowlink, &mut tec);
            }
        }
        tec.group = vec![vec![]; k];
        for i in 0..n {
            tec.group[tec.comp[i]].push(i);
        }
        tec.tree = Graph::new(k);
        for &(u, v) in &lowlink.bridges {
            tec.tree.add_edge(tec.comp[u], tec.comp[v], 1);
            tec.tree.add_edge(tec.comp[v], tec.comp[u], 1);
        }
        tec
    }
}

fn build_two_edge_connected_components(now: usize, prev: usize, k: &mut usize, graph: &Graph, lowlink: &LowLinkData, tec: &mut TwoEdgeConnectedComponentsData) {
    if prev != usize::MAX && lowlink.ord[prev] >= lowlink.low[now] {
        tec.comp[now] = tec.comp[prev];
    } else {
        tec.comp[now] = *k;
        *k += 1;
    }

    for &(nxt, _) in &graph.g[now] {
        if tec.comp[nxt] == usize::MAX {
            build_two_edge_connected_components(nxt, now, k, graph, lowlink, tec);
        }
    }
}
