#![allow(unused)]

// DLUR
static DR: [i64; 4] = [1, 0, -1, 0];
static DC: [i64; 4] = [0, -1, 0, 1];

#[derive(Clone, Debug)]
struct Rectangle {
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
}

impl Rectangle {
    fn null() -> Self {
        Rectangle {
            r1: usize::MAX,
            c1: usize::MAX,
            r2: usize::MAX,
            c2: usize::MAX,
        }
    }

    fn area(&self) -> i64 {
        ((self.r2 - self.r1) * (self.c2 - self.c1)) as i64
    }
}

#[derive(Clone, Debug)]
struct State {
    w: usize,
    rect: Vec<Vec<Rectangle>>, // rect[d][n]
    vcnt: Vec<Vec<Vec<i64>>>,  // vcnt[d][w][w-1]
    hcnt: Vec<Vec<Vec<i64>>>,  // hcnt[d][w-1][w]
    sel: Vec<Vec<Vec<bool>>>,  // sel[d][w][w]
    cost: i64,
}

impl State {
    fn new(w: usize, d: usize, n: usize, rng: &mut ChaCha20Rng) -> Self {
        let mut rect = vec![vec![Rectangle::null(); n]; d];
        let vcnt = vec![vec![vec![0; w + 1]; w]; d];
        let hcnt = vec![vec![vec![0; w]; w + 1]; d];
        let mut sel = vec![vec![vec![false; w]; w]; d];
        for i in 0..d {
            for j in 0..n {
                let mut r = rng.gen_range(0..w);
                let mut c = rng.gen_range(0..w);
                while sel[i][r][c] {
                    r = rng.gen_range(0..w);
                    c = rng.gen_range(0..w);
                }
                sel[i][r][c] = true;
                rect[i][j] = Rectangle {
                    r1: r,
                    c1: c,
                    r2: r + 1,
                    c2: c + 1,
                };
            }
        }

        let mut state = State {
            w,
            rect,
            vcnt,
            hcnt,
            sel,
            cost: 0,
        };

        for i in 0..d {
            state.sort_rect(i);
        }

        let rect = state.rect.clone();
        for i in 0..d {
            for j in 0..n {
                state.add_rect(i, &rect[i][j]);
            }
        }

        state
    }

    fn sort_rect(&mut self, d: usize) {
        self.rect[d].sort_by(|rect1, rect2| rect1.area().cmp(&rect2.area()))
    }

    fn update_vcnt(&mut self, d: usize, r: usize, c: usize, val: i64) {
        if val == 0 {
            return;
        }
        assert!(-1 <= val && val <= 1);
        self.vcnt[d][r][c] += val;
        let check_val = if val > 0 { 1 } else { 0 };
        let cost_val = 1 - check_val * 2; // 1 -> -1, 0 -> 1;
        if self.vcnt[d][r][c] == check_val && c != 0 && c != self.w {
            if d != 0 && self.vcnt[d - 1][r][c] > 0 {
                self.cost += cost_val;
            } else if d != 0 && self.vcnt[d - 1][r][c] == 0 {
                self.cost -= cost_val;
            }
            if d != self.vcnt.len() - 1 && self.vcnt[d + 1][r][c] > 0 {
                self.cost += cost_val;
            } else if d != self.vcnt.len() - 1 && self.vcnt[d + 1][r][c] == 0 {
                self.cost -= cost_val;
            }
        }
    }

    fn update_hcnt(&mut self, d: usize, r: usize, c: usize, val: i64) {
        if val == 0 {
            return;
        }
        assert!(-1 <= val && val <= 1);
        self.hcnt[d][r][c] += val;
        let check_val = if val > 0 { 1 } else { 0 };
        let cost_val = 1 - check_val * 2; // 1 -> -1, 0 -> 1;
        if self.hcnt[d][r][c] == check_val && r != 0 && r != self.w {
            if d != 0 && self.hcnt[d - 1][r][c] > 0 {
                self.cost += cost_val;
            } else if d != 0 && self.hcnt[d - 1][r][c] == 0 {
                self.cost -= cost_val;
            }
            if d != self.vcnt.len() - 1 && self.hcnt[d + 1][r][c] > 0 {
                self.cost += cost_val;
            } else if d != self.vcnt.len() - 1 && self.hcnt[d + 1][r][c] == 0 {
                self.cost -= cost_val;
            }
        }
    }

