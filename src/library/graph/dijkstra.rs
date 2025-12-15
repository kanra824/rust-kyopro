// use crate::library::graph::graph::*;
use super::graph::*;

use std::collections::BinaryHeap;

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
