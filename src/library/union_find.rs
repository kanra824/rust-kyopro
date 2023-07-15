//use proconio::input;
//use proconio::marker::{Usize1, Isize1};

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
        UnionFind {
            _n: n,
            par,
            sz,
        }
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

// fn main() {
//     input! {
//         n: usize,
//         q: usize,
//         query: [(i32, usize, usize); q],
//     }
// 
//     let mut uf = UnionFind::new(n);
//     for (com, x, y) in query {
//         if com == 0 {
//             uf.unite(x, y);
//         } else {
//             if uf.same(x, y) {
//                 println!("Yes");
//             } else {
//                 println!("No");
//             }
//         }
//     }
// }