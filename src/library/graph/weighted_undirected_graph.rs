type Cost = i32;
struct WeightedUndirectedGraph {
    n: usize,
    g: Vec<Vec<(usize, Cost)>>,
}

impl WeightedUndirectedGraph {
    pub fn new(n: usize) -> Self {
        WeightedUndirectedGraph {
            n,
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, cost: Cost) {
        self.g.get_mut(a).unwrap().push((b, cost));
        self.g.get_mut(b).unwrap().push((a, cost));
    }
}

// ----- Test -----
#[test]
fn test_add_edge() {
    let mut graph = WeightedUndirectedGraph::new(3);
    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 2);

    assert_eq!(
        graph.g,
        vec![vec![(1, 1)], vec![(0, 1), (2, 2)], vec![(1, 2)]]
    );
}
