use crate::library::number::fft::*;
#[test]
fn test_convolution() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![2.0, 3.0, 4.0];
    let c = convolution(&a, &b);
    let expected = vec![2.0, 7.0, 16.0, 17.0, 12.0];

    let mut diff = 0.0;
    for i in 0..5 {
        diff += (c[i] - expected[i]).abs();
    }
    assert!(diff < 10e-8);
}

#[test]
fn test_butterfly() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![2.0, 3.0, 4.0];
    let c = convolution_butterfly(&a, &b);
    let expected = vec![2.0, 7.0, 16.0, 17.0, 12.0];

    eprintln!("{:?}", c);

    let mut diff = 0.0;
    for i in 0..5 {
        diff += (c[i] - expected[i]).abs();
    }
    assert!(diff < 10e-8);
}
