#![allow(unused)]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    g: Vec<Vec<bool>>
}

fn tatami(g: &mut Vec<Vec<bool>>, mut ng: Vec<Vec<bool>>, ndp: &mut BTreeMap<State, i64>, val: i64, cost: i64) -> bool {
    if g[0][0] && !g[1][0] {
        // #. -> #.
        // .. -> ##
        if g[1][1] {
            return false;
        }

        g[1][0] = true;
        g[1][1] = true;
        ng[1][0] = true;

        // pr_g(&g);
        // pr_g(&ng);
        // println!();

        let now = ndp.entry(State{g: ng}).or_insert(i64::MAX);
        if val + cost < *now {
            *now = val + cost;
        }
    } else if !g[0][0] && g[1][0] {
        // .. -> ##
        // #. -> #.
        if g[0][1] {
            return false;
        }
        g[0][0] = true;
        g[0][1] = true;
        ng[0][0] = true;
        // pr_g(&g);
        // pr_g(&ng);
        // println!();

        let now = ndp.entry(State{g: ng}).or_insert(i64::MAX);
        if val + cost < *now {
            *now = val + cost;
        }
    } else if g[0][0] && g[1][0] {
        // #. -> ##
        // #. -> ##
        if g[0][1] || g[1][1] {
            // pr_g(&g);
            // pr_g(&ng);
            // println!();
            let now = ndp.entry(State{g: ng}).or_insert(i64::MAX);
            if val + cost < *now {
                *now = val + cost;
            }
            return false;
        }
        g[0][1] = true;
        g[1][1] = true;
        ng[0][0] = true;
        ng[1][0] = true;

        // pr_g(&g);
        // pr_g(&ng);
        // println!();
        let now = ndp.entry(State{g: ng}).or_insert(i64::MAX);
        if val + cost < *now {
            *now = val + cost;
        }
    } else {
        panic!();
    }
    true
}

fn pr_g(g: &Vec<Vec<bool>>) {
    for i in 0..2 {
        for j in 0..2 {
            if g[i][j] {
                print!("T");
            } else {
                print!("F");
            }
        }
        println!();
    }
}

