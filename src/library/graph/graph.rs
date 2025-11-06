pub type Cost = i64;

pub struct Graph {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
    pub edges: Vec<(usize, usize, Cost)>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            g: vec![Vec::new(); n],
            edges: vec![],
        }
    }

    pub fn from_weighted_directed(n: usize, edges: Vec<(usize, usize, Cost)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v, c) in edges {
            graph.add_edge(u, v, c);
        }
        graph
    }

    pub fn from_directed(n: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v) in edges {
            graph.add_edge(u, v, 1);
        }
        graph
    }

    pub fn from_weighted(n: usize, edges: Vec<(usize, usize, Cost)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v, c) in edges {
            graph.add_edge(u, v, c);
            graph.add_edge(v, u, c);
        }
        graph
    }

    pub fn from_edges(n: usize, edges: Vec<(usize, usize)>) -> Self {
        let mut graph = Graph::new(n);
        for (u, v) in edges {
            graph.add_edge(u, v, 1);
            graph.add_edge(v, u, 1);
        }
        graph
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: Cost) {
        self.g.get_mut(a).unwrap().push((b, c));
        self.edges.push((a, b, c));
    }

    pub fn edges(&self) -> Vec<(usize, usize, Cost)> {
        self.edges.clone()
    }

    pub fn rev(&self) -> Self {
        let mut revg = Graph::new(self.n);
        for &(u, v, c) in &self.edges {
            revg.add_edge(v, u, c);
        }
        revg
    }
}
