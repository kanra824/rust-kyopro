#![allow(unused)]
use std::collections::*;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Isize1;
use proconio::marker::Usize1;
use proconio::source::line::LineSource;
use std::cmp::max;
use std::cmp::min;
use std::fmt;
use std::io::BufReader;
use std::io::Read;
use std::io::Stdin;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::ops;
use std::str::FromStr;

fn solve() {
    input! {
        p: i64,
        a: i64,
        b: i64,
        s: i64,
        g: i64,
    }

    if s == g {
        pr(0);
        return;
    }

    if a == 0 {
        if b == g {
            pr(1);
        } else {
            pr(-1);
        }
        return;
    }

    Modint::set_p(p);

    let mut inva = Modint::new(a).inv();
    let mut invb = inva * Modint::new(-b);

    let mut biga = Modint::new(1);
    let mut bigb = Modint::new(0);
    let mut m = 1;
    while m * m < p {
        m += 1;
    }

    let mut small = BTreeMap::new();
    let mut crr = Modint::new(g);
    for i in 0..m {
        if !small.contains_key(&crr.x) {
            small.insert(crr.x, i);
        }
        crr = crr * inva + invb;
        biga *= a;
        bigb *= a;
        bigb += b;
    }

    let mut crr = Modint::new(s);
    for i in 0..p/m+1 {
        if small.contains_key(&crr.x) {
            pr(i * m + small[&crr.x]);
            return;
        }
        crr = crr * biga + bigb;
    }
    pr(-1);


}

fn main() {
    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        // from &mut source,
        t: usize,
    }


    for _ in 0..t {
        solve();
    }

}

pub const MOD998244353: i64 = 998244353;

static mut MINT_MOD: i64 = 998244353;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint {
    pub x: i64,
    pub p: i64,
}

impl std::fmt::Display for Modint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl Modint {
    fn get_p() -> i64 {
        unsafe { MINT_MOD }
    }

    pub fn set_p(p: i64) {
        unsafe {
            MINT_MOD = p;
        }
    }

    pub fn new(x: i64) -> Self {
        let p = Modint::get_p();
        if x >= 0 {
            Modint { x: x % p, p }
        } else {
            let tmp = x.abs() % p;
            let val = x + tmp * p;
            Modint { x: (val + p) % p, p }
        }
    }

    pub fn pow(&self, mut k: i64) -> Self {
        let mut mul = Modint::new(self.x);
        let mut res = Modint::new(1);
        while k > 0 {
            if k & 1 == 1 {
                res = res * mul;
            }
            mul = mul * mul;
            k /= 2;
        }
        res
    }

    pub fn inv(&self) -> Self {
        if self.x == 0 {
            panic!("0 has no inv");
        }
        self.pow((self.p - 2) as i64)
    }
}

impl std::ops::Neg for Modint {
    type Output = Modint;

    fn neg(mut self) -> Modint {
        self.x = (self.p - self.x);
        if self.x >= self.p {
            self.x -= self.p;
        }
        self
    }
}

impl std::ops::Add<Self> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: Self) -> Modint {
        self + rhs.x
    }
}

impl std::ops::Add<i64> for Modint {
    type Output = Modint;

    fn add(mut self, rhs: i64) -> Modint {
        self.x = (self.x + rhs % self.p) % self.p;
        self
    }
}

impl std::ops::Sub<Self> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: Self) -> Modint {
        self - rhs.x
    }
}

impl std::ops::Sub<i64> for Modint {
    type Output = Modint;

    fn sub(mut self, rhs: i64) -> Modint {
        self.x = (self.x + self.p - rhs % self.p) % self.p;
        self
    }
}

impl std::ops::Mul<Self> for Modint {
    type Output = Modint;
    fn mul(mut self, rhs: Self) -> Modint {
        self * rhs.x
    }
}

impl std::ops::Mul<i64> for Modint {
    type Output = Modint;
    fn mul(mut self, mut rhs: i64) -> Modint {
        rhs %= self.p;
        self.x = self.x * rhs % self.p;
        self
    }
}

impl std::ops::Div<Self> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: Self) -> Modint {
        self / rhs.x
    }
}

impl std::ops::Div<i64> for Modint {
    type Output = Modint;
    fn div(mut self, rhs: i64) -> Modint {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::new(rhs).inv()
    }
}

impl std::ops::Rem<Self> for Modint {
    // implement only for num_traits::Numstd::ops
    type Output = Modint;
    fn rem(mut self, rhs: Self) -> Modint {
        panic!("cannot rem");
    }
}


impl std::ops::AddAssign<Self> for Modint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::AddAssign<i64> for Modint {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Self> for Modint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::SubAssign<i64> for Modint {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign<Self> for Modint {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<i64> for Modint {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign<Self> for Modint {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<i64> for Modint {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}




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


