use crate::library::others::matrix::*;
use crate::library::number::mint::*;

fn to_mint(a: Vec<Vec<i64>>) -> Vec<Vec<Modint>> {
    let mut res = vec![vec![Modint::zero();a[0].len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            res[i][j] = Modint::new(a[i][j]);
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