use crate::library::number::mint::Modint;
use super::fps::*;

/// Bostan-Mori
/// 
/// P(x) / Q(x) の n 項目を計算する
/// 
/// P(x), Q(x) の最大次数を k として、 O(k logk logn )で動作する
/// 
/// TODO: 線形漸化式を P(x) / Q(x) に変換するパートを実装したい。
pub fn bostan_mori(n: usize, mut p: Fps, mut q: Fps) -> Modint {
    let mut k = n;
    while k > 0 {
        // 分子分母に Q(-x) をかける
        let mut minusq_v = vec![];
        for i in 0..q.n {
            if i % 2 == 0 {
                minusq_v.push(q.a[i]);
            } else {
                minusq_v.push(-q.a[i]);
            }
        }
        let minusq = Fps::from_mint_vec(minusq_v);
        let mut pq = &p * &minusq;
        let mut qq = &q * &minusq;
        let mut np = vec![];
        let mut nq = vec![];
        if k % 2 == 0 {
            // 偶数項を見る
            for i in 0..pq.n {
                if i % 2 == 0 {
                    np.push(pq.a[i]);
                }
            }
            for i in 0..qq.n {
                if i % 2 == 0 {
                    nq.push(qq.a[i]);
                }
            }

        } else {
            // 奇数項を見る
            for i in 0..pq.n {
                if i % 2 == 1 {
                    np.push(pq.a[i]);
                }
            }
            for i in 0..qq.n {
                if i % 2 == 0 {
                    nq.push(qq.a[i]);
                }
            }
            if np.len() == 0 {
                np.push(Modint::new(0));
            }
        }
        p = Fps::from_mint_vec(np);
        q = Fps::from_mint_vec(nq);
        k /= 2;
    }

    p.a[0] / q.a[0]
}

/// a_n = c_(k-1) * a_(n-1) + c_(k-2) * a_(n-2) + ... + c_0 * a_(n-k) とする。
///
/// Q(x) = 1 - (c_(k-1) * x + c_(k-2) * x^2 + ... + c_1 * x^k) とする。
/// 
/// また、A(x) = a_0 + a_1 * x + ... + a_(k-1) * x^(k-1) とする。
/// 
/// P(x) = A(x) * Q(x) mod x^k としてもとまる。
fn convert_linear_recurrence_relation(mut c: Vec<i64>, a: Vec<i64>) -> (Fps, Fps) {
    assert_eq!(c.len(), a.len());
    let k = c.len();
    c.reverse();
    let a_poly = Fps::from_i64_vec(a);
    let mut q = vec![1];
    for i in 0..k {
        q.push(-c[i]);
    }
    let q_poly = Fps::from_i64_vec(q);
    let p_poly = (&a_poly * &q_poly).get_n(k-1);

    (p_poly, q_poly)
}

/// a_n = c_(k-1) * a_(n-1) + c_(k-2) * a_(n-2) + ... + c_0 * a_(n-k) とする。
/// 
/// n 項目を O(k logk logn) で求める。
pub fn bostan_mori_from_linear_recurrence_relation(n: usize, c: Vec<i64>, a: Vec<i64>) -> Modint {
    let (p, q) = convert_linear_recurrence_relation(c, a);
    bostan_mori(n, p, q)
}
