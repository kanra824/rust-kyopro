#[derive(Debug, Clone)]
pub struct Matrix {
    pub h: usize,
    pub w: usize,
    pub a: Vec<Vec<i64>>,
}

impl Matrix {
    pub fn new(a: Vec<Vec<i64>>) -> Self {
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
            a: vec![vec![0; w]; h],
        }
    }

    pub fn e(n: usize) -> Self {
        let mut a = vec![vec![0;n];n];
        for i in 0..n {
            a[i][i] = 1;
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
        let mut res = vec![vec![0; b.w]; self.h];
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
        let mut res = vec![vec![0; b.w]; self.h];
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
}
