#![allow(unused)]

use std::time::{Instant, SystemTime};

#[derive(Debug)]
struct XorShift {
    w: u32,
    x: u32,
    y: u32,
    z: u32,
}

impl XorShift {
    fn new() -> Self {
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let seed = d.as_secs() as u32;
        Self::from_seed(seed)
    }

    fn from_seed(seed: u32) -> Self {
        let w = seed;
        let x = w << 13;
        let y = (w >> 9) ^ (x << 6);
        let z = y >> 7;
        Self { w, x, y, z }
    }

    fn rand(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19) ^ (t ^ (t >> 8)));
        self.w
    }

    // [min, max] のu32乱数
    fn rand_u32(&mut self, min: u32, max: u32) -> u32 {
        self.rand() % (max - min + 1) + min
    }

    // [min, max] のf64乱数
    fn rand_double(&mut self, min: f64, max: f64) -> f64 {
        (self.rand() % 0xFFFF) as f64 / (0xFFFF as f64 * (max - min) + min)
    }
}


fn get_path(n: usize, from: (usize, usize), to: (usize, usize), d: &Vec<Vec<i64>>, g: &Vec<Vec<usize>>) -> Vec<(usize, usize, char)> {
    let mut now = from;
    let mut res = vec![];
    while now != to {
        let (r, c) = now;
        let mut mi = i64::MAX;
        let mut miidx = (usize::MAX, usize::MAX);
        for &nxt in &g[r*n+c] {
            let nr = nxt / n;
            let nc = nxt % n;
            let nowd = d[nr*n+nc][to.0*n+to.1];
            if nowd < mi {
                mi = nowd;
                miidx = (nr, nc);
            }
        }

        let dir = if c == miidx.1 + 1 {
            'L'
        } else if c + 1 == miidx.1 {
            'R'
        } else if r == miidx.0 + 1 {
            'U'
        } else {
            'D'
        };
        
        res.push((now.0, now.1, dir));
        now = miidx;
    }
    res.push((now.0, now.1, 'S'));
    res
}

fn dir_to_num(dir: char) -> i32 {
    if dir == 'L' {
        0
    } else if dir == 'U' {
        1
    } else if dir == 'R' {
        2
    } else if dir == 'D' {
        3
    } else {
        panic!();
    }
}

fn next_cq(mut c: i32, mut q: i32, mut x: i32) -> (i32, i32, i32) {
    if q == x {
        if c == x {
            q -= 1;
        } else {
            c += 1;
        }
    } else if q == 0 {
        x += 1;
        q = x;
        c = 0;
    } else {
        q -= 1;
    }
    (c, q, x)
}

fn next_cq2(mut c: usize, mut q: usize, sz_q: usize) -> (usize, usize) {
    q += 1;
    if q == sz_q {
        q = 0;
        c += 1;
    }
    (c, q)
}

#[derive(Debug, Clone)]
struct State {
    score: (usize, usize),
    init_col: Vec<Vec<usize>>,
    qv: Vec<Vec<usize>>,
    mp: BTreeMap<(usize, usize), (usize, usize, char)>,
    mp_rev: BTreeMap<(usize, usize, char), (usize, usize)>,
    ans_st: BTreeSet<(usize, usize, usize, usize, char)>,
    q_col: Vec<usize>,
    q_ma: usize,
    id: (usize, usize),
    used_id: BTreeSet<(usize, usize)>,
}

impl State {
    fn new(n: usize) -> Self {
        State {
            score: (usize::MAX, usize::MAX),
            init_col: vec![],
            qv: vec![],
            mp: BTreeMap::new(),
            mp_rev: BTreeMap::new(),
            ans_st: BTreeSet::new(),
            q_col: vec![],
            q_ma: usize::MAX,
            id: (usize::MAX, usize::MAX),
            used_id: BTreeSet::new(),
        }
    }
}

