use std::io;
use std::str::FromStr;

fn input<T: FromStr>() -> T {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().ok().unwrap()
}

fn input_vec<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let v = buffer.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
    v
}

// 複数の型が入り得る場合を処理したい（どうやって？）

fn input_lines<T: FromStr>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    for i in 0..n {
        v.push(input());
    }
    v
}

fn solve(n: usize) {
    ()
}

fn main() {
    loop {
        let n = input();
        if n == 0 {
            break;
        }
        solve(n);
    }
}
