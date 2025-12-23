use crate::library::number::mint::Modint;

/// p に対応する val の列から、これに対応する x を p で割ったあまりを求める。
/// 
/// O(n^2 + n log(max{pv}))
pub fn garner(mut pv: Vec<i64>, mut valv: Vec<i64>, p: i64) -> i64 {
    pv.push(p);
    valv.push(0);
    let n = pv.len();

    let mut coffs = vec![1; n];
    let mut consts = vec![0; n];
    for i in 0..n-1 {
        let mut val = (valv[i] - consts[i]) * Modint::new_p(coffs[i], pv[i]).inv().x % pv[i];

        if val < 0 {
            val += pv[i];
        }

        for j in i+1..n {
            consts[j] += coffs[j] * val;
            consts[j] %= pv[j];
            coffs[j] *= pv[i];
            coffs[j] %= pv[j];
        }
    }

    consts[n-1]
}