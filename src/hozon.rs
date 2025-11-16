#![allow(unused)]

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
        let mut first = true;
        for j in 0..v.len()-1 {
            let (r, c, dir) = v[j];
            // col: r * n + c
            // q: i
            // a(c, q): colとおなじ
            // s(c, q): i
            // d(c, q): dir

            let col = r * n + c;
            let (nr, nc, ndir) = v[j+1];
            if i > 0 && first {
                ans.push(((col, i-1), (col, i, dir, r, c, nr, nc)));
                first = false;
            } else {
                ans.push(((col, i), (col, i, dir, r, c, nr, nc)));
            }
        }
    }


    let mut turn_mp = vec![vec![BTreeMap::new(); n]; n];
    let mut turn_id = vec![vec![0; n]; n];
    turn_mp[pos[0].0][pos[0].1].insert(0, 0);
    turn_id[pos[0].0][pos[0].1] += 1;
    for &((col, q), (a, s, d, r, c, nr, nc)) in &ans {
        // eprintln!("({}, {}), ({}, {}, {}, {}, {}, {}, {})", col, q, a, s, d, r, c, nr, nc);
        // eprintln!("{} {:?} {:?}", q, turn_mp[r][c], turn_id[r][c]);
        if !turn_mp[r][c].contains_key(&q) {
            turn_mp[r][c].insert(q, turn_id[r][c]);
            turn_id[r][c] += 1;
        }
        // eprintln!("{} {:?} {:?}", s, turn_mp[nr][nc], turn_id[nr][nc]);
        if !turn_mp[nr][nc].contains_key(&s) {
            turn_mp[nr][nc].insert(s, turn_id[nr][nc]);
            turn_id[nr][nc] += 1;
        }
    }


    // init_col を 全部0 で初期化する
    // ans を後ろから順にみて、(col, q) から (a, s, d) が決まるように色を決める
    // (col, q) の組をキーとする map を持って、キーが被ったら col を増やして登録

    let mut init_col = vec![vec![0; n]; n];
    let mut col_mp = BTreeMap::new();
    let mut col_id = BTreeMap::new();
    for i in (0..ans.len()).rev() {
        let ((col, q), (a, s, d, r, c, nr, nc)) = ans[i];
        let q = turn_mp[r][c][&q];
        let s = turn_mp[nr][nc][&s];
        
        let ncol = init_col[r][c];
        let nowcol = col_id.entry(q).or_insert(0);
        if col_mp.contains_key(&(*nowcol, q)) {
            let val = col_mp[&(*nowcol, q)];
            if val != (ncol, s, d) {
                *nowcol += 1;
            }
            col_mp.insert((*nowcol, q), (ncol, s, d));
            init_col[r][c] = *nowcol;
            ans[i] = ((*nowcol, q), (ncol, s, d, r, c, nr, nc));
            // eprintln!("({}, {}), ({}, {}, {}, {}, {}, {}, {})", *nowcol, q, ncol, s, d, r, c, nr, nc);
            // eprintln!("{:?}", ans[i]);
            // eprintln!();
        } else {
            col_mp.insert((*nowcol, q), (ncol, s, d));
            init_col[r][c] = *nowcol;
            ans[i] = ((*nowcol, q), (ncol, s, d, r, c, nr, nc));
            // eprintln!("({}, {}), ({}, {}, {}, {}, {}, {}, {})", *nowcol, q, ncol, s, d, r, c, nr, nc);
            // eprintln!("{:?}", ans[i]);
            // eprintln!();
        }
    }

    // for i in (0..ans.len()).rev() {
    //     let ((col, q), (a, s, d, r, c, nr, nc)) = ans[i];
    //     eprintln!("{:?}", ans[i]);
    //     if col == 220 {
    //         panic!();
    //     }
    // }

    // let mut col_mp = HashMap::new();
    // let mut col_id = 0;
    // for &((col, q), (a, s, d, r, c, nr, nc)) in &ans {
    //     if !col_mp.contains_key(&col) {
    //         col_mp.insert(col, col_id);
    //         col_id += 1;
    //     }
    // }

    // eprintln!("{:?}", col_id);

    let mut c = 0;
    for (_, &v) in &col_id {
        c = c.max(v);
    }
    c += 1;

    let mut q = 0;
    for i in 0..n {
        for j in 0..n {
            q = q.max(turn_id[i][j]);
        }
    }

    // let mut init_col = vec![vec![0; n]; n];
    // for i in 0..n {
    //     for j in 0..n {
    //         let &mut col = col_mp.entry(i*n+j).or_insert(0);
    //         init_col[i][j] = col;
    //     }
    // }

    let mut ans_st = BTreeSet::new();
    let mut ans_out = vec![];
    for ((col, q), (a, s, d, r, c, nr, nc)) in ans {
        // let q = turn_mp[r][c][&q];
        // let s = turn_mp[nr][nc][&s];
        if ans_st.contains(&(col, q)) {
            continue;
        }
        ans_out.push((col, q, a, s, d));
        ans_st.insert((col, q));
    }


    println!("{} {} {}", c, q, ans_st.len());
    for i in 0..n {
        for j in 0..n {
            print!("{}", init_col[i][j]);
            if j == n-1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
    // eprintln!("{:?}", &turn_mp[12][10]);
    // eprintln!("{:?}", &turn_mp[12][11]);

    // eprintln!("{:?}", col_id);
    

    for (col, q, a, s, d) in ans_out {
        println!("{} {} {} {} {}", col, q, a, s, d);
    }


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

