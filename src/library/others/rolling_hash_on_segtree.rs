use crate::library::others::rolling_hash::RollingHash;
use crate::library::structure::segment_tree::SegmentTree;

// 問題例
// https://atcoder.jp/contests/abc331/tasks/abc331_f

type ElmT = (u128, usize);
type ElmFunc = Box<dyn Fn(ElmT, ElmT)->ElmT>;
pub struct RollingHashOnSegTree
{
    st: SegmentTree<ElmT, ElmFunc, ElmFunc>,
    pow: Vec<u128>,
}

impl RollingHashOnSegTree
{
    pub const MOD: u128 = RollingHash::MOD;
    pub const BASE: u128 = RollingHash::BASE;

    pub fn new(s: Vec<char>) -> Self {
        let v: Vec<u128> = s.iter().map(|c| RollingHash::xorshift(*c as u128 + 1)).collect();
        let n = v.len();
        let mut pow = vec![1; n + 1];
        for i in 0..n {
            pow[i + 1] = RollingHash::mul(pow[i], Self::BASE);
        }

        let pow_f = pow.clone();
        let f: ElmFunc = Box::new(move |(h1, len1), (h2, len2)| {
            let res = RollingHash::mul(h1, pow_f[len2]) + h2;
            if res >= Self::MOD {
                (res - Self::MOD, len1 + len2)
            } else {
                (res, len1 + len2)
            }
        });
        let g: ElmFunc = Box::new(|a, b| b);

        let mut st = SegmentTree::new(v.len(), v.iter().map(|x| (*x, 1)).collect(), f, g, (0, 0));

        RollingHashOnSegTree { st, pow }
    }

    pub fn update(&mut self, idx: usize, val: char) {
        self.st.update(idx, (RollingHash::xorshift(val as u128 + 1), 1));
    }

    pub fn query(&mut self, l: usize, r: usize) -> u128 {
        self.st.query(l, r).0
    }
}
