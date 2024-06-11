use crate::library::number::sieve::*;
#[test]
fn test_sieve() {
    let res = sieve(100);
    assert!(!res[0]);
    assert!(!res[1]);
    assert!(res[2]);
    assert!(res[3]);
    assert!(res[5]);
    assert!(!res[100]);
}
