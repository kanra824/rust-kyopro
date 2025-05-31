#![allow(unused)]

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_distr::{Dirichlet, Distribution};

#[derive(Clone, Debug)]

struct Input {
    n: usize,
    k: usize,
    h: usize,
    t: usize,
    d: i64,
    own: Vec<[f64; 3]>,
    target: Vec<[f64; 3]>,
}

fn input() -> Input {
    input! {
        n: usize,
        k: usize,
        h: usize,
        t: usize,
        d: i64,
        own: [[f64; 3]; k],
        target: [[f64; 3]; h],
    }

    Input {
        n,
        k,
        h,
        t,
        d,
        own: own.into_iter().map(|x| [x[0], x[1], x[2]]).collect(),
        target: target.into_iter().map(|x| [x[0], x[1], x[2]]).collect(),
    }
}

fn output(wall_v: &Vec<Vec<bool>>, wall_h: &Vec<Vec<bool>>, actions: &Vec<Action>) {
    let n = wall_v.len();
    for i in 0..n {
        for j in 0..n - 1 {
            print!("{} ", if wall_v[i][j] { '1' } else { '0' });
        }
        println!();
    }
    for i in 0..n - 1 {
        for j in 0..n {
            print!("{} ", if wall_h[i][j] { '1' } else { '0' });
        }
        println!();
    }
    for action in actions {
        match action {
            Action::Add { i, j, k } => println!("1 {} {} {}", i, j, k),
            Action::Deliver { i, j } => println!("2 {} {}", i, j),
            Action::Discard { i, j } => println!("3 {} {}", i, j),
            Action::Toggle { i1, j1, i2, j2 } => println!("4 {} {} {} {}", i1, j1, i2, j2),
        }
    }
}

enum Action {
    Add {
        i: usize,
        j: usize,
        k: usize,
    },
    Deliver {
        i: usize,
        j: usize,
    },
    Discard {
        i: usize,
        j: usize,
    },
    Toggle {
        i1: usize,
        j1: usize,
        i2: usize,
        j2: usize,
    },
}

struct State {
    wall_v: Vec<Vec<bool>>, // n * n-1
    wall_h: Vec<Vec<bool>>, // n-1 * n
    ids: Vec<Vec<usize>>,
    caps: Vec<i64>, // id に対応する範囲の容量
    vols: Vec<f64>, // id に対応する各マスに入っている絵の具の量
    colors: Vec<[f64; 3]>, // idに対応する範囲の色
    delivered: Vec<[f64; 3]>, // 
    v: i64,
    error: f64,
    rng: ChaCha20Rng,
    input: Input,
    lambda: usize,
}

fn get_ids(
    wall_v: &Vec<Vec<bool>>,
    wall_h: &Vec<Vec<bool>>
) -> (usize, Vec<Vec<usize>>, Vec<i64>) {
    let n = wall_v.len();
    let mut ids = vec![vec![usize::MAX; n]; n];
    let mut id = 0;
    let mut caps = vec![];
    for i in 0..n {
        for j in 0..n {
            if ids[i][j] != usize::MAX {
                continue;
            }
            let mut st = vec![(i, j)];
            ids[i][j] = id;
            let mut cap = 0;
            while let Some((i, j)) = st.pop() {
                cap += 1;
                if j + 1 < n && !wall_v[i][j] && ids[i][j+1] == usize::MAX {
                    ids[i][j+1] = id;
                    st.push((i, j+1));
                }
                if i + 1 < n && !wall_h[i][j] && ids[i+1][j] == usize::MAX {
                    ids[i+1][j] = id;
                    st.push((i+1, j));
                }
                if j > 0 && !wall_v[i][j-1] && ids[i][j-1] == usize::MAX {
                    ids[i][j-1] = id;
                    st.push((i, j-1));
                }
                if i > 0 && !wall_h[i-1][j] && ids[i-1][j] == usize::MAX {
                    ids[i-1][j] = id;
                    st.push((i-1, j));
                }
            }
            caps.push(cap);
            id += 1;
        }
    }
    (id, ids, caps)
}