    fn expand(&mut self, w: usize, d: usize, n: usize, dir: usize, a: &Vec<Vec<i64>>) {
        let mut rect = self.rect[d][n].clone();

        let prev_area_cost = self.calc_area_cost(d, self.rect[0].len(), a);

        // DLUR
        let ok = match dir {
            0 => {
                // D
                let mut ok = true;
                if rect.r2 == w {
                    ok = false;
                }
                if ok {
                    for j in rect.c1..rect.c2 {
                        if self.sel[d][rect.r2][j] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    for j in rect.c1..rect.c2 {
                        self.sel[d][rect.r2][j] = true;
                    }
                    // 横
                    self.update_vcnt(d, rect.r2, rect.c1, 1);
                    self.update_vcnt(d, rect.r2, rect.c2, 1);
                    // 下消す
                    for j in rect.c1..rect.c2 {
                        self.update_hcnt(d, rect.r2, j, -1);
                    }
                    rect.r2 += 1;
                    // 下足す
                    for j in rect.c1..rect.c2 {
                        self.update_hcnt(d, rect.r2, j, 1);
                    }
                    // 更新
                    self.rect[d][n] = rect;
                }
                ok
            }
            1 => {
                // L
                let mut ok = true;
                if rect.c1 == 0 {
                    ok = false;
                }
                if ok {
                    for i in rect.r1..rect.r2 {
                        if self.sel[d][i][rect.c1 - 1] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    for i in rect.r1..rect.r2 {
                        self.sel[d][i][rect.c1 - 1] = true;
                    }
                    // 上下
                    self.update_hcnt(d, rect.r1, rect.c1 - 1, 1);
                    self.update_hcnt(d, rect.r2, rect.c1 - 1, 1);
                    // 左消す
                    for i in rect.r1..rect.r2 {
                        self.update_vcnt(d, i, rect.c1, -1);
                    }

                    rect.c1 -= 1;
                    // 左足す
                    for i in rect.r1..rect.r2 {
                        self.update_vcnt(d, i, rect.c1, 1);
                    }

                    // 更新
                    self.rect[d][n] = rect;
                }
                ok
            }
            2 => {
                // U
                let mut ok = true;
                if rect.r1 == 0 {
                    ok = false;
                }
                if ok {
                    for j in rect.c1..rect.c2 {
                        if self.sel[d][rect.r1 - 1][j] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    for j in rect.c1..rect.c2 {
                        self.sel[d][rect.r1 - 1][j] = true;
                    }
                    // 横
                    self.update_vcnt(d, rect.r1 - 1, rect.c1, 1);
                    self.update_vcnt(d, rect.r1 - 1, rect.c2, 1);
                    // 上消す
                    for j in rect.c1..rect.c2 {
                        self.update_hcnt(d, rect.r1, j, -1);
                    }
                    rect.r1 -= 1;
                    // 上足す
                    for j in rect.c1..rect.c2 {
                        self.update_hcnt(d, rect.r1, j, 1);
                    }
                    self.rect[d][n] = rect;
                }
                ok
            }
            3 => {
                // R
                let mut ok = true;
                if rect.c2 == w {
                    ok = false;
                }
                if ok {
                    for i in rect.r1..rect.r2 {
                        if self.sel[d][i][rect.c2] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    for i in rect.r1..rect.r2 {
                        self.sel[d][i][rect.c2] = true;
                    }
                    // 上下
                    self.update_hcnt(d, rect.r1, rect.c2, 1);
                    self.update_hcnt(d, rect.r2, rect.c2, 1);
                    // 右消す
                    for i in rect.r1..rect.r2 {
                        self.update_vcnt(d, i, rect.c2, -1);
                    }
                    rect.c2 += 1;
                    // 右足す
                    for i in rect.r1..rect.r2 {
                        self.update_vcnt(d, i, rect.c2, 1);
                    }
                    self.rect[d][n] = rect;
                }
                ok
            }
            _ => panic!("dir must be from 0 to 3"),
        };

        if ok {
            self.sort_rect(d);
            self.cost += self.calc_area_cost(d, self.rect[0].len(), a) - prev_area_cost;
        }
    }

    /// 長方形の追加に応じて vcnt, hcnt を更新する
    fn add_rect(&mut self, d: usize, rect: &Rectangle) {
        // ここでスコアの差分更新もしたい...

        for i in rect.r1..rect.r2 {
            self.vcnt[d][i][rect.c1] += 1;
            self.vcnt[d][i][rect.c2] += 1;
        }
        for j in rect.c1..rect.c2 {
            self.hcnt[d][rect.r1][j] += 1;
            self.hcnt[d][rect.r2][j] += 1;
        }
    }

    /// 長方形の削除に応じて vcnt, hcnt を更新する
    fn delete_rect(&mut self, d: usize, rect: &Rectangle) {
        // ここでスコアの差分更新もしたい...

        for i in rect.r1..rect.r2 {
            self.vcnt[d][i][rect.c1] -= 1;
            self.vcnt[d][i][rect.c2] -= 1;
        }
        for j in rect.c1..rect.c2 {
            self.hcnt[d][rect.r1][j] -= 1;
            self.hcnt[d][rect.r2][j] -= 1;
        }
    }

    fn calc_area_cost(&mut self, d: usize, n: usize, a: &Vec<Vec<i64>>) -> i64 {
        let mut cost = 0;
        // area
        for j in 0..n {
            let area = self.rect[d][j].area();
            if a[d][j] > area {
                cost += (a[d][j] - area) * 100;
            }
        }
        cost
    }

    fn calc_cost(&mut self, w: usize, d: usize, n: usize, a: &Vec<Vec<i64>>) -> i64 {
        let mut cost = 0;
        // vcnt
        for i in 0..d - 1 {
            for j in 0..w {
                for k in 1..w - 1 {
                    if self.vcnt[i][j][k] == 0 && self.vcnt[i + 1][j][k] != 0
                        || self.vcnt[i][j][k] != 0 && self.vcnt[i + 1][j][k] == 0
                    {
                        cost += 1;
                    }
                }
            }
        }

        // hcnt
        for i in 0..d - 1 {
            for j in 1..w - 1 {
                for k in 0..w {
                    if self.hcnt[i][j][k] == 0 && self.hcnt[i + 1][j][k] != 0
                        || self.hcnt[i][j][k] != 0 && self.hcnt[i + 1][j][k] == 0
                    {
                        cost += 1;
                    }
                }
            }
        }

        // area
        for i in 0..d {
            //cost += self.calc_area_cost(i, n, a);
            for j in 0..n {
                let area = self.rect[i][j].area();
                if a[i][j] > area {
                    cost += (a[i][j] - area) * 100;
                }
            }
        }

        cost
    }
}

struct Solver {
    w: usize,
    d: usize,
    n: usize,
    a: Vec<Vec<i64>>,
    rng: ChaCha20Rng,
    state: State,
}

impl Solver {
    fn new(w: usize, d: usize, n: usize, a: Vec<Vec<i64>>) -> Self {
        let mut rng = ChaCha20Rng::seed_from_u64(398578937);
        let state = State::new(w, d, n, &mut rng);
        Solver {
            w,
            d,
            n,
            a,
            rng,
            state,
        }
    }

    fn init(&mut self) {
        for i in 0..10000 {
            let d = self.rng.gen_range(0..self.d);
            let n = self.rng.gen_range(0..self.n);
            let dir = self.rng.gen_range(0..4);
            self.state.expand(self.w, d, n, dir, &self.a);
        }
    }

    fn update(&mut self) {
    }

    fn climb(&mut self, start: time::Instant) {
        // 山登り
        loop {
            if time::Instant::now() - start > time::Duration::from_millis(2950) {
                break;
            }
            self.update();
        }
    }
}

fn main() {
    // // AOJ
    // let mut s = String::new();
    // let stdin = stdin();
    // let mut reader = Reader::new(&mut s, stdin);

    // // interactive
    // let stdin = stdin();
    // let mut source = LineSource::new(BufReader::new(stdin.lock()));

    // 初期化
    // 開始点はランダム
    // ランダムに拡大
    // 小さい順に割当て

    // 更新
    // 日をランダムに選択
    // 長方形をランダムに選択
    // 拡大、縮小、縦横変更

    // 一回の更新で触ったところを保持→Lの計算をするときにそこだけチェック
    // 拡大するときは拡大するところだけ見る。縮小も同様。

    let start = time::Instant::now();

    input! {
        // from &mut source,
        w: usize,
        d: usize,
        n: usize,
        a: [[i64; n]; d],
    }

    let mut solver = Solver::new(w, d, n, a.clone());

    solver.state.cost = solver.state.calc_cost(w, d, n, &a);

    solver.init();

    //solver.climb(start);

    for i in 0..d {
        for j in 0..n {
            let rect = solver.state.rect[i][j].clone();
            println!("{} {} {} {}", rect.r1, rect.c1, rect.r2, rect.c2);
        }
    }
    eprintln!("{}", solver.state.cost + 1);
    // eprintln!("{}", solver.state.calc_cost(w, d, n, &a) + 1);
}

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cmp::{max, min};
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};
use std::str::FromStr;
use std::time::Instant;
use std::{collections::*, time};

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

/// 単一の値をデバッグプリントするための関数
fn pd<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}

/// 単一の値を入力する
fn input<T: FromStr>() -> T {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().ok().unwrap()
}

/// 一行の複数の値を入力する
fn input_vec<T: FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let v = buffer
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    v
}

// TODO: 複数の型が入り得る場合を処理したい（どうやって？）
/// 複数行を入力する
fn input_lines<T: FromStr>(n: usize) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    for i in 0..n {
        v.push(input());
    }
    v
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
}

