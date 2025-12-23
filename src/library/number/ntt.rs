use crate::library::number::mint::{Modint};
use crate::library::number::garner::*;
use std::sync::OnceLock;

fn check_prime(p: i64) -> (i64, i64, i64) {
    // https://www.mathenachia.blog/ntt-mod-list-01/
    if p == 998244353 {
        (119, 23, 3)
    } else if p == 167772161 {
        (5, 25, 3)
    } else if p == 469762049 {
        (7, 26, 3)
    } else if p == 1224736769 {
        (73, 24, 3)
    } else {
        unimplemented!();
    }
}

fn calc_powv(p: i64) -> Vec<Modint> {
    let (a, b, g) = check_prime(p);
    let mut res = vec![];
    let mut r = Modint::new(g).pow(a);
    for i in 0..b {
        res.push(r);
        r = r * r;
    }
    res.reverse();
    res
}

fn calc_invpowv(p: i64) -> Vec<Modint> {
    let (a, b, g) = check_prime(p);
    let mut res = vec![];
    let mut r = Modint::new(g).pow(a).inv();
    
    for i in 0..b {
        res.push(r);
        r = r * r;
    }
    res.reverse();
    res
}

fn ntt(a: &Vec<Modint>, depth: i64, root: &Vec<Modint>) -> Vec<Modint> {
    let n = a.len();
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

    let d_even = ntt(&even, depth - 1, root);
    let d_odd = ntt(&odd, depth - 1, root);

    let r = root[depth as usize];

    let mut now = Modint::new(1);
    let mut res = vec![];
    for i in 0..n {
        res.push(d_even[i % (n / 2)] + now * d_odd[i % (n / 2)]);
        now = now * r;
    }

    res
}

// Butterfly NTT（反復的な実装、最適化版）
fn butterfly_ntt(a: &mut Vec<Modint>, root: &Vec<Modint>) {
    let n = a.len();
    if n == 1 {
        return;
    }

    let log2n = n.trailing_zeros() as usize;

    // ビット反転によるデータの並び替え（最適化版）
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    // 回転因子を事前計算
    let mut twiddles = vec![vec![Modint::new(1); 1]; log2n];
    for h in 0..log2n {
        let len = 1 << (h + 1);
        let r = root[h];
        twiddles[h].resize(len / 2, Modint::new(1));
        for k in 1..(len / 2) {
            twiddles[h][k] = twiddles[h][k - 1] * r;
        }
    }

    // Butterfly演算
    for h in 0..log2n {
        let len = 1 << (h + 1);
        let half = len / 2;
        let tw = &twiddles[h];
        
        for i in (0..n).step_by(len) {
            for j in 0..half {
                let u = a[i + j];
                let v = a[i + j + half] * tw[j];
                a[i + j] = u + v;
                a[i + j + half] = u - v;
            }
        }
    }
}

/// 任意 mod FFT
/// sza + szb < 2^23 を前提とする。 (8 * 10^6 くらい)
pub fn convolution_with_arbitrary_mod(p: i64, mut a: Vec<i64>, mut b: Vec<i64>) -> Vec<i64> {
    let sza = a.len();
    let szb = b.len();
    let mut n = 1;
    while n <= sza + szb - 1 {
        n *= 2;
    }

    let pv = vec![167772161, 469762049, 1224736769];
    let mut v = vec![];
    for i in 0..3 {
        let mut a_mint = a.iter().map(|&x| Modint::new_p(x, pv[i])).collect::<Vec<_>>();
        let mut b_mint = b.iter().map(|&x| Modint::new_p(x, pv[i])).collect::<Vec<_>>();

        a_mint.resize(n, Modint::new_p(0, pv[i]));
        b_mint.resize(n, Modint::new_p(0, pv[i]));

        let powv = calc_powv(pv[i]);
        let invpowv = calc_invpowv(pv[i]);

        butterfly_ntt(&mut a_mint, &powv);
        butterfly_ntt(&mut b_mint, &powv);

        // 要素ごとの乗算を最適化
        for i in 0..n {
            a_mint[i] = a_mint[i] * b_mint[i];
        }

        butterfly_ntt(&mut a_mint, &invpowv);

        let ninv = Modint::new(n as i64).inv();
        a_mint.truncate(sza + szb - 1);
        for i in 0..(sza + szb - 1) {
            a_mint[i] = a_mint[i] * ninv;
        }
        v.push(a_mint);
    }

    let mut res = vec![];
    for i in 0..v[0].len() {
        let mut valv = vec![];
        for j in 0..3 {
            valv.push(v[j][i].x);
        }
        let val = garner(pv.clone(), valv.clone(), p);
        res.push(val);
    }
    res
}

pub fn convolution(mut a: Vec<Modint>, mut b: Vec<Modint>) -> Vec<Modint> {

    let sza = a.len();
    let szb = b.len();
    let mut n = 1;
    while n <= sza + szb - 1 {
        n *= 2;
    }

    a.resize(n, Modint::new(0));
    b.resize(n, Modint::new(0));

    let powv = calc_powv(998244353);
    let invpowv = calc_invpowv(998244353);

    butterfly_ntt(&mut a, &powv);
    butterfly_ntt(&mut b, &powv);

    // 要素ごとの乗算を最適化
    for i in 0..n {
        a[i] = a[i] * b[i];
    }

    butterfly_ntt(&mut a, &invpowv);

    let ninv = Modint::new(n as i64).inv();
    a.truncate(sza + szb - 1);
    for i in 0..(sza + szb - 1) {
        a[i] = a[i] * ninv;
    }
    a
}
