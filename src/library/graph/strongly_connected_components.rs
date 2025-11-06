use super::graph::*;

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