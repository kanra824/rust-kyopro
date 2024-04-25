pub struct Graph {
    pub n: usize,
    pub g: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n: n,
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.g.get_mut(a).unwrap().push(b);
    }

    pub fn edges(&mut self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for i in 0..self.n {
            for &j in self.g[i].iter() {
                res.push((i, j));
            }
        }
        res
    }
}

// ----- Test -----
#[test]
fn test_add_edge() {
    let mut graph = Graph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    graph.add_edge(2, 1);

    assert_eq!(graph.g, vec![vec![1], vec![2], vec![0, 1]]);
}