fn mix(v1: f64, p1: [f64; 3], v2: f64, p2: [f64; 3]) -> [f64; 3] {
    let sum = v1 + v2;
    if sum <= 0.0 {
        return [0.0; 3];
    }
    [
        (v1 * p1[0] + v2 * p2[0]) / sum,
        (v1 * p1[1] + v2 * p2[1]) / sum,
        (v1 * p1[2] + v2 * p2[2]) / sum,
    ]
}

fn distance(p1: [f64; 3], p2: [f64; 3]) -> f64 {
    ((p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2) + (p1[2] - p2[2]).powi(2)).sqrt()
}

impl State {
    fn new(input: Input, wall_v: &Vec<Vec<bool>>, wall_h: &Vec<Vec<bool>>, lambda: usize) -> State {
        let (id, ids, caps) = get_ids(wall_v, wall_h);
        let vols = vec![0.0; id];
        let colors = vec![[0.0; 3]; id];

        let rng = ChaCha20Rng::seed_from_u64(12345);

        State{
            wall_v: wall_v.clone(),
            wall_h: wall_h.clone(),
            ids,
            caps,
            vols,
            colors,
            delivered: vec![],
            v: 0,
            error: 0.0,
            rng,
            input,
            lambda,
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::Add { i, j, k } => {
                // ここに絵の具を追加するロジックを書く
                // input.own[k] を (i, j) に追加

                self.v += 1;
                let id = self.ids[i][j];
                let w = self.caps[id] as f64 - self.vols[id];
                if w <= 1.0 {
                    self.colors[id] = mix(self.vols[id], self.colors[id], w, self.input.own[k]);
                    self.vols[id] = self.caps[id] as f64;
                } else {
                    self.colors[id] = mix(self.vols[id], self.colors[id], 1.0, self.input.own[k]);
                    self.vols[id] += 1.0;
                }
            }
            Action::Deliver { i, j } => {
                // ここに絵の具を配達するロジックを書く
                // (i, j) の色を計算して、delivered に追加

                if self.delivered.len() >= self.input.h {
                    panic!("Too many deliveries");
                }

                if self.vols[self.ids[i][j]] < 1.0 - 1e-6 {
                    panic!("Cannot deliver less than 1.0 volume");
                }

                let color = self.colors[self.ids[i][j]];
                let target = self.input.target[self.delivered.len()];
                self.error += ((color[0] - target[0]).powi(2)
                    + (color[1] - target[1]).powi(2)
                    + (color[2] - target[2]).powi(2))
                    .sqrt();
                self.vols[self.ids[i][j]] = (self.vols[self.ids[i][j]] - 1.0).max(0.0);
                self.delivered.push(color);
            }
            Action::Discard { i, j } => {
                // ここに絵の具を破棄するロジックを書く
                // (i, j) の色を削除

                self.vols[self.ids[i][j]] = (self.vols[self.ids[i][j]] - 1.0).max(0.0);
            }
            Action::Toggle { i1, j1, i2, j2 } => {
                // ここに壁を切り替えるロジックを書く
                // wall_v と wall_h を更新

                if i1 == i2 {
                    let i = i1;
                    let j = j1.min(j2);
                    self.wall_v[i][j] ^= true;
                } else {
                    let i = i1.min(i2);
                    let j = j1;
                    self.wall_h[i][j] ^= true;
                }
                let (id, ids, caps) = get_ids(&self.wall_v, &self.wall_h);

                if self.ids[i1][j1] == self.ids[i2][j2] && ids[i1][j1] != ids[i2][j2] {
                    // split

                    let id1 = ids[i1][j1];
                    let id2 = ids[i2][j2];
                    let v = self.vols[self.ids[i1][j1]];
                    let mut vols = vec![0.0; id];
                    let mut colors = vec![[0.0; 3]; id];
                    for i in 0..self.input.n {
                        for j in 0..self.input.n {
                            vols[ids[i][j]] = self.vols[self.ids[i][j]];
                            colors[ids[i][j]] = self.colors[self.ids[i][j]];
                        }
                    }
                    vols[id1] = v * caps[id1] as f64 / (caps[id1] + caps[id2]) as f64;
                    vols[id2] = v * caps[id2] as f64 / (caps[id1] + caps[id2]) as f64;
                    self.ids = ids;
                    self.caps = caps;
                    self.vols = vols;
                    self.colors = colors;
                } else {
                    // merge

                    let id = ids[i1][j1];
                    let id1 = self.ids[i1][j1];
                    let id2 = self.ids[i2][j2];
                    let v1 = self.vols[id1];
                    let v2 = self.vols[id2];
                    let c1 = self.colors[id1];
                    let c2 = self.colors[id2];
                    let mut vols = vec![0.0; id];
                    let mut colors = vec![[0.0; 3]; id];
                    for i in 0..self.input.n {
                        for j in 0..self.input.n {
                            vols[ids[i][j]] = self.vols[self.ids[i][j]];
                            colors[ids[i][j]] = self.colors[self.ids[i][j]];
                        }
                    }
                    vols[id] = v1 + v2;
                    colors[id] = mix(v1, c1, v2, c2);
                    self.ids = ids;
                    self.caps = caps;
                    self.vols = vols;
                    self.colors = colors;
                }
            }
        }
    }

