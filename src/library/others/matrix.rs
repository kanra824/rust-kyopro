use crate::library::number::mint::*;

type Mint = Modint;

// 正方行列
#[derive(Debug, Clone)]
pub struct Matrix {
    pub h: usize,
    pub w: usize,
    pub a: Vec<Vec<Mint>>,
}

impl Matrix {
    pub fn new(a: Vec<Vec<Mint>>) -> Self {
        assert!(!a.is_empty());
        assert!(a.iter().all(|v| v.len() == a[0].len()));
        Matrix {
            h: a.len(),
            w: a[0].len(),
            a,
        }
    }

    pub fn zero(h: usize, w: usize) -> Self {
        Matrix {
            h,
            w,
            a: vec![vec![Mint::new(0); w]; h],
        }
    }

    pub fn e(n: usize) -> Self {
        let mut a = vec![vec![Mint::new(0);n];n];
        for i in 0..n {
            a[i][i] = Mint::new(1);
        }
        Matrix {
            h: n,
            w: n,
            a,
        }
    }

    pub fn add(&self, b: &Matrix) -> Matrix {
        debug_assert_eq!(self.h, b.h);
        debug_assert_eq!(self.w, b.w);
        let mut res = vec![vec![Mint::new(0); b.w]; self.h];
        for i in 0..self.h {
            for j in 0..self.w {
                res[i][j] = self.a[i][j] + b.a[i][j];
            }
        }
        Matrix {
            h: res.len(),
            w: res[0].len(),
            a: res,
        }
    }

    pub fn mul(&self, b: &Matrix) -> Matrix {
        debug_assert_eq!(self.w, b.h);
        let mut res = vec![vec![Mint::new(0); b.w]; self.h];
        for i in 0..self.h {
            for j in 0..b.w {
                for k in 0..self.w {
                    res[i][j] = res[i][j] + self.a[i][k] * b.a[k][j];
                }
            }
        }
        Matrix {
            h: res.len(),
            w: res[0].len(),
            a: res,
        }
    }

    pub fn pow(&self, mut k: i64) -> Matrix {
        assert_eq!(self.h, self.w);
        let mut res = Matrix::e(self.h);
        let mut now = self.clone();
        while k > 0 {
            if k & 1 == 1 {
                res = res.mul(&now);
            }
            now = now.mul(&now);
            k >>= 1;
        }
        res
    }

    fn pivot(a: &Matrix, rank: usize, col: usize) -> usize {
        for i in rank..a.h {
            if a.a[i][col] != Mint::new(0) {
                return i;
            }
        }
        return usize::MAX;
    }

    pub fn sweep(mut a: Matrix, rank: usize, col: usize, pivot: usize) -> Matrix {
        let tmp = a.a[pivot].clone();
        a.a[pivot] = a.a[rank].clone();
        a.a[rank] = tmp;
        
        let div = a.a[rank][col].inv();
        for j in 0..col {
            assert_eq!(a.a[rank][j], Mint::new(0));
        }
        for j in col..a.w {
            a.a[rank][j] *= div;
        }

        for i in 0..a.h {
            if i == rank {
                continue;
            }
            for j in 0..col {
                assert_eq!(a.a[rank][j], Mint::new(0));
            }
            let tmp = a.a[i][col];
            for j in col..a.w {
                let sub = a.a[rank][j] * tmp;
                a.a[i][j] -= sub;
            }
        }
        a
    }

    pub fn determinant(&self) -> Mint {
        let mut a = self.clone();
        let mut rank = 0;
        let mut res = Mint::new(1);
        for col in 0..self.w {
            let pivot = Self::pivot(&a, rank, col);
            if pivot == usize::MAX {
                return Mint::new(0);
            }
            res *= a.a[pivot][rank];
            a = Self::sweep(a, rank, col, pivot);
            rank += 1;
        }
        res
    }

}
