pub struct SegmentTree<T, F, G>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
    G: Fn(T, T) -> T,
{
    n: usize,
    v: Vec<T>,
    f: F,
    g: G,
    zero: T,
}

impl<T, F, G> SegmentTree<T, F, G>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
    G: Fn(T, T) -> T,
{
    pub fn new(n: usize, v: Vec<T>, f: F, g: G, zero: T) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }

        let mut v_ = vec![zero; 2 * n_];
        for i in 0..n {
            v_[n_ + i] = v[i];
        }
        for i in (0..=n_ - 1).rev() {
            v_[i] = f(v_[i * 2], v_[i * 2 + 1]);
        }

        SegmentTree {
            n: n_,
            v: v_,
            f,
            g,
            zero,
        }
    }

    pub fn update(&mut self, i: usize, x: T) {
        self.v[self.n + i] = (self.g)(self.v[self.n + i], x);
        let mut now = (self.n + i) / 2;
        while now > 0 {
            self.v[now] = (self.f)(self.v[now * 2], self.v[now * 2 + 1]);
            now /= 2;
        }
    }

    fn query_(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
        if r <= a || b <= l {
            return self.zero;
        }
        if a <= l && r <= b {
            return self.v[k];
        }

        let val1 = self.query_(l, (l + r) / 2, 2 * k, a, b);
        let val2 = self.query_((l + r) / 2, r, 2 * k + 1, a, b);
        (self.f)(val1, val2)
    }

    pub fn query(&self, a: usize, b: usize) -> T {
        self.query_(0, self.n, 1, a, b)
    }
}

fn input<T: std::str::FromStr>() -> T {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().ok().unwrap()
}

fn input_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v = buffer
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    v
}

// #[test]
// fn main() {
//     let v_in = input_vec();
//     let n = v_in[0];
//     let q = v_in[1];
// 
//     let v = vec![0; n];
//     let mut st = SegmentTree::new(n, v, |a, b| a + b, |a, b| a + b, 0);
// 
//     for _ in 0..q {
//         let v_in: Vec<i64> = input_vec();
//         let com = v_in[0];
//         let x = v_in[1];
//         let y = v_in[2];
//         if com == 0 {
//             let x = (x - 1) as usize;
//             st.update(x, y);
//         } else {
//             let x = (x - 1) as usize;
//             let y = y as usize;
//             println!("{}", st.query(x, y));
//         }
//     }
// }