// dir の方向にすすむ
fn next_pos(w: usize, h: usize, now: (usize, usize), dir: (i64, i64)) -> Option<(usize, usize)> {
    let nr = now.0 as i64 + dir.0;
    let nc = now.1 as i64 + dir.1;
    if !(0 <= nr && nr < h as i64 && 0 <= nc && nc < w as i64) {
        return None;
    }
    let nr = nr as usize;
    let nc = nc as usize;
    Some((nr, nc))
}

type Mint = Modint<MOD998>;

use std::{fmt, ops};
type ModintMod = i64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modint<const MOD: ModintMod> {
    x: ModintMod,
}

impl<const MOD: ModintMod> std::fmt::Display for Modint<MOD> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl<const MOD: ModintMod> Modint<MOD> {
    pub fn zero() -> Self {
        Modint { x: 0 }
    }

    pub fn new(x: ModintMod) -> Self {
        Modint {
            x: (x % MOD) as ModintMod,
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
        self.pow((MOD - 2) as i64)
    }
}

impl<const MOD: ModintMod> ops::Neg for Modint<MOD> {
    type Output = Modint<MOD>;

    fn neg(mut self) -> Modint<MOD> {
        self.x = (MOD - self.x);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Add<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: Self) -> Modint<MOD> {
        self + rhs.x
    }
}

impl<const MOD: ModintMod> ops::Add<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn add(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = (self.x + rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Sub<Self> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: Self) -> Modint<MOD> {
        self - rhs.x
    }
}

impl<const MOD: ModintMod> ops::Sub<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;

