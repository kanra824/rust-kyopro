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

    pub fn from_vec(g: Vec<Vec<(usize, Cost)>>) -> Self {
        Graph {
            n: g.len(),
            g
        }
    }

    pub fn from_unweighted_vec(g_in: Vec<Vec<usize>>) -> Self {
        let n = g_in.len();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            for &j in &g_in[i] {
                g[i].push((g_in[i][j], 1));
            }
        }
        Graph {
            n,
            g
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
