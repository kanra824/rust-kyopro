use super::graph::*;
use std::collections::BTreeMap;

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
                build_lowlink(i, usize::MAX, &self, &mut visited, &mut order, &mut lowlink);
            }
        }
        lowlink
    }
}

fn build_lowlink(now: usize, prev: usize, graph: &Graph, visited: &mut Vec<bool>, order: &mut usize, lowlink: &mut LowLinkData) {
    if visited[now] {
        return;
    }
    visited[now] = true;
    lowlink.ord[now] = *order;
    lowlink.low[now] = *order;
    *order += 1;
    let mut is_articulation = false;
    let mut tmp_bool = false;
    let mut cnt = 0;

    // let mut prev_cnt_mp = BTreeMap::new();
    // for &(nxt, _) in &graph.g[now] {
    //     let val = prev_cnt_mp.entry(nxt).or_insert(0);
    //     *val += 1;
    // }

    // for (&nxt, &prev_cnt) in &prev_cnt_mp {

    for &(nxt, _) in &graph.g[now] {
        if nxt == prev {
            let nowtmp = tmp_bool;
            tmp_bool = true;
            if !nowtmp {
                continue;
            }
        }

        if !visited[nxt] {
            cnt += 1;
            build_lowlink(nxt, now, graph, visited, order, lowlink);
            lowlink.low[now] = lowlink.low[now].min(lowlink.low[nxt]);
            is_articulation = is_articulation || prev != usize::MAX && lowlink.low[nxt] >= lowlink.ord[now];
            if lowlink.ord[now] < lowlink.low[nxt] {
                lowlink.bridges.push((now.min(nxt), now.max(nxt)));
            }
        // } else if prev_cnt >= 2 || nxt != prev {
        } else {
            lowlink.low[now] = lowlink.low[now].min(lowlink.ord[nxt]);
        }
    }
    is_articulation = is_articulation || prev == usize::MAX && cnt >= 2;
    if is_articulation {
        lowlink.articulations.push(now);
    }
}

pub trait BiConnectedComponents: LowLink {
    fn biconnected_components(&self, lowlink: &LowLinkData) -> Vec<Vec<(usize, usize)>>;
}

impl BiConnectedComponents for Graph {
    // 多重辺があるときに動作しないので、事前に多重辺を取り除いておく
    fn biconnected_components(&self, lowlink: &LowLinkData) -> Vec<Vec<(usize, usize)>> {
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

fn build_biconnected_components(now: usize, prev: usize, graph: &Graph, lowlink: &LowLinkData, used: &mut Vec<bool>, tmp: &mut Vec<(usize, usize)>, bc: &mut Vec<Vec<(usize, usize)>>) {
    used[now] = true;
    let mut tmp_bool = false;
    for &(nxt, _) in &graph.g[now] {
        if nxt == prev && !tmp_bool {
            tmp_bool = true;
            continue;
        }
        if !used[nxt] || lowlink.ord[nxt] < lowlink.ord[now] {
            // pd(&lowlink);
            // pd(&used);
            // println!("{} {}", now, nxt);
            tmp.push((now.min(nxt), now.max(nxt)));
        }

        if !used[nxt] {
            build_biconnected_components(nxt, now, graph, lowlink, used, tmp, bc);
            if lowlink.low[nxt] >= lowlink.ord[now] {
                bc.push(vec![]);
                loop {
                    let e = tmp.pop().unwrap();
                    let sz = bc.len();
                    bc[sz-1].push(e);
                    if e.0 == now.min(nxt) && e.1 == now.max(nxt) {
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
