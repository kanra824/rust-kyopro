use super::graph::*;

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
    let mut cnt = 0;
    for &(nxt, _) in &graph.g[now] {
        if !visited[nxt] {
            cnt += 1;
            build_lowlink(nxt, now, graph, visited, order, lowlink);
            lowlink.low[now] = lowlink.low[now].min(lowlink.low[nxt]);
            is_articulation = is_articulation || prev != usize::MAX && lowlink.low[nxt] >= lowlink.ord[now];
            if lowlink.ord[now] < lowlink.low[nxt] {
                lowlink.bridges.push((now.min(nxt), now.max(nxt)));
            }
        } else if nxt != prev {
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