    fn gen_w(&mut self) -> Vec<Vec<f64>> {
        let mut res = vec![];
        let alpha = vec![1.0; self.input.k];
        for i in 0..4000 {
            let dir = Dirichlet::new(&alpha).unwrap();
            let w = dir.sample(&mut self.rng);
            res.push(w);
        }
        let alpha = vec![0.3; self.input.k];
        for j in 0..2000 {
            let dir = Dirichlet::new(&alpha).unwrap();
            let w = dir.sample(&mut self.rng);
            res.push(w);
        }
        let alpha = vec![3.0; self.input.k];
        for j in 0..2000 {
            let dir = Dirichlet::new(&alpha).unwrap();
            let w = dir.sample(&mut self.rng);
            res.push(w);
        }
        res
    }

    fn get_mi_vol(&mut self, lambda: usize, targets: &Vec<Vec<usize>>, pos_idx: usize) -> Vec<i64> {
        // 乱択で色の配分を決める
        let ww = self.gen_w();
        let mut mi = f64::MAX;
        let mut mi_vol = vec![0; self.input.k];
        for w in &ww {
            let mut vol = vec![0; self.input.k];
            let mut r = vec![(0.0, 0); self.input.k];
            for j in 0..self.input.k {
                vol[j] = (lambda as f64 * w[j]).floor() as i64;
                r[j] = ((lambda as f64 * w[j]) - vol[j] as f64, j);
            }
            r.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            r.reverse();
            for j in 0..lambda - vol.iter().sum::<i64>() as usize {
                vol[r[j].1] += 1;
            }

            let mut nowv = vol[0];
            let mut nowcol = self.input.own[0];
            for j in 1..self.input.k {
                let nxtv = nowv + vol[j];
                let nxtcol = mix(nowv as f64, nowcol, vol[j] as f64, self.input.own[j]);
                nowv = nxtv;
                nowcol = nxtcol;
            }

            let mut error = 0.0;
            for target in &targets[pos_idx] {
                error += distance(nowcol, self.input.target[*target]);
            }
            if error < mi {
                mi = error;
                mi_vol = vol.clone();
            }
        }
        mi_vol
    }

