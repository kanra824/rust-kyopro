
pub struct SegmentTree {
    n: usize,
    v: Vec<i64>,
}

impl SegmentTree {
    pub fn new(n: usize, v: Vec<i64>) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }

        let mut v_ = vec![0; 2 * n_];
        for i in 0..n {
            v_[i+1] = v[i];
        }
        for i in (0..=n_-1).rev() {
            v_[i] = v_[i*2] + v_[i*2+1];
        }

        SegmentTree {
            n: n_,
            v: v_,
        }
    }

    pub fn update(&mut self, i: usize, x: i64) {
        self.v[self.n+i] += x;
        let mut now = (self.n + i) / 2;
        while now > 0 {
            self.v[now] = self.v[now*2] + self.v[now*2+1];
            now /= 2;
        }
    }

    fn query_(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.v[k];
        }

        self.query_(l, (l+r)/2, 2*k, a, b) + self.query_((l+r)/2, r, 2*k+1, a, b)
    }

    pub fn query(&self, a: usize, b: usize) -> i64 {
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
    let v = buffer.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
    v
}

#[test]
fn main() {
    let v_in = input_vec();
    let n = v_in[0];
    let q = v_in[1];

    let v = vec![0; n];
    let mut st = SegmentTree::new(n, v);

    for _ in 0..q {
        let v_in: Vec<i64> = input_vec();
        let com = v_in[0];
        let x = v_in[1];
        let y = v_in[2];
        if com == 0 {
            let x = (x - 1) as usize;
            st.update(x, y);
        } else {
            let x = (x - 1) as usize;
            let y = y as usize;
            println!("{}", st.query(x, y));
        }
    }
}