fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: usize,
        m: usize,
        a: [Usize1; m],
    }

    let mut v = vec![(0, 0); n];
    for i in 0..n {
        v[i].1 = i;
    }
    let f = |(a1, b1), (a2, b2)| {
        if a1 > a2 {
            (a1, b1)
        } else if a1 < a2 {
            (a2, b2)
        } else {
            if b1 < b2 {
                (a1, b1)
            } else {
                (a2, b2)
            }
        }
    };
    let g = |(a, b), (c, _)| (a + c, b);
    let mut st = SegmentTree::new(n, v, f, g, (usize::MIN, usize::MAX));

    for i in 0..m {
        st.update(a[i], (1, 0));
        let (_, val) = st.query(0, n);
        pr(val+1);
    }

}

pub struct SegmentTree<T, F, G>
where
    T: Clone + Copy + std::fmt::Debug,
    F: Fn(T, T) -> T,
    G: Fn(T, T) -> T,
{
    n: usize,
    v: Vec<T>,
    f: F,
    g: G,
    zero: T,
}

impl<T, F, G> SegmentTree<T, F, G>
where
    T: Clone + Copy + std::fmt::Debug,
    F: Fn(T, T) -> T,
    G: Fn(T, T) -> T,
{
    pub fn new(n: usize, v: Vec<T>, f: F, g: G, zero: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }

        let mut v_ = vec![zero; 2 * n_];
        for i in 0..n {
            v_[n_ + i] = v[i];
        }
        for i in (0..=n_ - 1).rev() {
            v_[i] = f(v_[i * 2], v_[i * 2 + 1]);
        }

        SegmentTree {
            n: n_,
            v: v_,
            f,
            g,
            zero,
        }
    }

    pub fn update(&mut self, i: usize, x: T) {
        self.v[self.n + i] = (self.g)(self.v[self.n + i], x);
        let mut now = (self.n + i) / 2;
        while now > 0 {
            self.v[now] = (self.f)(self.v[now * 2], self.v[now * 2 + 1]);
            now /= 2;
        }
    }

    fn query_(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
        if r <= a || b <= l {
            return self.zero;
        }
        if a <= l && r <= b {
            return self.v[k];
        }

        let val1 = self.query_(l, (l + r) / 2, 2 * k, a, b);
        let val2 = self.query_((l + r) / 2, r, 2 * k + 1, a, b);
        (self.f)(val1, val2)
    }

    pub fn query(&self, a: usize, b: usize) -> T {
        self.query_(0, self.n, 1, a, b)
    }
}

#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, Write};

#[allow(unused)]
const MOD998: i64 = 998244353;
#[allow(unused)]
const MOD107: i64 = 1000000007;

#[allow(unused)]
fn pr<T>(val: T)
where
    T: std::fmt::Display,
{
    println!("{}", val);
}

#[allow(unused)]
fn pd<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}
