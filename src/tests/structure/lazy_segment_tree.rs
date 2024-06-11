use crate::library::structure::lazy_segment_tree::*;
use rand::prelude::*;

#[test]
fn test_add_sum() {
    let n = 100;
    let mut st = LazySegmentTree::new(
        n,
        |a, b| a + b,
        |a, b, len| a + b * len as i64,
        |a, b| a + b,
        0,
        0,
    );
    let mut v: Vec<i64> = (0..n as i64).collect();
    st.build(&v);

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let t = rng.gen_bool(1.0 / 3.0);
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l + 1..=n);
        if t {
            let x = rng.gen_range(-100000..100000);
            for i in l..r {
                v[i] += x;
            }
            st.update(l, r, x);
        } else {
            let mut su = 0;
            for i in l..r {
                su += v[i];
            }
            assert_eq!(su, st.query(l, r));
        }
        println!("{:?}", v);
    }
}

#[test]
fn test_update_sum() {
    let n = 2000;
    let q = 2000;
    let mut st = LazySegmentTree::new(
        n,
        |a, b| a + b,
        |a, b, len| b * len as i64,
        |a, b| b,
        0,
        i64::MAX,
    );
    let mut v: Vec<i64> = (0..n as i64).collect();
    st.build(&v);

    let mut rng = rand::thread_rng();
    for _ in 0..q {
        let t = rng.gen_bool(1.0 / 2.0);
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l + 1..=n);
        if t {
            let x = rng.gen_range(-100000..100000);
            for i in l..r {
                v[i] = x;
            }
            st.update(l, r, x);
            for i in l..r {
                assert_eq!(x, st.query(i, i + 1));
            }
        } else {
            let mut su = 0;
            for i in l..r {
                su += v[i];
            }
            assert_eq!(su, st.query(l, r));
        }
    }
}

#[test]
fn test_add_min() {
    let n = 2000;
    let q = 2000;
    let mut st = LazySegmentTree::new(
        n,
        |a, b| a.min(b),
        |a, b, len| a + b,
        |a, b| a + b,
        i64::MAX,
        0,
    );
    let mut v: Vec<i64> = (0..n as i64).collect();
    st.build(&v);

    let mut rng = rand::thread_rng();
    for _ in 0..q {
        let t = rng.gen_bool(1.0 / 3.0);
        let l = rng.gen_range(0..n);
        let r = rng.gen_range(l + 1..=n);
        if t {
            let x = rng.gen_range(-100000..100000);
            for i in l..r {
                v[i] += x;
            }
            st.update(l, r, x);
            for i in l..r {
                println!("{} {} {}", l, r, i);
                assert_eq!(v[i], st.query(i, i + 1));
            }
        } else {
            let mut mi = i64::MAX;
            for i in l..r {
                mi = mi.min(v[i]);
            }
            assert_eq!(mi, st.query(l, r));
        }
        println!("{:?}", v);
    }
}

#[test]
fn test_matrix() {
    // a1, a2
    // 0, 1
    // の形をした行列をのせる
    // f は和

    // もっと大きくしたければ、がんばる...

    let n = 100;
    let mut st = LazySegmentTree::new(
        n,
        |a, b| a + b,
        |a, b, len| b.0 * a + b.1,
        |a, b| (a.0 * b.0, b.0 * a.1 + b.1),
        0,
        (1, 0),
    );
}