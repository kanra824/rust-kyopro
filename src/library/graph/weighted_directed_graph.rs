type Cost = i32;

pub struct WeightedDirectedGraph {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
}

impl WeightedDirectedGraph {
    pub fn new(n: usize) -> Self {
        WeightedDirectedGraph {
            n,
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, cost: Cost) {
        self.g.get_mut(a).unwrap().push((b, cost))
    }
}

// ----- Test -----
#[test]
fn test_add_edge() {
    let mut graph = WeightedDirectedGraph::new(3);
    graph.add_edge(0, 1, 1);
    graph.add_edge(1, 2, 2);
    graph.add_edge(2, 0, 3);

    assert_eq!(graph.g, vec![vec![(1, 1)], vec![(2, 2)], vec![(0, 3)]]);
}
