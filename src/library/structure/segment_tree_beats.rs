pub struct SegmentTreeBeats {
    pub n: usize,

    pub ma: Vec<i64>,
    pub sma: Vec<i64>,
    pub macnt: Vec<i64>,

    pub mi: Vec<i64>,
    pub smi: Vec<i64>,
    pub micnt: Vec<i64>,

    pub su: Vec<i64>,

    pub len: Vec<i64>,
    pub ladd: Vec<i64>,
    pub lval: Vec<i64>,
}

impl SegmentTreeBeats {
    // -------------- public --------------
    pub fn new(n0: usize, init: Vec<i64>) -> Self {
        let mut n = 1;
        while n < n0 {
            n *= 2;
        }

        let mut ma = vec![i64::MIN; 2*n];
        let mut sma = vec![i64::MIN; 2*n];
        let mut macnt = vec![0; 2*n];

        let mut mi = vec![i64::MAX; 2*n];
        let mut smi = vec![i64::MAX; 2*n];
        let mut micnt = vec![0; 2*n];

        let mut su = vec![0; 2*n];

        for i in 0..n0 {
            ma[n+i] = init[i];
            macnt[n+i] = 1;

            mi[n+i] = init[i];
            micnt[n+i] = 1;

            su[n+i] = init[i];
        }

        let mut len = vec![0; 2*n];
        let mut ladd = vec![0; 2*n];
        let mut lval = vec![i64::MAX; 2*n];
        len[1] = n as i64;

        for i in 1..n {
            len[2 * i] = len[i] / 2;
            len[2 * i + 1] = len[i] / 2;
        }

        let mut res = SegmentTreeBeats {
            n,
            ma,
            sma,
            macnt,
            mi,
            smi,
            micnt,
            su,
            len,
            ladd,
            lval,
        };

        for i in (1..n).rev() {
            res.update(i);
        }

        res
    }

    // [a, b) を min(x, a[i]) に更新
    pub fn update_min(&mut self, a: usize, b: usize, x: i64) {
        self._update_min(0, self.n, 1, a, b, x);
    }

    // [a, b) を max(x, a[i]) に更新
    pub fn update_max(&mut self, a: usize, b: usize, x: i64) {
        self._update_max(0, self.n, 1, a, b, x);
    }

    // [a, b) に x を加算
    pub fn add_val(&mut self, a: usize, b: usize, x: i64) {
        self._add_val(0, self.n, 1, a, b, x);
    }

    // [a, b) を x に更新
    pub fn update_val(&mut self, a: usize, b: usize, x: i64) {
        self._update_val(0, self.n, 1, a, b, x);
    }

    // [a, b) の max を取得
    pub fn query_max(&mut self, a: usize, b: usize) -> i64 {
        self._query_max(0, self.n, 1, a, b)
    }

    // [a, b) の min を取得
    pub fn query_min(&mut self, a: usize, b: usize) -> i64 {
        self._query_min(0, self.n, 1, a, b)
    }

    // [a, b) の 和 を取得
    pub fn query_sum(&mut self, a: usize, b: usize) -> i64 {
        self._query_sum(0, self.n, 1, a, b)
    }

    // -------------- private --------------
    fn update_node_max(&mut self, k: usize, x: i64) {
        self.su[k] += (x - self.ma[k]) * self.macnt[k];

        if self.ma[k] == self.mi[k] {
            self.mi[k] = x;
        } else if self.ma[k] == self.smi[k] {
            self.smi[k] = x;
        }
        self.ma[k] = x;

        if self.lval[k] != i64::MAX && x < self.lval[k] {
            self.lval[k] = x;
        }
    }

    fn update_node_min(&mut self, k: usize, x: i64) {
        self.su[k] += (x - self.mi[k]) * self.micnt[k];

        if self.mi[k] == self.ma[k] {
            self.ma[k] = x;

        } else if self.mi[k] == self.sma[k] {
            self.sma[k] = x;
        }
        self.mi[k] = x;

        if self.lval[k] != i64::MAX && self.lval[k] < x {
            self.lval[k] = x;
        }
    }

    fn push(&mut self, k: usize) {
        if k >= self.n {
            return;
        }

        if self.lval[k] != i64::MAX {
            self.update_all(2 * k, self.lval[k]);
            self.update_all(2 * k + 1, self.lval[k]);
            self.lval[k] = i64::MAX;
            return;
        }

        if self.ladd[k] != 0 {
            self.add_all(2 * k, self.ladd[k]);
            self.add_all(2 * k + 1, self.ladd[k]);
            self.ladd[k] = 0;
        }

        if self.ma[k] < self.ma[2 * k] {
            self.update_node_max(2 * k, self.ma[k]);
        }
        if self.mi[k] > self.mi[2 * k] {
            self.update_node_min(2 * k, self.mi[k]);
        }

        if self.ma[k] < self.ma[2 * k + 1] {
            self.update_node_max(2 * k + 1, self.ma[k]);
        }
        if self.mi[k] > self.mi[2 * k + 1] {
            self.update_node_min(2 * k + 1, self.mi[k]);
        }
    }


