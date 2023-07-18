struct UndirectedGraph {
    n: usize,
    g: Vec<Vec<usize>>,
}

impl UndirectedGraph {
    pub fn new(n: usize) -> Self {
        UndirectedGraph {
            n: n,
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.g.get_mut(a).unwrap().push(b);
        self.g.get_mut(b).unwrap().push(a);
    }
}

// ----- Test -----
#[test]
fn test_add_edge() {
    let mut graph = UndirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);

    assert_eq!(graph.g, vec![vec![1], vec![0, 2], vec![1]]);
}
