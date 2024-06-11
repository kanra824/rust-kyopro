use crate::library::number::ntt::*;
use crate::library::number::mint::Modint;
#[test]
fn test_convolution() {
    let a = vec![1, 2, 3];
    let b = vec![2, 3, 4];

    let a = a.iter().map(|&x| Modint::new(x)).collect();
    let b = b.iter().map(|&x| Modint::new(x)).collect();

    let c = convolution(a, b);
    let expected = vec![2, 7, 16, 17, 12];
    let expected: Vec<Modint> = expected.iter().map(|&x| Modint::new(x)).collect();

    assert_eq!(c, expected);
}