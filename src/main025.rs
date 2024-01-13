use rand;
use rand::{Rng, SeedableRng, seq::SliceRandom};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
        d: usize,
        q: usize,
    }
    
    // 方針
    // ランダムにd個に分ける
    let mut v = vec![0; n];
    for i in 0..n {
        v[i] = i % d;
    }
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    v.shuffle(&mut rng);

    // ランダムに2個測る
    // 大きい方から小さい方へ一つ移す
    for i in 0..q {
        let a1: usize = rng.gen_range(0..d);
        let mut a2 = rng.gen_range(0..d-1);
        if a1 == a2 {
            a2 += 1;
        }
        let mut v1 = vec![];
        let mut v2 = vec![];
        for i in 0..n {
            if v[i] == a1 {
                v1.push(i);
            } else if v[i] == a2 {
                v2.push(i);
            }
        }
        print!("{} {}", v1.len(), v2.len());
        for i in 0..v1.len() {
            print!(" {}", v1[i]);
        }
        for i in 0..v2.len() {
            print!(" {}", v2[i]);
        }
        println!("");
        
        input! {
            from &mut source,
            c: char,
        }
        if c == '<' {
            let idx = rng.gen_range(0..v2.len());
            v[v2[idx]] = a1;
            if v2.len() == 1 {
                let idx = rng.gen_range(0..v1.len());
                v[v1[idx]] = a2;
            }
        } else if c == '>' {
            let idx = rng.gen_range(0..v1.len());
            v[v1[idx]] = a2;
            if v1.len() == 1 {
                let idx = rng.gen_range(0..v2.len());
                v[v2[idx]] = a1;
            }
        }
    }
    
    for i in 0..n {
        print!("{}", v[i]);
        if i != n-1 {
            print!(" ");
        }
    }
    println!("");
}

#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
#[allow(unused_imports)]
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufReader, Write};
use std::collections::*;
use std::cmp::{min, max};

#[allow(unused)]
const MOD998: i64 = 998244353;
#[allow(unused)]
const MOD107: i64 = 1000000007;

#[allow(unused)]
fn pp<T>(val: T)
where
    T: std::fmt::Display,
{
    println!("{}", val);
}

