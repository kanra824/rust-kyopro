use crate::library::graph::graph::*;

#[test]
fn test_add_edge() {
    let mut graph = Graph::new(3);
    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 2);
    graph.add_edge(2, 0, 3);

    assert_eq!(graph.g, vec![vec![(1, 1)], vec![(2, 2)], vec![(0, 3)]]);
}

#[test]
fn test_edges() {
    let mut graph = Graph::new(3);
    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 2);
    graph.add_edge(2, 0, 3);
    assert_eq!(graph.edges(), vec![(0, 1, 1), (1, 2, 2), (2, 0, 3)]);
}