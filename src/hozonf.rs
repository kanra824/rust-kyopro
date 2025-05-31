#![allow(unused)]

#[derive(Clone, Copy, Debug)]
enum Expr {
    Sum(i64, i64),
    Mul(i64, bool, i64, bool),
    Num(i64),
}

#[derive(Clone, Debug)]
struct State {
    sum_cnt: i64,
    sum: Option<Expr>,
    mul_cnt: i64,
    mul: Option<Expr>,
    num_cnt: i64,
    num: Option<Expr>,
}

impl State {
    fn new() -> Self {
        State {
            sum_cnt: i64::MAX,
            sum: None,
            mul_cnt: i64::MAX,
            mul: None,
            num_cnt: i64::MAX,
            num: None
        }
    }

    fn get_mi(&self) -> (i64, Expr) {
        let mut mi_cnt = self.sum_cnt;
        let mut mi = self.sum;

        if mi_cnt > self.mul_cnt {
            mi_cnt = self.mul_cnt;
            mi = self.mul;
        }

        if mi_cnt > self.num_cnt {
            mi_cnt = self.num_cnt;
            mi = self.num;
        }

        (mi_cnt, mi.unwrap())
    }

    fn get_paren_mi(&self) -> (i64, Expr, bool) {
        let mut mi_cnt = if self.sum_cnt == i64::MAX {
            i64::MAX
        } else {
            self.sum_cnt + 2
        };
        let mut mi = self.sum;

        if mi_cnt > self.mul_cnt {
            mi_cnt = self.mul_cnt;
            mi = self.mul;
        }

        if mi_cnt > self.num_cnt {
            mi_cnt = self.num_cnt;
            mi = self.num;
        }

        let paren = if let Some(mi) = mi {
            match mi {
                Expr::Sum(_, _) => true,
                _ => false,
            }
        } else {
            false
        };

        (mi_cnt, mi.unwrap(), paren)
    }

    fn print(paren: bool, n: i64, mp: &HashMap<i64, State>) {
        let state = &mp[&n];
        let mi = if paren {
            state.get_mi().1
        } else {
            state.get_paren_mi().1
        };

        match mi {
            Expr::Num(i) => {
                print!("{}", i)
            },
            Expr::Sum(i, j) => {
                State::print(false,i, mp);
                print!("+");
                State::print(false,j, mp);
            },
            Expr::Mul(i, pareni, j, parenj) => {
                if pareni {
                    print!("(");
                    State::print(true, i, mp);
                    print!(")");
                } else {
                    State::print(false, i, mp);
                }
                print!("*");
                if parenj {
                    print!("(");
                    State::print(true, j, mp);
                    print!(")");
                } else {
                    State::print(false, j, mp);
                }
            }
        }
    }

}

fn main() {
    // // AOJ, codeforces, etc...
    // let mut s = String::new();
    // let stdin = stdin();
    // let mut reader = Reader::new(&mut s, stdin);

    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: i64,
    }

    let mut mp: HashMap<i64, State> = HashMap::new();
    mp.insert(1, State{
        sum_cnt: i64::MAX,
        sum: None,
        mul_cnt: i64::MAX,
        mul: None,
        num_cnt: 1,
        num: Some(Expr::Num(1)),
    });

    for i in 2..=n {
        let mut nxt = State::new();

        // そのまま
        if i == 11 {
            nxt.num_cnt = 2;
            nxt.num = Some(Expr::Num(11));
        }
        if i == 111 {
            nxt.num_cnt = 3;
            nxt.num = Some(Expr::Num(111));
        }
        if i == 1111 {
            nxt.num_cnt = 4;
            nxt.num = Some(Expr::Num(1111));
        }

        // 和
        for j in 1..=(i/2+1).min(i-1) {
            let k = i - j;
            let sj = &mp[&j];
            let (micntj, mij) = sj.get_mi();
            let sk = &mp[&k];
            let (micntk, mik) = sk.get_mi();

            if nxt.sum_cnt > micntj + 1 + micntk {
                nxt.sum_cnt = micntj + 1 + micntk;
                nxt.sum = Some(Expr::Sum(j, k));
            }
        }

        // 積
        // 和と和, 積と積 -> そのまま
        // 和と積, 積と和 -> 和にカッコつける
        for j in 2..=(i/2+1).min(i-1) {
            if i % j != 0 {
                continue;
            }
            let k = i / j;

            let sj = &mp[&j];
            let (micntj, mij, parenj) = sj.get_paren_mi();
            let sk = &mp[&k];
            let (micntk, mik, parenk) = sk.get_paren_mi();

            if nxt.mul_cnt > micntj + 1 + micntk {
                nxt.mul_cnt = micntj + 1 + micntk;
                nxt.mul = Some(Expr::Mul(j, parenj, k, parenk));
            }
        }

        mp.insert(i, nxt);
    }

    // for i in 1..n {
    //     println!("{}", i);
    //     State::print(i, &mp);
    //     println!();
    // }
    State::print(false, n, &mp);
    println!();
}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};
use std::str::FromStr;
use std::{fmt, ops};

