use crate::library::number::pow::*;

#[test]
fn pow_test() {
    assert_eq!(pow(2, 3), 8);
    assert_eq!(pow(3, 4), 81);
    assert_eq!(pow(10, 4), 10000);
    assert_eq!(pow(5, 3), 125);
}
