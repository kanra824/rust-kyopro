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