/// 有名MODその1
const MOD998: i64 = 998244353;
/// 有名MODその2
const MOD107: i64 = 1000000007;

/// 単一の値をプリントするための関数
fn pr<T>(val: T)
where
    T: std::fmt::Display,
{
    println!("{}", val);
}

fn pr_vec<T>(v: &Vec<T>)
where
    T: std::fmt::Display,
{
    for i in 0..v.len() {
        print!("{}", v[i]);
        if i == v.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}

fn pr_yesno(x: bool) {
    if x {
        pr("Yes");
    } else {
        pr("No");
    }
}

/// 単一の値をデバッグプリントするための関数
fn pd<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}

struct Reader<'a> {
    stdin: Stdin,
    tokens: Vec<VecDeque<&'a str>>,
    idx: usize,
}

impl<'a> Reader<'a> {
    fn new(str: &'a mut String, mut stdin: Stdin) -> Self {
        stdin.read_to_string(str).unwrap();
        let tokens: Vec<VecDeque<&str>> = str
            .trim()
            .split('\n')
            .map(|v| v.split_whitespace().collect())
            .collect();
        Reader {
            stdin,
            tokens,
            idx: 0,
        }
    }

    fn r<T: FromStr>(&mut self) -> T {
        let str = self.tokens[self.idx].pop_front().unwrap();
        let res = str.parse().ok().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        res
    }

    fn rv<T: FromStr>(&mut self) -> Vec<T> {
        let deque = &mut self.tokens[self.idx];
        let mut res = vec![];
        while !deque.is_empty() {
            let str = deque.pop_front().unwrap();
            res.push(str.parse().ok().unwrap());
        }
        self.idx += 1;
        res
    }

    fn rl<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut res = vec![];
        let len = self.tokens.len();
        assert!(self.idx + n <= len);
        for i in 0..n {
            let str = self.tokens[self.idx].pop_front().unwrap();
            res.push(str.parse().ok().unwrap());
            assert!(self.tokens[self.idx].is_empty());
            self.idx += 1;
        }
        res
    }

    fn as_chars(&mut self) -> Vec<char> {
        let str = self.tokens[self.idx].pop_front().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        str.chars().collect()
    }

    fn end_input(&self) -> bool {
        self.idx > self.tokens.len()
    }
}

// グリッドの範囲を見てすすめるマスを列挙(壁がある場合は呼び出し側でチェック)
fn adj_pos(w: usize, h: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let dir = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    for (dr, dc) in dir {
        let nr = r as i64 + dr;
        let nc = c as i64 + dc;
        if !(0 <= nr && nr < h as i64 && 0 <= nc && nc < w as i64) {
            continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        res.push((nr, nc))
    }
    res
}

fn char_to_i64(c: char) -> i64 {
    c as u32 as i64 - '0' as u32 as i64
}

