pub type Cost = i64;

pub struct Graph {
    pub n: usize,
    pub g: Vec<Vec<(usize, Cost)>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            g: vec![Vec::new(); n],
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: Cost) {
        self.g.get_mut(a).unwrap().push((b, c))
    }

    pub fn edges(&mut self) -> Vec<(usize, usize, Cost)> {
        let mut res = vec![];
        for i in 0..self.n {
            for &(j, cost) in self.g[i].iter() {
                res.push((i, j, cost));
            }
        }
        res
    }
}

// ----- Test -----
#[cfg(test)]
mod tests {
    use super::*;
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
}