    fn sub(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = (self.x + MOD - rhs);
        if self.x >= MOD {
            self.x -= MOD;
        }
        self
    }
}

impl<const MOD: ModintMod> ops::Mul<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: Self) -> Modint<MOD> {
        self * rhs.x
    }
}

impl<const MOD: ModintMod> ops::Mul<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn mul(mut self, rhs: ModintMod) -> Modint<MOD> {
        self.x = self.x * rhs % MOD;
        self
    }
}

impl<const MOD: ModintMod> ops::Div<Self> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: Self) -> Modint<MOD> {
        self / rhs.x
    }
}

impl<const MOD: ModintMod> ops::Div<ModintMod> for Modint<MOD> {
    type Output = Modint<MOD>;
    fn div(mut self, rhs: ModintMod) -> Modint<MOD> {
        if rhs == 0 {
            panic!("0 division is occured");
        }
        self * Modint::<MOD>::new(rhs).inv()
    }
}

impl<const MOD: ModintMod> ops::AddAssign<Self> for Modint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const MOD: ModintMod> ops::AddAssign<ModintMod> for Modint<MOD> {
    fn add_assign(&mut self, rhs: ModintMod) {
        *self = *self + rhs;
    }
}

impl<const MOD: ModintMod> ops::SubAssign<Self> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const MOD: ModintMod> ops::SubAssign<ModintMod> for Modint<MOD> {
    fn sub_assign(&mut self, rhs: ModintMod) {
        *self = *self - rhs;
    }
}

impl<const MOD: ModintMod> ops::MulAssign<Self> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const MOD: ModintMod> ops::MulAssign<ModintMod> for Modint<MOD> {
    fn mul_assign(&mut self, rhs: ModintMod) {
        *self = *self * rhs;
    }
}

impl<const MOD: ModintMod> ops::DivAssign<Self> for Modint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const MOD: ModintMod> ops::DivAssign<ModintMod> for Modint<MOD> {
    fn div_assign(&mut self, rhs: ModintMod) {
        *self = *self / rhs;
    }
}