fn solve(re: &mut Reader) {
    let n: usize = re.r();
    let l: i64 = re.r();
    let mut a: Vec<(i64, i64)> = vec![];
    for i in 0..n {
        let val1: i64 = re.r();
        let val2: i64 = re.r();
        a.push((val1 - 1, val2 - 1));
    }

    let mut mp = BTreeMap::new();
    mp.entry(0).or_insert(vec![]);
    for i in 0..n {
        let (r, c) = a[i];
        mp.entry(c-1).or_insert(vec![]); // c-1 と c を dp 中でみる
        let v = mp.entry(c).or_insert(vec![]);
        v.push(r);
    }
    pd(&mp);

    let sz = mp.len();
    let mut dp = BTreeMap::new();
    let mut initial_state = State{g: vec![vec![true, false], vec![true, false]]};
    dp.insert(initial_state, 0i64);

    let dir: Vec<(i32, i32, i64)> = vec![(0, 0, 0), (1, 0, 1), (0, 1, 1), (-1, 0, 1), (0, -1, 1)];
    let idx_v: Vec<i64> = mp.keys().cloned().collect();
    for i in 0..idx_v.len() {
        let idx = idx_v[i];
        pr(idx);
        pd(&dp);
        let mut ndp = BTreeMap::new();
        for (state, val) in dp {
            let g = state.g;
            println!("prev");
            pr_g(&g);
            println!();

            // コマ配置
            let mut v = if mp.contains_key(&(idx + 1)) {
                mp[&(idx+1)].clone()
            } else {
                vec![]
            };

            v.sort();
            if v == vec![] {
                let mut g = g.clone();
                let mut ng = vec![vec![false;2];2];
                ng[0][0] = g[0][1];
                ng[1][0] = g[1][1];
                let cost = 0;

                tatami(&mut g, ng, &mut ndp, val, cost);

            } else if v == vec![0] || v == vec![1] {
                let (r, c) = if v == vec![0] {
                    (0, 1)
                } else {
                    (1, 1)
                };

                // コマの動かし方を 5 通りためす
                for k in 0..5 {
                    let nr = r + dir[k].0;
                    let nc = c + dir[k].1;
                    let cost = dir[k].2;
                    if !(0 <= nr && nr < 2) || (dir[k].1 == -1 && idx + 1 == 0) || (dir[k].1 == 1 && idx + 1 == l-1) {
                        continue;
                    }

                    let mut g = g.clone();
                    let mut ng = vec![vec![false;2];2];
                    ng[0][0] = g[0][1];
                    ng[1][0] = g[1][1];

                    if nc <= 1 && g[nr as usize][nc as usize] {
                        continue;
                    }
                    if nc >= 1 && ng[nr as usize][(nc - 1) as usize] {
                        continue;
                    }
                    if nc <= 1 {
                        g[nr as usize][nc as usize] = true;
                    }
                    if nc >= 1 {
                        ng[nr as usize][(nc - 1) as usize] = true;
                    }


                    // 畳配置
                    tatami(&mut g, ng, &mut ndp, val, cost);

                }
            } else if v == vec![0, 1] {
                // ふたつうごかす
                let (r, c) = (0, 1);

                // 一つ目のコマの動かし方を 5 通りためす
                for k in 0..5 {
                    let nr = r + dir[k].0;
                    let nc = c + dir[k].1;
                    let cost = dir[k].2;
                    if !(0 <= nr && nr < 2) || (dir[k].1 == -1 && idx + 1 == 0) || (dir[k].1 == 1 && idx + 1 == l-1) {
                        continue;
                    }

                    let mut g = g.clone();
                    let mut ng = vec![vec![false;2];2];
                    ng[0][0] = g[0][1];
                    ng[1][0] = g[1][1];

                    if nc <= 1 && g[nr as usize][nc as usize] {
                        continue;
                    }
                    if nc >= 1 && ng[nr as usize][(nc - 1) as usize] {
                        continue;
                    }

                    if nc <= 1 {
                        g[nr as usize][nc as usize] = true;
                    }
                    if nc >= 1 {
                        ng[nr as usize][(nc - 1) as usize] = true;
                    }

                    // ふたつめの駒の動かし方を5通り試す
                    let (r, c) = (1, 1);
                    for k in 0..5 {
                        let nr = r + dir[k].0;
                        let nc = c + dir[k].1;
                        let cost = cost + dir[k].2;
                        if !(0 <= nr && nr < 2) || (dir[k].1 == -1 && idx + 1 == 0) || (dir[k].1 == 1 && idx + 1 == l-1) {
                            continue;
                        }

                        let mut g = g.clone();
                        let mut ng = ng.clone();
                        ng[0][0] = g[0][1];
                        ng[1][0] = g[1][1];

                        if nc <= 1 && g[nr as usize][nc as usize] {
                            continue;
                        }
                        if nc >= 1 && ng[nr as usize][(nc - 1) as usize] {
                            continue;
                        }

                        if nc <= 1 {
                            g[nr as usize][nc as usize] = true;
                        }
                        if nc >= 1 {
                            ng[nr as usize][(nc - 1) as usize] = true;
                        }


                        tatami(&mut g, ng, &mut ndp, val, cost);
                    }


                }
            }

        }

        // 次まで遠かったらすすめる
        // let nidx = idx_v[i+1];
        // if i != idx_v.len() - 1 && nidx > idx + 1 {
        //     // unimplemented
        // }

        dp = ndp;
    }

    pd(dp);

}

fn main() {
    let mut s = String::new();
    let stdin = stdin();
    let mut re = Reader::new(&mut s, stdin);
    
    solve(&mut re);
}


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

    // read a token
    fn r<T: FromStr>(&mut self) -> T {
        let str = self.tokens[self.idx].pop_front().unwrap();
        let res = str.parse().ok().unwrap();
        if self.tokens[self.idx].is_empty() {
            self.idx += 1;
        }
        res
    }

    // read vec
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

    // read n lines
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

    // read string as chars
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
fn adj_pos(h: usize, w: usize, r: usize, c: usize) -> Vec<(usize, usize)> {
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

