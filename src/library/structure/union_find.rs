//use proconio::input;
//use proconio::marker::{Usize1, Isize1};
#[derive(Clone)]
pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut par = Vec::new();
        for i in 0..n {
            par.push(i);
        }
        let sz = vec![1; n];
        UnionFind { n: n, par, sz }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    pub fn get_sz(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.sz[x]
    }

     pub fn count(&mut self) -> usize {
         let mut st = std::collections::HashSet::new(); 
         for i in 0..self.n {
            st.insert(self.root(i));
         }
         st.len()
     }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return;
        }

        if self.sz[x] > self.sz[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.sz[y] = self.sz[x] + self.sz[y];
        self.sz[x] = self.sz[y];
        self.par[x] = y;
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}