fn main() {
    let start = Instant::now();
    // // AOJ, codeforces, etc...
    // let mut s = String::new();
    // let stdin = stdin();
    // let mut reader = Reader::new(&mut s, stdin);

    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        n: usize,
        k: usize,
        t: usize,
        v: [Chars; n],
        h: [Chars; n-1],
        pos: [(usize, usize); k],
    }

    // 普通に位置と座標
    // col だけ圧縮 -> 19265
    // state も圧縮 -> 22620
    // q 探索 -> 提出:2870, ローカル:4588


    let mut g = vec![vec![]; n*n];
    let mut d = vec![vec![i64::MAX;n*n];n*n];
    for i in 0..n*n {
        d[i][i] = 0;
    }
    for i in 0..n {
        for j in 0..n-1 {
            if v[i][j] == '0' {
                g[i*n+j].push(i*n+j+1);
                g[i*n+j+1].push(i*n+j);
                d[i*n+j][i*n+j+1] = 1;
                d[i*n+j+1][i*n+j] = 1;
            }
        }
    }

    for i in 0..n-1 {
        for j in 0..n {
            if h[i][j] == '0' {
                g[i*n+j].push((i+1)*n+j);
                g[(i+1)*n+j].push(i*n+j);
                d[i*n+j][(i+1)*n+j] = 1;
                d[(i+1)*n+j][i*n+j]= 1;
            }
        }
    }

    for k in 0..n*n {
        for i in 0..n*n {
            for j in 0..n*n {
                if d[i][k] != i64::MAX && d[k][j] != i64::MAX {
                    d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                }
            }
        }
    }

    let mut ans = Vec::new();
    for i in 0..k-1 {
        let v = get_path(n, pos[i], pos[i+1], &d, &g);
        for j in 0..v.len()-1 {
            let (r, c, dir) = v[j];
            let (nr, nc, ndir) = v[j+1];
            ans.push((dir, r, c, nr, nc));
        }
    }

    let mut ans_st = BTreeSet::new();
    {
        let mut init_col = vec![vec![0; n]; n];
        let mut mp = BTreeMap::new(); // (c, q) -> (a, s, d);
        let mut mp_rev = BTreeMap::new();
        let mut id = (0, 1, 1); // (c, q, x), (0, 0) は最後に使うので、(0, 1) から始める
        let mut qv = vec![vec![0; n]; n];
        for i in (0..ans.len()).rev() {
            let (dir, r, c, nr, nc) = ans[i];
            let ncol = init_col[r][c];
            let nq = qv[nr][nc];
            let (col, q, x) = id;
            if i == 0 {
                // (0, 0) を使用
                ans_st.insert((0, 0, ncol, nq, dir));
                init_col[r][c] = 0;
            } else if mp_rev.contains_key(&(ncol, nq, dir)) {
                let (col, q) = mp_rev[&(ncol, nq, dir)];
                init_col[r][c] = col;
                qv[r][c] = q;
                ans_st.insert((col, q, ncol, nq, dir));
            } else {
                mp.insert((col, q), (ncol, nq, dir));
                mp_rev.insert( (ncol, nq, dir), (col, q));
                init_col[r][c] = col;
                qv[r][c] = q;
                ans_st.insert((col, q, ncol, nq, dir));
                id = next_cq(col, q, x);
            }
        }
    }

    let mut sz_c = 0;
    let mut sz_q = 0;
    for &(col, q, a, s, d) in &ans_st {
        sz_c = sz_c.max(col);
        sz_c = sz_c.max(a);
        sz_q = sz_q.max(q);
        sz_q = sz_q.max(s);
    }
    sz_c += 1;
    sz_q += 1;

    // let sz_c_mi = sz_c;
    // let sz_c_ma = sz_c;
    // let sz_q_mi = sz_q;
    // let sz_q_ma = sz_q;

    let sz_c_mi_mul = 7;
    let sz_c_ma_mul = 8;
    let sz_c_mi = sz_c * sz_c_mi_mul / 10;
    let sz_c_ma = sz_c * sz_c_ma_mul / 10;
    let sz_q_mi = sz_q / 2;
    let sz_q_ma = sz_q * 7 / 10;

    let mut score = usize::MAX;
    let mut ans_sz_c = 0;
    let mut ans_sz_q = 0;
    let mut ans_st_mi = BTreeSet::new();
    let mut init_col_mi = vec![];
    let mut rng = XorShift::new();
    'outer: for sz_c_mul in (sz_c_mi_mul..=sz_c_ma_mul).rev() {
        let sz_c = sz_c * sz_c_mul / 10;
        for sz_q in sz_q_mi..=sz_q_ma {
            if start.elapsed() >= std::time::Duration::from_millis(19700) {
                break 'outer;
            }
            let mut beam_width = 1;
            let mut beam = vec![vec![State::new(n); beam_width]; ans.len() + 1];
            beam[0][0] = State {
                score: (0, 1),
                init_col: vec![vec![0; n]; n],
                qv: vec![vec![0; n]; n],
                mp: BTreeMap::new(),
                mp_rev: BTreeMap::new(),
                ans_st: BTreeSet::new(),
                q_col: vec![0; sz_q as usize],
                q_ma: 0,
                id: (0, 1),
                used_id: BTreeSet::new(),
            };
            beam[0][0].q_col[0] = 1;

            for i in (0..ans.len()).rev() {
                let beam_idx = ans.len() - i - 1;
                // eprintln!("{:?}", beam[beam_idx][0].score);
                // eprintln!();
                for j in 0..beam_width {
                    if beam[beam_idx][j].score.0 == usize::MAX {
                        continue;
                    }
                    let (dir, r, c, nr, nc) = ans[i];
                    let ncol = beam[beam_idx][j].init_col[r][c];
                    let nq = beam[beam_idx][j].qv[nr][nc];

                    if i == 0 {
                        let mut state = beam[beam_idx][j].clone();
                        state.ans_st.insert((0, 0, ncol, nq, dir));
                        state.score = (state.score.0 + 1, state.q_col[0]);
                        state.init_col[r][c] = 0;
                        if beam[beam_idx+1][0].score > state.score {
                            beam[beam_idx+1][0] = state;
                        // } else if beam[beam_idx+1][1].score > state.score {
                        //     beam[beam_idx+1][1] = state;
                        }
                    } else if beam[beam_idx][j].mp_rev.contains_key(&(ncol, nq, dir)) {
                        let mut state = beam[beam_idx][j].clone();
                        let (col, q) = state.mp_rev[&(ncol, nq, dir)];
                        state.init_col[r][c] = col;
                        state.qv[r][c] = q;
                        // state.ans_st.insert((col, q, ncol, nq, dir));
                        if beam[beam_idx+1][0].score > state.score {
                            beam[beam_idx+1][0] = state;
                        // } else if beam[beam_idx+1][1].score > state.score {
                        //     beam[beam_idx+1][1] = state;
                        }
                    } else {
                        let mut  repeat = false;
                        let mut state = beam[beam_idx][j].clone();
                        for nowq in 0..=state.q_ma {
                            if state.q_col[nowq] >= sz_c as usize - 1 {
                                continue;
                            }
                            let mut update_v = vec![];
                            let mut update_ans = vec![];
                            let col = state.q_col[nowq];
                            state.q_col[nowq] += 1;
                            update_v.push((r, c, state.init_col[r][c], state.qv[r][c]));
                            update_ans.push((col, nowq, ncol, nq, dir));
                            state.init_col[r][c] = col;
                            state.qv[r][c] = nowq;
                            state.used_id.insert((col, nowq));
                            state.mp.insert((col, nowq), (ncol, nq, dir));
                            state.mp_rev.insert((ncol, nq, dir), (col, nowq));
                            state.ans_st.insert((col, nowq, ncol, nq, dir));
                            let prev_score = state.score;
                            state.score = (state.score.0 + 1, state.q_col[nowq]);
                            let mut cnt = 0;
                            let mut idx = i-1;
                            while idx > 0 {
                                let (dir, r, c, nr, nc) = ans[idx];
                                let ncol = state.init_col[r][c];
                                let nq = state.qv[nr][nc];
                                if state.mp_rev.contains_key(&(ncol, nq, dir)) {
                                    let (col, q) = state.mp_rev[&(ncol, nq, dir)];
                                    update_v.push((r, c, state.init_col[r][c], state.qv[r][c]));
                                    state.init_col[r][c] = col;
                                    state.qv[r][c] = q;
                                    cnt += 1;
                                } else {
                                    break;
                                }
                                idx -= 1;
                            }
                            if cnt > 0 {
                                repeat = true;
                            }

                            // beam 更新
                            let beam_nidx = ans.len() - idx - 1;
                            if beam[beam_nidx][0].score > state.score {
                                beam[beam_nidx][0] = state.clone();
                            } else if beam[beam_nidx][0].score == state.score {
                                let p = rng.rand_double(0.0, 0.1);
                                if p < 0.5 {
                                    beam[beam_nidx][0] = state.clone();
                                }
                            }

                            for j in (0..update_v.len()).rev() {
                                let (r, c, col, q) = update_v[j];
                                state.init_col[r][c] = col;
                                state.qv[r][c] = q;
                            }

                            for j in (0..update_ans.len()).rev() {
                                let (col, nowq, ncol, nq, dir) = update_ans[j];
                                state.used_id.remove(&(col, nowq));
                                state.mp.remove(&(col, nowq));
                                state.mp_rev.remove(&(ncol, nq, dir));
                                state.ans_st.remove(&(col, nowq, ncol, nq, dir));
                            }
                            state.score = prev_score;

                            state.q_col[nowq] -= 1;

                        }

                        // 新 id を使用
                        if !repeat {
                            while state.used_id.contains(&state.id) {
                                state.id = next_cq2(state.id.0, state.id.1, sz_q as usize);
                            }
                            state.q_col[state.id.1] = state.id.0 + 1;
                            state.q_ma = state.q_ma.max(state.id.1);
                            let (col, q) = state.id;
                            state.used_id.insert((col, q));
                            state.mp.insert((col, q), (ncol, nq, dir));
                            state.mp_rev.insert((ncol, nq, dir), (col, q));
                            state.init_col[r][c] = col;
                            state.qv[r][c] = q;
                            state.ans_st.insert((col, q, ncol, nq, dir));
                            state.score = (state.score.0 + 1, state.q_col[state.id.1]);

                            if beam[beam_idx + 1][0].score > state.score {
                                beam[beam_idx+1][0] = state;
                            }
                        }
                    }
                }
            }

            let mut ans_c = 0;
            let mut ans_q = 0;
            for &(col, q, a, s, d) in &beam[beam.len()-1][0].ans_st {
                ans_c = ans_c.max(col);
                ans_c = ans_c.max(a);
                ans_q = ans_q.max(q);
                ans_q = ans_q.max(s);
            }
            ans_c += 1;
            ans_q += 1;
            if score > ans_c + ans_q {
                ans_sz_c = ans_c;
                ans_sz_q = ans_q;
                ans_st_mi = beam[beam.len()-1][0].ans_st.clone();
                init_col_mi = beam[beam.len()-1][0].init_col.clone();
            }

        }
    }



    // let mut score = usize::MAX;
    // let mut ans_sz_c = 0;
    // let mut ans_sz_q = 0;
    // let mut ans_st_mi = BTreeSet::new();
    // let mut init_col_mi = vec![];
    // let mut rng = XorShift::new();
    // 'outer: for sz_c in (sz_c_mi..=sz_c_ma).rev() {
    //     for sz_q in sz_q_mi..=sz_q_ma {
    //         if start.elapsed() >= std::time::Duration::from_millis(1970) {
    //             break 'outer;
    //         }

    //         // sz_c, sz_q を基準に、できるだけ使いまわせるように作り直す
    //         let mut ans_st = BTreeSet::new();
    //         let mut init_col = vec![vec![0; n]; n];
    //         let mut qv = vec![vec![0; n]; n];
    //         let mut mp = BTreeMap::new(); // (c, q) -> (a, s, d);
    //         let mut mp_rev = BTreeMap::new();
    //         let mut q_col = vec![0; sz_q as usize];
    //         q_col[0] = 1;
    //         let mut q_ma = 0;

    //         let mut id = (0, 1);
    //         let mut used_id = BTreeSet::new();


    //         for i in (0..ans.len()).rev() {
    //             let (dir, r, c, nr, nc) = ans[i];
    //             let ncol = init_col[r][c];
    //             let nq = qv[nr][nc];

    //             if i == 0 {
    //                 // (0, 0) を使用
    //                 ans_st.insert((0, 0, ncol, nq, dir));
    //                 init_col[r][c] = 0;
    //             } else if mp_rev.contains_key(&(ncol, nq, dir)) {
    //                 let (col, q) = mp_rev[&(ncol, nq, dir)];
    //                 init_col[r][c] = col;
    //                 qv[r][c] = q;
    //                 ans_st.insert((col, q, ncol, nq, dir));
    //             } else {
    //                 // q を調べる
    //                 let mut ma = (0, 0); // (繰り返し回数, -q_col)
    //                 let mut maq_v = vec![];
    //                 for nowq in 0..=q_ma {
    //                     if q_col[nowq] >= sz_c as usize - 1 {
    //                         continue;
    //                     }
    //                     let mut update_v = vec![];
    //                     let col = q_col[nowq];
    //                     update_v.push((r, c, init_col[r][c], qv[r][c]));
    //                     init_col[r][c] = col;
    //                     qv[r][c] = nowq;
    //                     let mut cnt = 0;
    //                     let mut idx = i-1;
    //                     while idx > 0 {
    //                         let (dir, r, c, nr, nc) = ans[idx];
    //                         let ncol = init_col[r][c];
    //                         let nq = qv[nr][nc];
    //                         if mp_rev.contains_key(&(ncol, nq, dir)) {
    //                             let (col, q) = mp_rev[&(ncol, nq, dir)];
    //                             update_v.push((r, c, init_col[r][c], qv[r][c]));
    //                             init_col[r][c] = col;
    //                             qv[r][c] = q;
    //                             cnt += 1;
    //                         } else {
    //                             break;
    //                         }
    //                         idx -= 1;
    //                     }

    //                     for j in (0..update_v.len()).rev() {
    //                         let (r, c, col, q) = update_v[j];
    //                         init_col[r][c] = col;
    //                         qv[r][c] = q;
    //                     }

    //                     if ma < (cnt, -(q_col[nowq] as i32)) {
    //                         ma = (cnt, -(q_col[nowq] as i32));
    //                         maq_v.clear();
    //                         maq_v.push(nowq);
    //                     } else if ma == (cnt, -(q_col[nowq] as i32)) {
    //                         maq_v.push(nowq);
    //                     }
    //                 }
    //                 // eprintln!("{} {}", i, macnt);

    //                 let (col, q) = if ma.0 == 0 {
    //                     while used_id.contains(&id) {
    //                         id = next_cq2(id.0, id.1, sz_q as usize);
    //                     }
    //                     q_col[id.1] = id.0 + 1;
    //                     q_ma = q_ma.max(id.1);
    //                     id
    //                 } else {
    //                     let mut maq_idx = rng.rand_u32(0, maq_v.len() as u32-1) as usize;
    //                     let maq = maq_v[maq_idx];
    //                     q_col[maq] += 1;
    //                     (q_col[maq] - 1, maq)
    //                 };
    //                 used_id.insert((col, q));


    //                 mp.insert((col, q), (ncol, nq, dir));
    //                 mp_rev.insert( (ncol, nq, dir), (col, q));
    //                 init_col[r][c] = col;
    //                 qv[r][c] = q;
    //                 ans_st.insert((col, q, ncol, nq, dir));
    //             }
    //         }

    //         let mut ans_c = 0;
    //         let mut ans_q = 0;
    //         for &(col, q, a, s, d) in &ans_st {
    //             ans_c = ans_c.max(col);
    //             ans_c = ans_c.max(a);
    //             ans_q = ans_q.max(q);
    //             ans_q = ans_q.max(s);
    //         }
    //         ans_c += 1;
    //         ans_q += 1;

    //         if score > ans_c + ans_q {
    //             score = ans_c + ans_q;
    //             ans_sz_c = ans_c;
    //             ans_sz_q = ans_q;
    //             ans_st_mi = ans_st;
    //             init_col_mi = init_col;
    //         }
    //     }
    // }

    // eprintln!("{} {} {} : {} {} {}", sz_q_mi, ans_sz_q, sz_q_ma, sz_c_mi, ans_sz_c, sz_c_ma);


    println!("{} {} {}", ans_sz_c, ans_sz_q, ans_st_mi.len());
    for i in 0..n {
        for j in 0..n {
            print!("{}", init_col_mi[i][j]);
            if j == n-1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }

    for (col, q, a, s, d) in ans_st_mi {
        println!("{} {} {} {} {}", col, q, a, s, d);
    }

    eprintln!("{:?}", start.elapsed())

}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};
use std::str::FromStr;
use std::{fmt, ops};
use rand::Rng;

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

