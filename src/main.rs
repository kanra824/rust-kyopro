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

    let mut t = vec![vec![]; tec.tree.n];
    // tree check
    for i in 0..tec.tree.n {
        for &(nxt, _) in &tec.tree.g[i] {
            t[i].push(nxt);
        }
    }

    let mut hld = HeavyLightDecomposition::new(tec.tree.n, t);
    
    for _ in 0..q {
        input! {
            (a, b, c): (Usize1, Usize1, Usize1),
        }

        let compa = tec.comp[a];
        let compb = tec.comp[b];
        let compc = tec.comp[c];

        let dist_ab = hld.depth[compa] + hld.depth[compb] - 2 * hld.depth[hld.lca(compa, compb)];
        let dist_bc = hld.depth[compb] + hld.depth[compc] - 2 * hld.depth[hld.lca(compb, compc)];
        let dist_ac = hld.depth[compa] + hld.depth[compc] - 2 * hld.depth[hld.lca(compa, compc)];

        if dist_ac == dist_ab + dist_bc {
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
}pub type Cost = i64;

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
// https://qiita.com/recuraki/items/cb888afdc107b64a4a6e
// verify: https://atcoder.jp/contests/abc294/submissions/70278032
// verify lca: https://onlinejudge.u-aizu.ac.jp/problems/GRL_5_C

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
        let mut res = HeavyLightDecomposition {
            n,
            g,
            prev: vec![usize::MAX; n],
            depth: vec![i64::MAX; n],
            child_cnt: vec![i64::MAX; n],
            node_to_hld: vec![usize::MAX; n], // 各 heavy-path に対してセグ木のクエリを実行したいときはこれで写した番号を使う
            hld_to_node: vec![],
            shallow: vec![usize::MAX; n],
        };

        res.dfs(0, usize::MAX, 0);
        res.hld_rec(0, 0);

        res
    }

    pub fn hld(&mut self, root: usize) -> Vec<usize> {
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