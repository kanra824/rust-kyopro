use crate::library::graph::graph::*;
use crate::library::graph::centroid::*;

#[test]
fn test_centroid1() {
    let mut g = Graph::new(5);
    g.add_edge(0, 1, 1);
    g.add_edge(1, 2, 1);
    g.add_edge(2, 3, 1);
    g.add_edge(3, 4, 1);
    let mut res = g.centroid();
    res.sort();
    assert_eq!(res, vec![2]);
}

#[test]
fn test_centroid2() {
    let mut g = Graph::new(4);
    g.add_edge(0, 1, 1);
    g.add_edge(1, 2, 1);
    g.add_edge(2, 3, 1);
    let mut res = g.centroid();
    res.sort();
    assert_eq!(res, vec![1, 2]);
}