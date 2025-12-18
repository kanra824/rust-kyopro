use super::gcd::*;

pub fn ceil(x: i64, y: i64) -> i64 {
    if x > 0 {
        (x + y - 1) / y
    } else {
        x / y
    }
}

pub fn min_of_linear_segments(mut a: i64, mut b: i64, mut m: i64) -> (Vec<i64>, Vec<i64>) {
    assert!(0 <= a && a < m);
    assert!(0 <= b && b < m);
    let mut vx = vec![0];
    let mut dx = vec![];
    let g = gcd(a, m);
    a /= g;
    b /= g;
    m /= g;

    // p/q <= (m-a)/m <= r/s
    let mut p = 0;
    let mut q = 1;
    let mut r = 1;
    let mut s = 1;
    let mut det_l = m - a;
    let mut det_r = a;
    let mut x = 0;
    let mut y = b;

    while y > 0 {
        // upd r/s
        let mut k = det_r / det_l;
        det_r %= det_l;
        if det_r ==  0 {
            k -= 1;
            det_r = det_l;
        }
        r += k * p;
        s += k * q;
        loop {
            let mut k = ceil(det_l - y, det_r).max(0);
            if det_l - k * det_r <= 0 {
                break;
            }
            det_l -= k * det_r;
            p += k * r;
            q += k * s;

            // p/q <= a/m
            // (aq - pm) = det_l を y から引く
            k = y / det_l;
            y -= k * det_l;
            x += q * k;
            vx.push(x);
            dx.push(q);
        }
        k = det_l / det_r;
        det_l -= k * det_r;
        p += k * r;
        q += k * s;
    }

    (vx, dx)
}

// min {x in [l, r)} (ax + b mod m)
pub fn min_of_mod_of_linear(l: i64, r: i64, a: i64, mut b: i64, m: i64) -> (i64, i64) {
    let n = r - l;
    b = (b + a * l) % m;
    if b < 0 {
        b += m;
    }

    let (vx, dx) = min_of_linear_segments(a, b, m);

    let mut x = 0;
    for i in 0..vx.len()-1 {
        let xl = vx[i];
        let xr = vx[i+1];
        if xr < n {
            x = xr;
            continue;
        }
        x = xl + (n - 1 - x) / dx[i] * dx[i];
        break;
    }
    let y = (a * x + b) % m;
    (l + x, y)
}