#![allow(unused)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Trie {
    cnt: i64,
    end: i64,
    child: Vec<Option<Rc<RefCell<Trie>>>>,
    par: Option<Weak<RefCell<Trie>>>,
    val: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            cnt: 0,
            end: 0,
            child: vec![None; 26],
            par: None,
            val: usize::MAX,
        }
    }

    pub fn new_child() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Trie::new()))
    }

    pub fn add_recursive(node: Rc<RefCell<Self>>, s: &Vec<char>, depth: usize) {
        node.borrow_mut().cnt += 1;
        if depth >= s.len() {
            node.borrow_mut().end += 1;
            return;
        }
        
        let idx = s[depth] as usize - 'a' as usize;
        if node.borrow().child[idx].is_none() {
            let new_child = Self::new_child();
            new_child.borrow_mut().val = idx;
            new_child.borrow_mut().par = Some(Rc::downgrade(&node));
            node.borrow_mut().child[idx] = Some(new_child);
        }
        
        let child = node.borrow().child[idx].as_ref().unwrap().clone();
        Self::add_recursive(child, s, depth + 1);
    }
    
    pub fn has_nth(&self, idx: usize) -> bool {
        !self.child[idx].is_none()
    }

    pub fn nxt(now: Rc<RefCell<Trie>>, idx: usize) -> Option<Rc<RefCell<Trie>>> {
        now.borrow().child[idx].clone()
    }

    // 最小共通接頭辞の長さの総和
    pub fn calc_lcp_sum(&self, s: &Vec<char>) -> i64 {
        let mut now = Rc::new(RefCell::new(self.clone()));
        let mut res = 0;
        let mut depth = 0;
        let mut prevcnt = now.borrow().cnt;
        for i in 0..s.len() {
            let idx = s[i] as usize - 'a' as usize;
            if now.borrow().child[idx].is_none() {
                res += now.borrow().cnt * depth;
                return res;
            }
            let nxt = now.borrow_mut().child[idx].as_ref().unwrap().clone();
            now = nxt;
            res += (prevcnt - now.borrow().cnt) * depth;
            depth += 1;
            prevcnt = now.borrow().cnt;
        }
        res += now.borrow().cnt * depth;
        res
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
        q: usize,
    }

    let mut trie = Rc::new(RefCell::new(Trie::new()));
    let mut v = vec![];
    for i in 0..n + 1 {
        v.push(trie.clone());
    }

    for _ in 0..q {
        input! {
            t: usize,
            p: Usize1,
        }
        if t == 1 {
            v[p] = v[n].clone();
        } else if t == 2 {
            input! {
                s: Chars,
            }
            Trie::add_recursive(v[p].clone(), &s, 0);
            for c in s {
                let idx = c as usize - 'a' as usize;
                let nxt = v[p].borrow_mut().child[idx].as_ref().unwrap().clone();
                v[p] = nxt;
            }
        } else {
            v[n] = v[p].clone();
        }
    }


    let mut ans = vec![];
    let mut now = v[n].clone();
    loop {
        let idx = now.borrow().val;
        if idx == usize::MAX {
            break;
        }
        let c = (idx + 'a' as usize) as u8 as char;
        ans.push(c);
        let par = now.borrow().par.clone();
        if par.is_none() {
            break;
        }
        now = par.unwrap().upgrade().unwrap();
    }
    ans.reverse();
    for c in ans {
        print!("{}", c)
    }
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
