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
use std::sync::OnceLock;

pub const MOD998244353: i64 = 998244353;

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
    pub fn new(x: i64) -> Self {
        let p = MOD998244353;
        if x >= 0 {
            Modint { x: x % p, p }
        } else {
            let tmp = x.abs() % p;
            let val = x + tmp * p;
            Modint { x: (val + p) % p, p }
        }
    }

    pub fn new_p(x: i64, p: i64) -> Self {
        if x >= 0 {
            Modint { x: x % p, p }
        } else {
            let tmp = x.abs() % p;
            let val = x + tmp * p;
            Modint { x: (val + p) % p, p }
        }
    }

    pub fn pow(&self, mut k: i64) -> Self {
        let mut mul = Modint::new_p(self.x, self.p);
        let mut res = Modint::new_p(1, self.p);
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
        self * Modint::new_p(rhs, self.p).inv()
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

    let mut inva = Modint::new_p(a, p).inv();
    let mut invb = inva * Modint::new_p(-b, p);

    let mut biga = Modint::new_p(1, p);
    let mut bigb = Modint::new_p(0, p);
    let mut m = 1;
    while m * m < p {
        m += 1;
    }

    let mut small = BTreeMap::new();
    let mut crr = Modint::new_p(g, p);
    for i in 0..m {
        if !small.contains_key(&crr.x) {
            small.insert(crr.x, i);
        }
        crr = crr * inva + invb;
        biga *= a;
        bigb *= a;
        bigb += b;
    }

    let mut crr = Modint::new_p(s, p);
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


pub struct Combination {
    n: usize,
    fact: Vec<Modint>,
    rfact: Vec<Modint>,
}

impl Combination {
    pub fn new() -> Self {
        Combination {
            n: 1,
            fact: vec![Modint::new(1)],
            rfact: vec![Modint::new(1)],
        }
    }

    pub fn extend(&mut self, n: usize) {
        if self.n >= n {
            return
        }
        for i in self.n..n {
            self.fact.push(self.fact[i - 1] * Modint::new(i as i64));
        }
        for i in self.n..n {
            self.rfact.push(self.fact[i].inv());
        }
        self.n = n;
    }

    pub fn fact(&mut self, k: usize) -> Modint {
        self.extend(k + 1);
        self.fact[k]
    }

    pub fn rfact(&mut self, k: usize) -> Modint {
        self.extend(k + 1);
        self.rfact[k]
    }

    #[allow(non_snake_case)]
    pub fn P(&mut self, n: usize, k: usize) -> Modint {
        if n < k {
            Modint::new(0)
        } else {
            self.fact(n) * self.rfact(n - k)
        }
    }

    #[allow(non_snake_case)]
    pub fn C(&mut self, n: usize, k: usize) -> Modint {
        if n < k {
            Modint::new(0)
        } else {
            self.fact(n) * self.rfact(k) * self.rfact(n - k)
        }
    }

    #[allow(non_snake_case)]
    pub fn H(&mut self, n: usize, k: usize) -> Modint {
        if n == 0 && k == 0 {
            Modint::new(1)
        } else {
            self.C(n + k - 1, k)
        }
    }
}



static POWV: OnceLock<Vec<Modint>> = OnceLock::new();
static INVPOWV: OnceLock<Vec<Modint>> = OnceLock::new();

fn calc_powv() -> Vec<Modint> {
    let mut res = vec![];
    let mut r = Modint::new(3).pow(119);
    for i in 0..23 {
        res.push(r);
        r = r * r;
    }
    res.reverse();
    res
}

fn calc_invpowv() -> Vec<Modint> {
    let mut res = vec![];
    let mut r = Modint::new(3).pow(119).inv();
    for i in 0..23 {
        res.push(r);
        r = r * r;
    }
    res.reverse();
    res
}

fn get_powv() -> &'static Vec<Modint> {
    POWV.get_or_init(calc_powv)
}

fn get_invpowv() -> &'static Vec<Modint> {
    INVPOWV.get_or_init(calc_invpowv)
}

