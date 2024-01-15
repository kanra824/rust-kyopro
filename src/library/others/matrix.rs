use crate::library::number::mint::Modint;
const MOD: i64 = 998244353;
type Mint = Modint<MOD>;

// 正方行列
#[derive(Debug, Clone)]
struct Matrix {
    h: usize,
    w: usize,
    a: Vec<Vec<Mint>>,
}

impl Matrix {
    fn new(a: Vec<Vec<Mint>>) -> Self {
        assert!(!a.is_empty());
        assert!(a.iter().all(|v| v.len() == a[0].len()));
        Matrix {
            h: a.len(),
            w: a[0].len(),
            a,
        }
    }

    fn zero(h: usize, w: usize) -> Self {
        Matrix {
            h,
            w,
            a: vec![vec![Mint::zero(); w]; h],
        }
    }

    fn e(n: usize) -> Self {
        let mut a = vec![vec![Mint::zero();n];n];
        for i in 0..n {
            a[i][i] = Mint::new(1);
        }
        Matrix {
            h: n,
            w: n,
            a,
        }
    }

    fn add(&self, b: &Matrix) -> Matrix {
        debug_assert_eq!(self.h, b.h);
        debug_assert_eq!(self.w, b.w);
        let mut res = vec![vec![Mint::zero(); b.w]; self.h];
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

    fn mul(&self, b: &Matrix) -> Matrix {
        debug_assert_eq!(self.w, b.h);
        let mut res = vec![vec![Mint::zero(); b.w]; self.h];
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

    fn pow(&self, mut k: i64) -> Matrix {
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
            if a.a[i][col] != Mint::zero() {
                return i;
            }
        }
        return usize::MAX;
    }

    fn sweep(mut a: Matrix, rank: usize, col: usize, pivot: usize) -> Matrix {
        let tmp = a.a[pivot].clone();
        a.a[pivot] = a.a[rank].clone();
        a.a[rank] = tmp;
        
        let div = a.a[rank][col].inv();
        for j in 0..col {
            assert_eq!(a.a[rank][j], Mint::zero());
        }
        for j in col..a.w {
            a.a[rank][j] *= div;
        }

        for i in 0..a.h {
            if i == rank {
                continue;
            }
            for j in 0..col {
                assert_eq!(a.a[rank][j], Mint::zero());
            }
            let tmp = a.a[i][col];
            for j in col..a.w {
                let sub = a.a[rank][j] * tmp;
                a.a[i][j] -= sub;
            }
        }
        a
    }

    fn determinant(&self) -> Mint {
        let mut a = self.clone();
        let mut rank = 0;
        let mut res = Mint::new(1);
        for col in 0..self.w {
            let pivot = Self::pivot(&a, rank, col);
            if pivot == usize::MAX {
                return Mint::zero();
            }
            res *= a.a[pivot][rank];
            a = Self::sweep(a, rank, col, pivot);
            rank += 1;
        }
        res
    }

}


#[cfg(test)]
mod test {
    use super::*;

    fn to_mint(a: Vec<Vec<i64>>) -> Vec<Vec<Mint>> {
        let mut res = vec![vec![Mint::zero();a[0].len()]; a.len()];
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                res[i][j] = Mint::new(a[i][j]);
            }
        }
        res
    }

    #[test]
    fn test_mul() {
        let mut a = Matrix::new(to_mint(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]));
        assert_eq!(
            a.mul(&a).a,
            to_mint(vec![vec![15, 18, 21], vec![42, 54, 66], vec![69, 90, 111]]),
        );
    }

    #[test]
    fn test_add() {
        let mut a = Matrix::new(to_mint(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]));
        assert_eq!(
            a.add(&a).a,
            to_mint(vec![vec![0, 2, 4], vec![6, 8, 10], vec![12, 14, 16]]),
        );
    }

    #[test]
    fn test_pow() {
        let mut a = Matrix::new(to_mint(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]));
        assert_eq!(
            a.pow(5).a,
            to_mint(vec![vec![32400, 41796, 51192], vec![99468, 128304, 157140], vec![166536, 214812, 263088]]),
        );
    }

    #[test]
    fn test_determinant() {
        let mut a = Matrix::new(to_mint(vec![vec![1, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]));
        assert_eq!(a.determinant(), Modint::new(MOD - 3));
    }
}
