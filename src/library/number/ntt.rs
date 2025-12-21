use crate::library::number::mint::{Modint};
use std::sync::OnceLock;

static POWV: OnceLock<Vec<Modint>> = OnceLock::new();
static INVPOWV: OnceLock<Vec<Modint>> = OnceLock::new();

fn calc_powv() -> Vec<Modint> {
    let mut res = vec![];
    let mut r = Modint::new(3).pow(119);
    for i in 0..23 {
        res.push(r);
        r = r * r;
    }
    res.reverse();
    res
}

fn calc_invpowv() -> Vec<Modint> {
    let mut res = vec![];
    let mut r = Modint::new(3).pow(119).inv();
    for i in 0..23 {
        res.push(r);
        r = r * r;
    }
    res.reverse();
    res
}

fn get_powv() -> &'static Vec<Modint> {
    POWV.get_or_init(calc_powv)
}

fn get_invpowv() -> &'static Vec<Modint> {
    INVPOWV.get_or_init(calc_invpowv)
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

pub fn convolution(mut a: Vec<Modint>, mut b: Vec<Modint>) -> Vec<Modint> {
    let sza = a.len();
    let szb = b.len();
    let mut n = 1;
    while n <= sza + szb - 1 {
        n *= 2;
    }

    a.resize(n, Modint::new(0));
    b.resize(n, Modint::new(0));

    let powv = get_powv();
    let invpowv = get_invpowv();

    butterfly_ntt(&mut a, powv);
    butterfly_ntt(&mut b, powv);

    // 要素ごとの乗算を最適化
    for i in 0..n {
        a[i] = a[i] * b[i];
    }

    butterfly_ntt(&mut a, invpowv);

    let ninv = Modint::new(n as i64).inv();
    a.truncate(sza + szb - 1);
    for i in 0..(sza + szb - 1) {
        a[i] = a[i] * ninv;
    }
    a
}