fn ntt(a: &Vec<Modint>, depth: i64, root: &Vec<Modint>) -> Vec<Modint> {
    let n = a.len();
    if n == 1 {
        return a.clone();
    }

    let mut even = vec![];
    let mut odd = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            even.push(a[i]);
        } else {
            odd.push(a[i]);
        }
    }

    let d_even = ntt(&even, depth - 1, root);
    let d_odd = ntt(&odd, depth - 1, root);

    let r = root[depth as usize];

    let mut now = Modint::new(1);
    let mut res = vec![];
    for i in 0..n {
        res.push(d_even[i % (n / 2)] + now * d_odd[i % (n / 2)]);
        now = now * r;
    }

    res
}

// Butterfly NTT（反復的な実装、最適化版）
fn butterfly_ntt(a: &mut Vec<Modint>, root: &Vec<Modint>) {
    let n = a.len();
    if n == 1 {
        return;
    }

    let log2n = n.trailing_zeros() as usize;

    // ビット反転によるデータの並び替え（最適化版）
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    // 回転因子を事前計算
    let mut twiddles = vec![vec![Modint::new(1); 1]; log2n];
    for h in 0..log2n {
        let len = 1 << (h + 1);
        let r = root[h];
        twiddles[h].resize(len / 2, Modint::new(1));
        for k in 1..(len / 2) {
            twiddles[h][k] = twiddles[h][k - 1] * r;
        }
    }

    // Butterfly演算
    for h in 0..log2n {
        let len = 1 << (h + 1);
        let half = len / 2;
        let tw = &twiddles[h];

        for i in (0..n).step_by(len) {
            for j in 0..half {
                let u = a[i + j];
                let v = a[i + j + half] * tw[j];
                a[i + j] = u + v;
                a[i + j + half] = u - v;
            }
        }
    }
}

pub fn convolution(mut a: Vec<Modint>, mut b: Vec<Modint>) -> Vec<Modint> {
    let sza = a.len();
    let szb = b.len();
    let mut n = 1;
    while n <= sza + szb - 1 {
        n *= 2;
    }

    a.resize(n, Modint::new(0));
    b.resize(n, Modint::new(0));

    let powv = get_powv();
    let invpowv = get_invpowv();

    butterfly_ntt(&mut a, powv);
    butterfly_ntt(&mut b, powv);

    // 要素ごとの乗算を最適化
    for i in 0..n {
        a[i] = a[i] * b[i];
    }

    butterfly_ntt(&mut a, invpowv);

    let ninv = Modint::new(n as i64).inv();
    a.truncate(sza + szb - 1);
    for i in 0..(sza + szb - 1) {
        a[i] = a[i] * ninv;
    }
    a
}


#[derive(Clone, Debug)]
pub struct Fps {
    pub n: usize,
    pub a: Vec<Modint>,
}

// \Sum[i=0 to inf] (x^k)^i = 1 / (1 - x^k)
// [x^n] 1 / (1 - x^k) = (n + r - 1) C (r - 1)

impl Fps {
    pub fn new() -> Self {
        Fps {
            n: 1,
            a: vec![Modint::new(0)],
        }
    }

    pub fn from_mint_vec(a: Vec<Modint>) -> Self {
        Fps { n: a.len(), a }
    }

    pub fn from_i64_vec(a_in: Vec<i64>) -> Self {
        let mut a = vec![];
        for i in 0..a_in.len() {
            a.push(Modint::new(a_in[i]));
        }
        Fps { n: a.len(), a }
    }

    pub fn from_const(val: i64) -> Self {
        Fps {
            n: 1,
            a: vec![Modint::new(val)],
        }
    }

    pub fn get_n(&self, n: usize) -> Self {
        let mut a = vec![Modint::new(0); n + 1];
        for i in 0..self.n {
            if i > n {
                break;
            }
            a[i] = self.a[i];
        }
        Fps {
            n: a.len(),
            a: a.to_vec(),
        }
    }

    /// 1 / (1 - x) を x^nまで計算する。
    /// 
    /// O(N logN)
    /// 
    /// f_0 = 0 のときは存在しない
    pub fn inv(&self, n: usize) -> Self {
        assert!(self.a[0] != Modint::new(0));
        let mut g = Fps::from_mint_vec(vec![self.a[0].inv()]);
        let mut sz = 1;
        while sz < n {
            sz *= 2;
            let mut ng = &g * &self.get_n(sz);
            ng = Fps::from_const(2) - ng;
            ng = &g * &ng;
            g = ng.get_n(sz);
        }

        g
    }

