pub struct LazySegmentTree<T, OT, F, G, H> {
    n: usize,
    sz: usize,
    data: Vec<T>,
    lazy: Vec<OT>,
    f: F,
    g: G,
    h: H,
    e: T,
    oe: OT,
}

impl<T, OT, F, G, H> LazySegmentTree<T, OT, F, G, H>
where
    T: Clone + Copy,
    OT: Clone + Copy + PartialEq + Eq,
    F: Fn(T, T) -> T,
    G: Fn(T, OT, usize) -> T,
    H: Fn(OT, OT) -> OT,
{
    pub fn new(n: usize, f: F, g: G, h: H, e: T, oe: OT) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }

        LazySegmentTree {
            n,
            sz,
            data: vec![e; 2 * sz],
            lazy: vec![oe; 2 * sz],
            f,
            g,
            h,
            e,
            oe,
        }
    }

    // 初期化する. O(n)
    // v: 初期値の配列
    // 使用例
    // let v = vec![1, 2, 3, 4, 5];
    // st.build(&v)
    pub fn build(&mut self, v: &Vec<T>) {
        assert!(v.len() == self.n);
        for i in 0..self.n {
            self.data[self.sz + i] = v[i];
        }
        for i in (1..self.sz).rev() {
            self.data[i] = (self.f)(self.data[i * 2], self.data[i * 2 + 1]);
        }
    }

    // 指定した区間に作用素 x を作用させる O(log n)
    // a, b: [a, b) に x を作用させる
    // x: 作用素モノイドの元
    // 使い方 (区間[a, b) を x に変更)
    // st.update(a, b, x);
    pub fn update(&mut self, a: usize, b: usize, x: OT) {
        self.update_range(a, b, x, 1, 0, self.sz);
    }

    // 指定した区間に取得クエリを実行する O(log n)
    // a, b: [a, b) のクエリを実行する
    // st.query(a, b)
    pub fn query(&mut self, a: usize, b: usize) -> T {
        self.query_range(a, b, 1, 0, self.sz)
    }

    fn propagate(&mut self, k: usize, len: usize) {
        if self.lazy[k] == self.oe {
            return;
        }
        if k < self.sz {
            self.lazy[k * 2] = (self.h)(self.lazy[k * 2], self.lazy[k]);
            self.lazy[k * 2 + 1] = (self.h)(self.lazy[k * 2 + 1], self.lazy[k]);
        }
        self.data[k] = (self.g)(self.data[k], self.lazy[k], len);
        self.lazy[k] = self.oe;
    }

    fn update_range(&mut self, a: usize, b: usize, x: OT, k: usize, l: usize, r: usize) {
        self.propagate(k, r - l);
        if r <= a || b <= l {
            return;
        } else if a <= l && r <= b {
            self.lazy[k] = (self.h)(self.lazy[k], x);
            self.propagate(k, r - l);
        } else {
            self.update_range(a, b, x, k * 2, l, (l + r) / 2);
            self.update_range(a, b, x, k * 2 + 1, (l + r) / 2, r);
            self.data[k] = (self.f)(self.data[k * 2], self.data[k * 2 + 1]);
        }
    }

    fn query_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        self.propagate(k, r - l);
        if r <= a || b <= l {
            return self.e;
        } else if a <= l && r <= b {
            return self.data[k];
        } else {
            let vl = self.query_range(a, b, k * 2, l, (l + r) / 2);
            let vr = self.query_range(a, b, k * 2 + 1, (l + r) / 2, r);
            return (self.f)(vl, vr);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_add_sum() {
        let n = 100;
        let mut st = LazySegmentTree::new(
            n,
            |a, b| a + b,
            |a, b, len| a + b * len as i64,
            |a, b| a + b,
            0,
            0,
        );
        let mut v: Vec<i64> = (0..n as i64).collect();
        st.build(&v);

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let t = rng.gen_bool(1.0 / 3.0);
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l + 1..=n);
            if t {
                let x = rng.gen_range(-100000..100000);
                for i in l..r {
                    v[i] += x;
                }
                st.update(l, r, x);
            } else {
                let mut su = 0;
                for i in l..r {
                    su += v[i];
                }
                assert_eq!(su, st.query(l, r));
            }
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_update_sum() {
        let n = 100;
        let mut st = LazySegmentTree::new(
            n,
            |a, b| a + b,
            |a, b, len| b * len as i64,
            |a, b| b,
            0,
            i64::MAX,
        );
        let mut v: Vec<i64> = (0..n as i64).collect();
        st.build(&v);

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let t = rng.gen_bool(1.0 / 3.0);
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l + 1..=n);
            if t {
                let x = rng.gen_range(-100000..100000);
                for i in l..r {
                    v[i] = x;
                }
                st.update(l, r, x);
                for i in l..r {
                    println!("{} {} {}", l, r, i);
                    assert_eq!(x, st.query(i, i + 1));
                }
            } else {
                let mut su = 0;
                for i in l..r {
                    su += v[i];
                }
                println!("{} {}", l, r);
                //assert_eq!(su, st.query(l, r));
            }
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_add_min() {
        let n = 100;
        let mut st = LazySegmentTree::new(
            n,
            |a, b| a.min(b),
            |a, b, len| a + b,
            |a, b| a + b,
            i64::MAX,
            0,
        );
        let mut v: Vec<i64> = (0..n as i64).collect();
        st.build(&v);

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let t = rng.gen_bool(1.0 / 3.0);
            let l = rng.gen_range(0..n);
            let r = rng.gen_range(l + 1..=n);
            if t {
                let x = rng.gen_range(-100000..100000);
                for i in l..r {
                    v[i] = x;
                }
                st.update(l, r, x);
                for i in l..r {
                    println!("{} {} {}", l, r, i);
                    assert_eq!(x, st.query(i, i + 1));
                }
            } else {
                let mut su = 0;
                for i in l..r {
                    su += v[i];
                }
                println!("{} {}", l, r);
                //assert_eq!(su, st.query(l, r));
            }
            println!("{:?}", v);
        }
    }

    #[test]
    fn test_matrix() {
        // a1, a2
        // 0, 1
        // の形をした行列をのせる
        // f は和

        // もっと大きくしたければ、がんばる...

        let n = 100;
        let mut st = LazySegmentTree::new(
            n,
            |a, b| a + b,
            |a, b, len| b.0 * a + b.1,
            |a, b| (a.0 * b.0, b.0 * a.1 + b.1),
            0,
            (1, 0),
        );
    }
}
