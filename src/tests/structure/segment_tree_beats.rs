use crate::library::structure::segment_tree_beats::*;
use rand::{self, Rng};

#[test]
fn test_segment_tree_beats() {
    let mut rng = rand::rng();
    let mut n = 1000;
    let mut q = 100000;
    let mut a = vec![0; n];
    let inf = 1000000000;

    for i in 0..n {
        a[i] = rng.random_range(-inf..inf);
    }

    let mut st = SegmentTreeBeats::new(n, a.clone());

    for i in 0..q {
        let t = rng.random_range(0..7);
        if t == 0 {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let x = rng.random_range(-inf..inf);
            st.update_min(l, r, x);
            update_min(l, r, x, &mut a);
        } else if t == 1 {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let x = rng.random_range(-inf..inf);
            st.update_max(l, r, x);
            update_max(l, r, x, &mut a);
        } else if t == 2 {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let x = rng.random_range(-inf..inf);
            st.add_val(l, r, x);
            add_val(l, r, x, &mut a);
        } else if t == 3 {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let x = rng.random_range(-inf..inf);
            st.update_val(l, r, x);
            update_val(l, r, x, &mut a);
        } else if t == 4 {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let val1 = st.query_min(l, r);
            let val2 = query_min(l, r, &a);
            assert_eq!(val1, val2);
        } else if t == 5 {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let val1 = st.query_max(l, r);
            let val2 = query_max(l, r, &a);
            assert_eq!(val1, val2);
        } else {
            let l = rng.random_range(0..n);
            let r = rng.random_range(l+1..=n);
            let val1 = st.query_sum(l, r);
            let val2 = query_sum(l, r, &a);
            assert_eq!(val1, val2);
        }
    }
}

fn update_min(l: usize, r: usize, x: i64, v: &mut Vec<i64>) {
    for i in l..r {
        v[i] = v[i].min(x);
    }
}

fn update_max(l: usize, r: usize, x: i64, v: &mut Vec<i64>) {
    for i in l..r {
        v[i] = v[i].max(x);
    }
}

fn add_val(l: usize, r: usize, x: i64, v: &mut Vec<i64>) {
    for i in l..r {
        v[i] += x;
    }
}

fn update_val(l: usize, r: usize, x: i64, v: &mut Vec<i64>) {
    for i in l..r {
        v[i] = x;
    }
}

fn query_max(l: usize, r: usize, v: &Vec<i64>) -> i64 {
    let mut res = i64::MIN;
    for i in l..r {
        res = res.max(v[i]);
    }
    res
}

fn query_min(l: usize, r: usize, v: &Vec<i64>) -> i64 {
    let mut res = i64::MAX;
    for i in l..r {
        res = res.min(v[i]);
    }
    res
}

fn query_sum(l: usize, r: usize, v: &Vec<i64>) -> i64 {
    let mut res = 0;
    for i in l..r {
        res += v[i];
    }
    res
}
