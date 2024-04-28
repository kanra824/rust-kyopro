use super::graph::Graph;
use std::{cmp::Reverse, collections::BinaryHeap};

trait Dijkstra {
    type Cost;
    fn dijkstra(&self, start: usize) -> Vec<Option<Self::Cost>>;
}

impl Dijkstra for Graph<i64> {
    type Cost = i64;
    fn dijkstra(&self, start: usize) -> Vec<Option<Self::Cost>> {
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

#[test]
fn dijkstra_test() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 2);
    graph.add_edge(2, 3, 3);
    graph.add_edge(3, 4, 7);

    let d = graph.dijkstra(0);

    assert_eq!(d, vec![Some(0), Some(1), Some(3), Some(6), Some(13)]);
}