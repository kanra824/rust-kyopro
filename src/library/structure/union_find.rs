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

#[cfg(test)]
mod test {
    use std::collections::btree_set::Union;

    use super::*;
    fn get_uf() -> UnionFind {
        let n = 10;
        let mut uf = UnionFind::new(n);

        uf.unite(1, 2);
        uf.unite(3, 4);
        uf.unite(2, 3);
        uf.unite(5, 6);
        uf.unite(7, 9);
        uf.unite(6, 7);
        uf
    }

    #[test]
    fn test_uf_get_sz() {
        let mut uf = get_uf();
        assert_eq!(uf.get_sz(1), 4);
        assert_eq!(uf.get_sz(5), 4);
        assert_eq!(uf.get_sz(8), 1);
    }

    #[test]
    fn test_uf_count() {
        let mut uf = get_uf();
        assert_eq!(uf.count(), 4);
    }

    #[test]
    fn test_uf_unite() {
        let mut uf = get_uf();

        assert!(uf.same(1, 2));
        assert!(!uf.same(1, 5));
        assert!(uf.same(6, 7));
        assert!(uf.same(1, 1));
    }

}