    fn solve(&mut self) -> (Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Action>) {
        let mut wall_v = self.wall_v.clone();
        let mut wall_h = self.wall_h.clone();


        let mut actions = vec![];

        // lambda * batch_cnt == input.h
        let mut batch_cnt = self.input.h / self.lambda;
        let mut pallet_cnt = self.input.n * self.input.n / self.lambda;
        assert!(self.lambda * batch_cnt == self.input.h, "lambda * batch_cnt must equal input.h");

        let mut pos = vec![(usize::MAX, usize::MAX); self.input.h];
        let mut pos_cnt = vec![0; pallet_cnt];
        let mut targets = vec![vec![]; pallet_cnt];


        for i in 0..self.input.h {
            if pos[i] == (usize::MAX, usize::MAX) {
                // クラスタを決める
                let mut v = vec![(0.0, i)];
                for j in 1..250 { // n*n 先までみる
                    if i + j >= self.input.h {
                        break;
                    }
                    if pos[i+j] != (usize::MAX, usize::MAX) {
                        continue;
                    }
                    let d = distance(self.input.target[i], self.input.target[i+j]);
                    v.push((d, i + j));
                }
                v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                // pos を決める。空いている場所を使う
                let mut pos_idx = 0;
                for k in 0..pallet_cnt {
                    if pos_cnt[k] == 0 {
                        pos_idx = k;
                        break;
                    }
                }
                let (r, c) = (pos_idx / (self.input.n / self.lambda), pos_idx % (self.input.n / self.lambda) * self.lambda);
                for j in 0..self.lambda {
                    let idx = v[j].1;
                    pos[idx] = (r, c);
                    pos_cnt[pos_idx] += 1;
                    targets[pos_idx].push(idx);
                }

                let mi_vol = self.get_mi_vol(self.lambda, &targets, pos_idx);
                // eprintln!("{:?}", mi_vol);
                // eprintln!("{:?}", targets[pos_idx]);

                for j in 0..self.input.k {
                    let vol = mi_vol[j];
                    for _ in 0..vol {
                        // Add action
                        actions.push(Action::Add {
                            i: pos_idx / (self.input.n / self.lambda),
                            j: pos_idx % (self.input.n / self.lambda) * self.lambda,
                            k: j,
                        });
                        self.apply(Action::Add {
                            i: pos_idx / (self.input.n / self.lambda),
                            j: pos_idx % (self.input.n / self.lambda) * self.lambda,
                            k: j,
                        });
                    }
                }
            }
            // i番目を届ける
            let (r, c) = pos[i];
            let pos_idx = r * (self.input.n / self.lambda) + c / self.lambda;

            // eprintln!("{} {}", r, c);
            // eprintln!("pos_cnt: {}", pos_cnt[pos_idx]);
            // eprintln!("vol: {}", self.vols[self.ids[r][c]]);

            actions.push(Action::Deliver {
                i: r,
                j: c,
            });
            self.apply(Action::Deliver {
                i: r,
                j: c,
            });
            pos_cnt[pos_idx] -= 1;

            // pos_cnt がゼロになったら掃除する
            if pos_cnt[pos_idx] == 0 {
                targets[pos_idx].clear();
                // Discard action
                while self.vols[self.ids[r][c]] > 1.0 {
                    actions.push(Action::Discard {
                        i: r,
                        j: c,
                    });
                    self.apply(Action::Discard {
                        i: r,
                        j: c,
                    });
                }
            }
        }

        (wall_v, wall_h, actions)
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

    let input = input();
    let n = input.n;
    let lambda = 5;

    let mut wall_v = vec![vec![false; n - 1]; n];
    let mut wall_h = vec![vec![false; n]; n - 1];

    for i in 0..n {
        for j in 1..(n / lambda) {
            wall_v[i][j*lambda - 1] = true;
        }
    }

    for i in 0..n-1 {
        for j in 0..n {
            wall_h[i][j] = true;
        }
    }
    let mut state = State::new(input, &wall_v, &wall_h, lambda);
    let (wall_v, wall_h, actions) = state.solve();
    output(&wall_v, &wall_h, &actions);
}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};
use std::str::FromStr;
use std::thread::Thread;
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

