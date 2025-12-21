use crate::library::fps::fps::*;
use crate::library::number::mint::Modint;
use rand::{rng, Rng};

fn create_random_fps(n: usize, m: i64) -> Fps {
    let mut rng = rng();
    let mut a = vec![];
    for _ in 0..n {
        a.push(rng.random_range(0..=m));
    }
    Fps::from_i64_vec(a)
}

#[test]
fn test_fps_inv() {
    // yosupo judge: https://judge.yosupo.jp/problem/inv_of_formal_power_series

    let n = 500000;
    let m = 1000000000;
    let f = create_random_fps(n, m);
    let start = std::time::Instant::now();
    let g = f.inv(n);
    let time = start.elapsed().as_millis();
    assert!(time <= 5000, "fps inv time exceeded: {} ms", time);
    let h = &f * &g;
    for i in 0..n {
        if i == 0 {
            assert_eq!(h.a[i], Modint::new(1));
        } else {
            assert_eq!(h.a[i], Modint::new(0));
        }
    }
}

#[test]
fn test_fps_differential() {
    let n = 500000;
    let m = 1000000000;
    let f = create_random_fps(n, m);
    let start = std::time::Instant::now();
    let g = f.differential(n);
    let time = start.elapsed().as_millis();
    assert!(time <= 100, "fps differential time exceeded: {} ms", time);
    for i in 0..n-1 {
        assert_eq!(g.a[i], f.a[i+1] * Modint::new(i as i64 + 1));
    }
}

#[test]
fn test_fps_integral() {
    let n = 500000;
    let m = 1000000000;
    let f = create_random_fps(n, m);
    let start = std::time::Instant::now();
    let g = f.integral(n);
    let time = start.elapsed().as_millis();
    assert!(time <= 100, "fps integral time exceeded: {} ms", time);
    assert_eq!(g.a[0], Modint::new(0));
    for i in 1..n {
        assert_eq!(g.a[i], f.a[i-1] * Modint::new(i as i64).inv());
    }
}
