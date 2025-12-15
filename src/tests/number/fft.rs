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

#[test]
fn test_butterfly_large() {
    let n = 100000;
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    
    // aとbに値を設定
    for i in 0..n {
        a[i] = (i % 10) as f64;
        b[i] = ((i + 1) % 10) as f64;
    }
    
    // convolution_butterfly でたたみ込みを計算
    let c = convolution_butterfly(&a, &b);
    
    // ナイーブな方法で検証用の結果を計算（最初と最後のいくつかの要素のみ）
    let verify_count = 10;
    for idx in 0..verify_count {
        let mut expected = 0.0;
        for i in 0..n {
            if idx >= i && idx - i < n {
                expected += a[i] * b[idx - i];
            }
        }
        let diff = (c[idx] - expected).abs();
        assert!(diff < 1e-6, "Mismatch at index {}: {} vs {}", idx, c[idx], expected);
    }
    
    // 末尾付近も検証
    for idx in (c.len() - verify_count)..c.len() {
        let mut expected = 0.0;
        for i in 0..n {
            if idx >= i && idx - i < n {
                expected += a[i] * b[idx - i];
            }
        }
        let diff = (c[idx] - expected).abs();
        assert!(diff < 1e-6, "Mismatch at index {}: {} vs {}", idx, c[idx], expected);
    }
    
    eprintln!("Test passed for n={}, result length={}", n, c.len());
}
