use crate::library::number::factorization::*;
#[test]
fn factorization_test() {
    let res = factorization(12);
    assert_eq!(res, vec![(2, 2), (3, 1)]);
    let res = factorization(11);
    assert_eq!(res, vec![(11, 1)]);
    let res = factorization(60);
    assert_eq!(res, vec![(2, 2), (3, 1), (5, 1)]);
}