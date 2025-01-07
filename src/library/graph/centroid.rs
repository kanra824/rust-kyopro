use super::graph::*;

pub trait Centroid {
    fn centroid(&self) -> Vec<usize>;
}

impl Centroid for Graph {
    fn centroid(&self) -> Vec<usize> {
        let n = self.g.len();
        let mut centroid = vec![];
        let mut sz = vec![0; n];
        dfs(n, 0, usize::MAX, &self.g, &mut centroid, &mut sz);

        centroid
    }
}

fn dfs(
    n: usize,
    now: usize,
    prev: usize,
    g: &Vec<Vec<(usize, i64)>>,
    centroid: &mut Vec<usize>,
    sz: &mut Vec<usize>,
) {
    sz[now] = 1;
    let mut is_centroid = true;
    for &(nxt, _) in g[now].iter() {
        if nxt == prev {
            continue;
        }
        dfs(n, nxt, now, g, centroid, sz);
        sz[now] += sz[nxt];
        if sz[nxt] > n / 2 {
            is_centroid = false;
        }
    }
    if n - sz[now] > n / 2 {
        is_centroid = false;
    }
    if is_centroid {
        centroid.push(now);
    }
}