    fn update(&mut self, k: usize) {
        // 2 * i と 2 * i + 1 から計算
        self.su[k] = self.su[2 * k] + self.su[2 * k + 1];

        if self.ma[2 * k] < self.ma[2 * k + 1] {
            self.ma[k] = self.ma[2 * k + 1];
            self.macnt[k] = self.macnt[2 * k + 1];
            self.sma[k] = self.ma[2 * k].max(self.sma[2 * k + 1]);
        } else if self.ma[2 * k] > self.ma[2 * k + 1] {
            self.ma[k] = self.ma[2 * k];
            self.macnt[k] = self.macnt[2 * k];
            self.sma[k] = self.sma[2 * k].max(self.ma[2 * k + 1]);
        } else {
            self.ma[k] = self.ma[2 * k];
            self.macnt[k] = self.macnt[2 * k] + self.macnt[2 * k + 1];
            self.sma[k] = self.sma[2 * k].max(self.sma[2 * k + 1]);
        }

        if self.mi[2 * k] > self.mi[2 * k + 1] {
            self.mi[k] = self.mi[2 * k + 1];
            self.micnt[k] = self.micnt[2 * k + 1];
            self.smi[k] = self.mi[2 * k].min(self.smi[2 * k + 1]);
        } else if self.mi[2 * k] < self.mi[2 * k + 1] {
            self.mi[k] = self.mi[2 * k];
            self.micnt[k] = self.micnt[2 * k];
            self.smi[k] = self.smi[2 * k].min(self.mi[2 * k + 1]);
        } else {
            self.mi[k] = self.mi[2 * k];
            self.micnt[k] = self.micnt[2 * k] + self.micnt[2 * k + 1];
            self.smi[k] = self.smi[2 * k].min(self.smi[2 * k + 1]);
        }
    }

    fn _update_min(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize, x: i64) {
        if r <= a || b <= l || self.ma[k] <= x {
            return;
        }
        if a <= l && r <= b && self.sma[k] < x {
            self.update_node_max(k, x);
            return;
        }

        self.push(k);
        self._update_min(l, (l + r) / 2, 2 * k, a, b, x);
        self._update_min((l + r) / 2, r, 2 * k + 1, a, b, x);
        self.update(k);
    }

    fn _update_max(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize, x: i64) {
        if r <= a || b <= l || self.mi[k] >= x {
            return;
        }
        if a <= l && r <= b && self.smi[k] > x {
            self.update_node_min(k, x);
            return;
        }

        self.push(k);
        self._update_max(l, (l + r) / 2, 2 * k, a, b, x);
        self._update_max((l + r) / 2, r, 2 * k + 1, a, b, x);
        self.update(k);
    }

    fn add_all(&mut self, k: usize, x: i64) {
        self.ma[k] += x;
        if self.sma[k] != i64::MIN {
            self.sma[k] += x;
        }

        self.mi[k] += x;
        if self.smi[k] != i64::MAX {
            self.smi[k] += x;
        }

        self.su[k] += self.len[k] * x;

        if self.lval[k] != i64::MAX {
            self.lval[k] += x;
        } else {
            self.ladd[k] += x;
        }
    }

    fn update_all(&mut self, k: usize, x: i64) {
        self.ma[k] = x;
        self.sma[k] = i64::MIN;
        self.mi[k] = x;
        self.smi[k] = i64::MAX;
        self.macnt[k] = self.len[k];
        self.micnt[k] = self.len[k];

        self.su[k] = x * self.len[k];
        self.lval[k] = x;
        self.ladd[k] = 0;
    }

    fn _add_val(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize, x: i64) {
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.add_all(k, x);
            return;
        }

        self.push(k);
        self._add_val(l, (l + r) / 2, 2 * k, a, b, x);
        self._add_val((l + r) / 2, r, 2 * k + 1, a, b, x);
        self.update(k);
    }

    fn _update_val(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize, x: i64) {
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.update_all(k, x);
            return;
        }

        self.push(k);
        self._update_val(l, (l + r) / 2, 2 * k, a, b, x);
        self._update_val((l + r) / 2, r, 2 * k + 1, a, b, x);
        self.update(k);
    }


    fn _query_max(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize) -> i64 {
        if r <= a || b <= l {
            return i64::MIN;
        }
        if a <= l && r <= b {
            return self.ma[k];
        }

        self.push(k);
        let lval = self._query_max(l, (l + r) / 2, 2 * k, a, b);
        let rval = self._query_max((l + r) / 2, r, 2 * k + 1, a, b);
        lval.max(rval)
    }

    fn _query_min(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize) -> i64 {
        if r <= a || b <= l {
            return i64::MAX;
        }
        if a <= l && r <= b {
            return self.mi[k];
        }

        self.push(k);
        let lval = self._query_min(l, (l + r) / 2, 2 * k, a, b);
        let rval = self._query_min((l + r) / 2, r, 2 * k + 1, a, b);
        lval.min(rval)
    }

    fn _query_sum(&mut self, l: usize, r: usize, k: usize, a: usize, b: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.su[k];
        }

        self.push(k);
        let lval = self._query_sum(l, (l + r) / 2, 2 * k, a, b);
        let rval = self._query_sum((l + r) / 2, r, 2 * k + 1, a, b);
        lval + rval
    }
}