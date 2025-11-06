use crate::library::graph::graph::Graph;
use crate::library::graph::strongly_connected_components::*;
use proconio::input;
use crate::*;

// verify: https://atcoder.jp/contests/practice2/submissions/70723664
fn test_strongly_connected_components() {

    input! {
        // from &mut source,
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut g = Graph::from_unweighted_edges(n, edges);
    let mut scc = g.strongly_connected_components();

    pr(scc.len());
    for v in scc {
        print!("{}", v.len());
        for val in v {
            print!(" {}", val);
        }
        println!();
    }
}