    /// 微分
    /// 
    /// O(N)
    pub fn differential(&self, n: usize) -> Self {
        let mut a = vec![];
        for i in 1..n {
            if i < self.n {
                a.push(self.a[i] * Modint::new(i as i64));
            } else {
                a.push(Modint::new(0));
            }
        }
        Fps::from_mint_vec(a)
    }

    /// 積分
    /// 
    /// O(N)
    pub fn integral(&self, n: usize) -> Self {
        let mut a = vec![Modint::new(0)];
        for i in 0..n - 1 {
            if i < self.n {
                a.push(self.a[i] / Modint::new((i as i64) + 1));
            } else {
                a.push(Modint::new(0));
            }
        }
        Fps::from_mint_vec(a)
    }

    /// log f
    /// 
    /// O(N logN)
    /// 
    /// log f = integral ((differential f) / f)
    /// 
    /// f_0 == 1 でないといけない
    pub fn log(&self, n: usize) -> Self {
        assert_eq!(self.a[0].x, 1);
        let df = self.differential(n);
        (&df / self).integral(n).get_n(n)
    }

    /// exp f
    /// 
    /// O(N logN)
    /// 
    /// f_0 == 0 でないといけない
    pub fn exp(&self, n: usize) -> Self {
        assert_eq!(self.a[0].x, 0);
        let mut g = Fps::from_const(1);
        let mut sz = 1;
        while sz < n {
            sz *= 2;
            let mut ng = &g * &(self.get_n(sz) + Fps::from_const(1) - g.log(sz));
            g = ng.get_n(sz);
        }

        g
    }

    pub fn inv_one_minus_xk(k: usize, n: usize, comb: &mut Combination) -> Modint {
        /*
        (1 + x^k + x^{2k} + x^{3k} + ... ) = 1 / (1 - x^k)

        [x^n] 1 / (1 - x^k) = (n + r - 1) C (r - 1)
        */
        comb.C(n + k - 1, k - 1)
    }
}

impl std::ops::Add<&Fps> for &Fps {
    type Output = Fps;

    fn add(self, other: &Fps) -> Fps {
        let mut c = vec![];
        let n = self.a.len();
        let m = other.a.len();
        for i in 0..n.min(m) {
            c.push(self.a[i] + other.a[i]);
        }

        if n > m {
            for i in m..n {
                c.push(self.a[i]);
            }
        } else {
            for i in n..m {
                c.push(other.a[i]);
            }
        }
        Fps::from_mint_vec(c)
    }
}

impl std::ops::Add<Fps> for Fps {
    type Output = Fps;
    fn add(self, other: Fps) -> Fps {
        &self + &other
    }
}

impl std::ops::Sub<&Fps> for &Fps {
    type Output = Fps;

    fn sub(self, other: &Fps) -> Fps {
        let mut c = vec![];
        let n = self.a.len();
        let m = other.a.len();
        for i in 0..n.max(m) {
            let a_val = if i < n { self.a[i] } else { Modint::new(0) };
            let b_val = if i < m { other.a[i] } else { Modint::new(0) };
            c.push(a_val - b_val);
        }
        Fps::from_mint_vec(c)
    }
}

impl std::ops::Sub<Fps> for Fps {
    type Output = Fps;
    fn sub(self, other: Fps) -> Fps {
        &self - &other
    }
}

impl std::ops::Mul<&Fps> for &Fps {
    type Output = Fps;

    fn mul(self, other: &Fps) -> Fps {
        let n = self.a.len();
        let m = other.a.len();
        let res = convolution(self.a.clone(), other.a.clone());
        Fps::from_mint_vec(res)
    }
}

impl std::ops::Mul<Fps> for Fps {
    type Output = Fps;

    fn mul(self, other: Fps) -> Fps {
        &self * &other
    }
}

impl std::ops::Div<&Fps> for &Fps {
    type Output = Fps;

    fn div(self, other: &Fps) -> Fps {
        let n = self.n;
        let m = other.n;
        let inv = other.inv(n.max(m) + 10);
        self * &inv
    }
}

impl std::ops::Div<Fps> for Fps {
    type Output = Fps;

    fn div(self, other: Fps) -> Fps {
        &self / &other
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


