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
