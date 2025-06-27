use std::cmp::max;
use std::collections::BinaryHeap;
use std::io::{self, Read};

/// ------------------------- Segment tree (range-max, point-set) -------------------------
struct SegTree {
    size: usize,
    dat: Vec<i32>, // 1-indexed完全二分木
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n + 2 {
            sz <<= 1;
        }
        SegTree {
            size: sz,
            dat: vec![0; sz << 1],
        }
    }
    /// 0-indexedの pos に val を代入して親を更新
    fn point_set(&mut self, mut pos: usize, val: i32) {
        pos += self.size;
        self.dat[pos] = val;
        pos >>= 1;
        while pos > 0 {
            self.dat[pos] = max(self.dat[pos << 1], self.dat[pos << 1 | 1]);
            pos >>= 1;
        }
    }
    /// [l, r] での最大値 (0-indexed, inclusive)
    fn range_max(&self, mut l: usize, mut r: usize) -> i32 {
        if l > r {
            return 0;
        }
        l += self.size;
        r += self.size;
        let mut res = 0;
        while l <= r {
            if l & 1 == 1 {
                res = max(res, self.dat[l]);
                l += 1;
            }
            if r & 1 == 0 {
                res = max(res, self.dat[r]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
    #[inline]
    fn all_max(&self) -> i32 {
        self.dat[1]
    }
}

/// ------------------------- “可削除” 最大ヒープ -------------------------
/// BinaryHeap だけだと任意要素の削除が O(N) になるので、
/// 「削除予定ヒープ」と二段重ねにして lazy-delete で amortized O(log N)
struct LazyMaxHeap {
    main: BinaryHeap<i32>,
    rm: BinaryHeap<i32>,
}
impl LazyMaxHeap {
    fn new() -> Self {
        LazyMaxHeap {
            main: BinaryHeap::new(),
            rm: BinaryHeap::new(),
        }
    }
    #[inline]
    fn push(&mut self, x: i32) {
        self.main.push(x);
    }
    #[inline]
    fn erase(&mut self, x: i32) {
        self.rm.push(x);
    }
    /// heap top（0 = 空）
    fn top(&mut self) -> i32 {
        while let (Some(&a), Some(&b)) = (self.main.peek(), self.rm.peek()) {
            if a == b {
                self.main.pop();
                self.rm.pop();
            } else {
                break;
            }
        }
        *self.main.peek().unwrap_or(&0)
    }
}

/// ------------------------- core: 右側 LDS を全 cut 同時計算 -------------------------
fn sweep_lis(n: usize, chords: &[(usize, usize)]) -> Vec<i32> {
    let m = 2 * n;
    // 左端で加算，右端で除去
    let mut add = vec![Vec::<usize>::new(); m + 2];
    let mut del = vec![Vec::<usize>::new(); m + 2];
    for (idx, &(l, r)) in chords.iter().enumerate() {
        add[l].push(idx);
        del[r].push(idx);
    }

    let mut seg = SegTree::new(m + 2);
    let mut heaps: Vec<LazyMaxHeap> = (0..=m).map(|_| LazyMaxHeap::new()).collect();
    let mut len_of = vec![0i32; n]; // chord idx に対する “この sweep での長さ”
    let mut best = vec![0i32; m + 2];

    for pos in 1..=m {
        // 左端が pos　　 →　追加
        for &idx in &add[pos] {
            let r = chords[idx].1;
            // LDS: r より大きい右端での最大長 + 1
            let val = 1 + if r < m { seg.range_max(r + 1, m) } else { 0 };
            len_of[idx] = val;
            heaps[r].push(val);
            seg.point_set(r, heaps[r].top());
        }
        // 現状態の全体 max
        best[pos] = seg.all_max();

        // 右端が pos　　 →　削除
        for &idx in &del[pos] {
            let r = chords[idx].1;
            heaps[r].erase(len_of[idx]);
            seg.point_set(r, heaps[r].top());
        }
    }
    best
}

/// ------------------------- main -------------------------
fn main() {
    // --- input ---
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    let m = 2 * n;
    let mut chords = Vec::<(usize, usize)>::with_capacity(n);
    let mut partner = vec![0usize; m + 1];
    for _ in 0..n {
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        let (l, r) = if a < b { (a, b) } else { (b, a) };
        chords.push((l, r));
        partner[l] = r;
        partner[r] = l;
    }

    // sweep 1: 右区間 (u,v) 型 → longest *decreasing* 右端
    let right_best = sweep_lis(n, &chords); // index: v

    // sweep 2: 左区間 (L→R 型)　— 座標を左右反転して同じ関数を再利用
    let mut rev_chords = Vec::<(usize, usize)>::with_capacity(n);
    for &(l, r) in &chords {
        // 反転座標
        let nl = m + 1 - r;
        let nr = m + 1 - l;
        rev_chords.push((nl, nr));
    }
    let mut left_best_rev = sweep_lis(n, &rev_chords); // index: “反転後” の v
    let mut left_best = vec![0i32; m + 2]; // index: u
    for u in 1..=m {
        left_best[u] = left_best_rev[m + 1 - u];
    }

    // prefix 最大を取りながら合成
    let mut ans = 0i32;
    let mut pref_max = 0i32;
    for v in 1..=m {
        pref_max = max(pref_max, left_best[v]);
        // ─2 は「新弦の両端と既存弦が共有するとき」の最大減衰量
        ans = max(ans, pref_max + right_best[v] - 2);
    }
    if ans < 0 {
        ans = 0;
    }
    println!("{}", ans);
}
