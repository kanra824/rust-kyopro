
use crate::library::others::rolling_hash_on_segtree::*;
use crate::library::others::rolling_hash::*;

const MOD: u128 = RollingHashOnSegTree::MOD;
const BASE: u128 = RollingHashOnSegTree::BASE;

fn xorshift(mut x: u128) -> u128 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

fn mul(a: u128, b: u128) -> u128 {
    let mut t = a * b;
    t = (t >> 61) + (t & MOD);
    if t >= MOD {
        t - MOD
    } else {
        t
    }
}

#[test]
fn test_rolling_hash_on_segtree() {
    let s: Vec<char> = "abcdef".chars().collect();
    let mut st = RollingHashOnSegTree::new(s);

    st.update(3, 'k');
    let val1 = st.query(2, 4); // ck

    let rh = RollingHash::new_from_literal("ck");
    let val2 = rh.hash(0, 2); // ck

    assert_eq!(val1, val2);
}

#[test]
fn rolling_hash_on_segtree() {
    use crate::library::structure::segment_tree::SegmentTree;

    let s: Vec<char> = "abcdef".chars().collect();
    let n = s.len();
    let mut v: Vec<(u128, usize)> = vec![(0, 0); n + 1];
    let mut pow: Vec<u128> = vec![1; n + 1];
    for i in 0..n {
        v[i] = ((xorshift(s[i] as u128 + 1), 1));
        pow[i + 1] = mul(pow[i], BASE);
    }

    let mut st = SegmentTree::new(
        n,
        v,
        |(h1, len1), (h2, len2)| {
            let res = mul(h1, pow[len2]) + h2;
            if res >= MOD {
                (res - MOD, len1 + len2)
            } else {
                (res, len1 + len2)
            }
        },
        |a, b| b,
        (0, 0),
    );

    println!("{:?}", st.v);

    st.update(3, (xorshift('k' as u128 + 1), 1));
    let val1 = st.query(2, 4); // cd

    let rh = RollingHash::new_from_literal("ck");
    let val2 = rh.hash(0, 2);

    assert_eq!(val1.1, 2);
    assert_eq!(val1.0, val2);
}
