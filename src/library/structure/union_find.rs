use proconio::input;
use crate::library::judge::judge;

pub struct UnionFind {
    _n: usize,
    par: Vec<usize>,
    sz: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut par = Vec::new();
        for i in 0..n {
            par.push(i);
        }
        let sz = vec![1; n];
        UnionFind { _n: n, par, sz }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
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

#[test]
fn test_union_find() {
    let solver = || {
        input! {
            n: usize,
            q: usize,
            queries: [(usize, usize, usize); q],
        }

        let mut uf = UnionFind::new(n);
        for (t, u, v) in queries {
            if t == 0 {
                uf.unite(u, v);
            } else {
                println!("{}", if uf.same(u, v) { 1 } else { 0 });
            }
        }
    };

    judge("DSL_1_A", solver).unwrap();

}