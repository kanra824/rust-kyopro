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

#[test]
fn test_convolution_large() {
    let n = 100000;
    let mut a = vec![Modint::new(0); n];
    let mut b = vec![Modint::new(0); n];
    
    // aとbに値を設定
    for i in 0..n {
        a[i] = Modint::new((i % 1000) as i64);
        b[i] = Modint::new(((i + 1) % 1000) as i64);
    }
    
    // Butterfly NTTでたたみ込みを計算
    let c = convolution_butterfly(a.clone(), b.clone());
    
    // 結果の長さを確認
    assert_eq!(c.len(), 2 * n - 1);
    
    // いくつかの要素をナイーブな方法で検証
    let verify_indices = vec![0, 1, 100, 1000, 10000, 50000, 100000, 150000, c.len() - 1];
    
    for &idx in &verify_indices {
        if idx >= c.len() {
            continue;
        }
        
        let mut expected = Modint::new(0);
        let start = if idx >= n - 1 { idx - (n - 1) } else { 0 };
        let end = (idx + 1).min(n);
        
        for i in start..end {
            if idx >= i && idx - i < n {
                expected = expected + a[i] * b[idx - i];
            }
        }
        
        assert_eq!(c[idx], expected, "Mismatch at index {}", idx);
    }
    
    eprintln!("Test passed for n={}, result length={}", n, c.len());
}