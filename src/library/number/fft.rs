use num_complex::Complex64;

use std::f64::consts::PI;

// inv = 1.0 なら FFT
// inv = -1.0 なら IFFT
fn fft(a: &Vec<Complex64>, inv: f64) -> Vec<Complex64> {
    let n = a.len();
    let mut res = vec![];
    if n == 1 {
        return a.clone();
    }

    let mut even = vec![];
    let mut odd = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            even.push(a[i]);
        } else {
            odd.push(a[i]);
        }
    }

    let d_even = fft(&even, inv);
    let d_odd = fft(&odd, inv);

    let dn = n as f64;

    let zeta = Complex64::new((2.0 * PI * inv / dn).cos(), (2.0 * PI * inv / dn).sin());

    let mut now = Complex64::new(1.0, 0.0);
    for i in 0..n {
        res.push(d_even[i % (n / 2)] + now * d_odd[i % (n / 2)]);
        now *= zeta;
    }

    res
}

pub fn convolution(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut a: Vec<Complex64> = a.iter().map(|&x| Complex64::new(x, 0.0)).collect();
    let mut b: Vec<Complex64> = b.iter().map(|&x| Complex64::new(x, 0.0)).collect();
    let sza = a.len();
    let szb = b.len();
    let mut n = 1;
    while n <= sza + szb - 1 {
        n *= 2;
    }

    let zero = Complex64::new(0.0, 0.0);
    while a.len() < n {
        a.push(zero);
    }
    while b.len() < n {
        b.push(zero);
    }

    let da = fft(&a, 1.0);
    let db = fft(&b, 1.0);

    let mut dc = vec![];
    for i in 0..n {
        dc.push(da[i] * db[i]);
    }

    let c = fft(&dc, -1.0);

    let mut res = vec![];
    let dn = n as f64;
    for i in 0..n {
        res.push(c[i].re / dn);
    }
    res
}

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
