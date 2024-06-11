use crate::library::graph::graph::*;
use crate::library::graph::dijkstra::*;

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