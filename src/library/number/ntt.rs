use crate::library::number::mint::{MOD, Modint};

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

pub fn convolution(mut a: Vec<Modint>, mut b: Vec<Modint>) -> Vec<Modint> {
    let sza = a.len();
    let szb = b.len();
    let mut n = 1;
    while n <= sza + szb - 1 {
        n *= 2;
    }

    while a.len() < n {
        a.push(Modint::zero());
    }
    while b.len() < n {
        b.push(Modint::zero());
    }

    let mut log_2n = 1;
    while 1<<log_2n < n {
        log_2n += 1;
    }

    let powv = calc_powv();
    let invpowv = calc_invpowv();

    let da = ntt(&a, log_2n - 1, &powv);
    let db = ntt(&b, log_2n - 1, &powv);

    let mut dc = vec![];
    for i in 0..n {
        dc.push(da[i] * db[i]);
    }

    let c = ntt(&dc, log_2n - 1, &invpowv);

    let mut res = vec![];
    let ninv = Modint::new(n as i64).inv();
    for i in 0..sza + szb - 1 {
        res.push(c[i] * ninv);
    }
    res
}
