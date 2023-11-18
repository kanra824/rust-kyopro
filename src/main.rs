fn main() {
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: usize,
        q: usize,
        c: [Usize1; n],
        queries: [(Usize1, Usize1); q],
    }
    let mut idx = vec![0; n];
    for i in 0..n {
        idx[i] = i;
    }
    let mut v = vec![HashSet::new();n];
    let mut sz = vec![1; n];
    for i in 0..n {
        v[i].insert(c[i]);
    }

    for (a, b) in queries {
        if sz[a] > sz[b] {
            // idx[a], idx[b] を swap した上で、 このあとidx[a] を idx[b] にうつす
            let tmp = idx[b];
            idx[b] = idx[a];
            idx[a] = tmp;
        }


        // idx[a] から idx[b] へ
        let ia = idx[a];
        let ib = idx[b];
        //assert!(sz[ia] <= sz[ib]);
        sz[ib] = sz[ia] + sz[ib];
        sz[ia] = 0;

        for elm in v[ia].clone() {
            v[ib].insert(elm);
        }
        v[ia] = HashSet::new();
        pr(v[ib].len());
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
