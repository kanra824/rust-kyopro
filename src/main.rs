#![allow(unused)]

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rand_distr::{Dirichlet, Distribution};

#[derive(Clone, Debug)]

struct Input {
    n: usize,
    k: usize,
    h: usize,
    t: usize,
    d: i64,
    own: Vec<[f64; 3]>,
    target: Vec<[f64; 3]>,
}

fn input() -> Input {
    input! {
        n: usize,
        k: usize,
        h: usize,
        t: usize,
        d: i64,
        own: [[f64; 3]; k],
        target: [[f64; 3]; h],
    }

    Input {
        n,
        k,
        h,
        t,
        d,
        own: own.into_iter().map(|x| [x[0], x[1], x[2]]).collect(),
        target: target.into_iter().map(|x| [x[0], x[1], x[2]]).collect(),
    }
}

fn output(wall_v: &Vec<Vec<bool>>, wall_h: &Vec<Vec<bool>>, actions: &Vec<Action>) {
    let n = wall_v.len();
    for i in 0..n {
        for j in 0..n - 1 {
            print!("{} ", if wall_v[i][j] { '1' } else { '0' });
        }
        println!();
    }
    for i in 0..n - 1 {
        for j in 0..n {
            print!("{} ", if wall_h[i][j] { '1' } else { '0' });
        }
        println!();
    }
    for action in actions {
        match action {
            Action::Add { i, j, k } => println!("1 {} {} {}", i, j, k),
            Action::Deliver { i, j } => println!("2 {} {}", i, j),
            Action::Discard { i, j } => println!("3 {} {}", i, j),
            Action::Toggle { i1, j1, i2, j2 } => println!("4 {} {} {} {}", i1, j1, i2, j2),
        }
    }
}

#[derive(Clone, Debug)]
enum Action {
    Add {
        i: usize,
        j: usize,
        k: usize,
    },
    Deliver {
        i: usize,
        j: usize,
    },
    Discard {
        i: usize,
        j: usize,
    },
    Toggle {
        i1: usize,
        j1: usize,
        i2: usize,
        j2: usize,
    },
}

struct Pallet {
    sz_v: Vec<usize>,           // id に対するサイズの列
    pos_v: Vec<(usize, usize)>, // id に対する pos の列
}

impl Pallet {
    fn new(input: &Input) -> (Pallet, Vec<Vec<bool>>, Vec<Vec<bool>>) {
        // 適当なサイズで配置を作成する
        // ここでは lambda = 5 と固定して 80 個に分割する
        let n = input.n;
        let sz_v = vec![5; 80];

        // 0 1 2 3 4
        // 5 6 7 8 9
        // .....
        let mut pos_v = vec![];
        for i in 0..input.n {
            for j in 0..4 {
                pos_v.push((i, j * (n / 4)));
            }
        }
        let mut wall_v = vec![vec![false; n - 1]; n];
        let mut wall_h = vec![vec![false; n]; n - 1];
        for i in 0..n {
            for j in 1..(n / 5) {
                wall_v[i][j * 5 - 1] = true;
            }
        }

        for i in 0..n - 1 {
            for j in 0..n {
                wall_h[i][j] = true;
            }
        }



        (Pallet { sz_v, pos_v }, wall_v, wall_h)
    }
}

#[derive(Clone, Debug)]
struct SAState {
    // SA の中で変わりうる要素だけ持たせる
    targets: Vec<Vec<Vec<usize>>>,
    targets_rev: Vec<(usize, usize)>, // target_id -> (well_id, pos_id)
}

impl SAState {
    fn new(input: &Input, pallet: &Pallet, lambda: usize) -> SAState {
        let mut targets = vec![vec![]; pallet.sz_v.len()];
        let mut targets_rev = vec![(usize::MAX, usize::MAX); input.h];

        // targets 初期化
        let mut well_cnt = vec![0; pallet.sz_v.len()];
        for i in 0..input.h {
            if targets_rev[i] != (usize::MAX, usize::MAX) {
                // すでに決まっているターゲットはスキップ
                well_cnt[targets_rev[i].0] -= 1;
                continue;
            }

            let mut v = vec![(0.0, i)];
            for j in 1..250 {
                if i + j >= input.h {
                    break;
                }
                if targets_rev[i + j] != (usize::MAX, usize::MAX) {
                    continue;
                }
                let d = distance(input.target[i], input.target[i + j]);
                v.push((d, i + j));
            }
            v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for j in 0..pallet.sz_v.len() {
                if well_cnt[j] == 0 {
                    // 空いている場所を見つける
                    targets[j].push(vec![]);
                    for k in 0..lambda {
                        let idx = v[k].1;
                        targets[j].last_mut().unwrap().push(idx);
                        targets_rev[idx] = (j, targets[j].len() - 1);
                    }
                    well_cnt[j] += lambda;
                    break;
                }
            }
            well_cnt[targets_rev[i].0] -= 1;

        }
        // 初期化おわり
        SAState {
            targets,
            targets_rev,
        }
    }

    fn update(&mut self, input: &Input, pallet: &Pallet, rng: &mut ChaCha20Rng) {
        // ターゲットのインデックスを2つ選んで、入れるターゲットを入れ替える
        let mut x = 0;
        let mut y = 0;
        while x == y {
            x = rng.gen_range(0..input.h);
            y = rng.gen_range(0..input.h);
        }

        if self.targets_rev[x] == self.targets_rev[y] {
            // 同じウェルに入っている場合はスキップ
            return;
        }

        let (well_id_x, pos_id_x) = self.targets_rev[x];
        let (well_id_y, pos_id_y) = self.targets_rev[y];

        // x のターゲットを y のターゲットに入れ替える
        let idx_x = self.targets[well_id_x][pos_id_x].iter().position(|&v| v == x).unwrap();
        let idx_y = self.targets[well_id_y][pos_id_y].iter().position(|&v| v == y).unwrap();
        self.targets[well_id_x][pos_id_x][idx_x] = y;
        self.targets[well_id_y][pos_id_y][idx_y] = x;
        // targets_rev を更新
        self.targets_rev[x] = (well_id_y, pos_id_y);
        self.targets_rev[y] = (well_id_x, pos_id_x);
    }

    fn get_score(&self, input: &Input, pallet: &Pallet, lambda: usize, rng: &mut ChaCha20Rng) -> f64 {
        let mut res = 0.0;

        let mut well_cnt = vec![0; pallet.sz_v.len()];
        let mut well_ids = vec![0; pallet.sz_v.len()];
        for i in 0..input.h {
            let well_id = self.targets_rev[i].0;
            let (well_r, well_c) = pallet.pos_v[well_id];
            if well_cnt[well_id] == 0 {
                //eprintln!("Add {} {} {} {}", i, well_id, well_r, well_c);
                // ウェルに絵の具がなければ絵の具を追加
                // 絵の具を追加
                let mi_vol = State::get_mi_vol(
                    lambda,
                    &self.targets[well_id][self.targets_rev[i].1],
                    input,
                    rng,
                );

                let mut nowcol = [0.0; 3];
                let mut nowvol = 0.0;
                for j in 0..input.k {
                    let vol = mi_vol[j];
                    well_cnt[well_id] += vol;
                    well_ids[well_id] = self.targets_rev[i].1;
                    nowcol = mix(nowvol, nowcol, vol as f64, input.own[j]);
                    nowvol += vol as f64;
                }
                for &cand in &self.targets[well_id][self.targets_rev[i].1] {
                    res += distance(nowcol, input.target[cand]);
                }
            } else if well_ids[well_id] != self.targets_rev[i].1 {
                // 同じウェルに入っている場合はスキップ
                return 1e6;
            }

            // i 番目を配達
            // eprintln!("{} {} {}", i, well_r, well_c);
            // eprintln!("{}", self.vols[self.ids[well_r][well_c]]);
            well_cnt[well_id] -= 1;
        }
        
        res
    }
}

struct State {
    wall_v: Vec<Vec<bool>>, // n * n-1
    wall_h: Vec<Vec<bool>>, // n-1 * n
    ids: Vec<Vec<usize>>,
    caps: Vec<i64>,           // id に対応する範囲の容量
    vols: Vec<f64>,           // id に対応する各マスに入っている絵の具の量
    colors: Vec<[f64; 3]>,    // idに対応する範囲の色
    delivered: Vec<[f64; 3]>, //
    v: i64,
    error: f64,
    rng: ChaCha20Rng,
    n: usize, // inputのnを保持
    lambda: usize,
}

fn get_ids(wall_v: &Vec<Vec<bool>>, wall_h: &Vec<Vec<bool>>) -> (usize, Vec<Vec<usize>>, Vec<i64>) {
    let n = wall_v.len();
    let mut ids = vec![vec![usize::MAX; n]; n];
    let mut id = 0;
    let mut caps = vec![];
    for i in 0..n {
        for j in 0..n {
            if ids[i][j] != usize::MAX {
                continue;
            }
            let mut st = vec![(i, j)];
            ids[i][j] = id;
            let mut cap = 0;
            while let Some((i, j)) = st.pop() {
                cap += 1;
                if j + 1 < n && !wall_v[i][j] && ids[i][j + 1] == usize::MAX {
                    ids[i][j + 1] = id;
                    st.push((i, j + 1));
                }
                if i + 1 < n && !wall_h[i][j] && ids[i + 1][j] == usize::MAX {
                    ids[i + 1][j] = id;
                    st.push((i + 1, j));
                }
                if j > 0 && !wall_v[i][j - 1] && ids[i][j - 1] == usize::MAX {
                    ids[i][j - 1] = id;
                    st.push((i, j - 1));
                }
                if i > 0 && !wall_h[i - 1][j] && ids[i - 1][j] == usize::MAX {
                    ids[i - 1][j] = id;
                    st.push((i - 1, j));
                }
            }
            caps.push(cap);
            id += 1;
        }
    }
    (id, ids, caps)
}

fn mix(v1: f64, p1: [f64; 3], v2: f64, p2: [f64; 3]) -> [f64; 3] {
    let sum = v1 + v2;
    if sum <= 0.0 {
        return [0.0; 3];
    }
    [
        (v1 * p1[0] + v2 * p2[0]) / sum,
        (v1 * p1[1] + v2 * p2[1]) / sum,
        (v1 * p1[2] + v2 * p2[2]) / sum,
    ]
}

fn distance(p1: [f64; 3], p2: [f64; 3]) -> f64 {
    ((p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2) + (p1[2] - p2[2]).powi(2)).sqrt()
}

impl State {
    fn new(
        input: &Input,
        wall_v: &Vec<Vec<bool>>,
        wall_h: &Vec<Vec<bool>>,
        lambda: usize,
    ) -> State {
        let (id, ids, caps) = get_ids(wall_v, wall_h);
        let vols = vec![0.0; id];
        let colors = vec![[0.0; 3]; id];
        let rng = ChaCha20Rng::seed_from_u64(12345);

        State {
            wall_v: wall_v.clone(),
            wall_h: wall_h.clone(),
            ids,
            caps,
            vols,
            colors,
            delivered: vec![],
            v: 0,
            error: 0.0,
            rng,
            n: input.n,
            lambda,
        }
    }

    fn apply(&mut self, action: Action, input: &Input) {
        match action {
            Action::Add { i, j, k } => {
                self.v += 1;
                let id = self.ids[i][j];
                let w = self.caps[id] as f64 - self.vols[id];
                if w <= 1.0 {
                    self.colors[id] = mix(self.vols[id], self.colors[id], w, input.own[k]);
                    self.vols[id] = self.caps[id] as f64;
                } else {
                    self.colors[id] = mix(self.vols[id], self.colors[id], 1.0, input.own[k]);
                    self.vols[id] += 1.0;
                }
            }
            Action::Deliver { i, j } => {
                if self.delivered.len() >= input.h {
                    panic!("Too many deliveries");
                }

                if self.vols[self.ids[i][j]] < 1.0 - 1e-6 {
                    panic!("Cannot deliver less than 1.0 volume");
                }

                let color = self.colors[self.ids[i][j]];
                let target = input.target[self.delivered.len()];
                self.error += distance(color, target);
                self.vols[self.ids[i][j]] = (self.vols[self.ids[i][j]] - 1.0).max(0.0);
                self.delivered.push(color);
            }
            Action::Discard { i, j } => {
                self.vols[self.ids[i][j]] = (self.vols[self.ids[i][j]] - 1.0).max(0.0);
            }
            Action::Toggle { i1, j1, i2, j2 } => {
                if i1 == i2 {
                    let i = i1;
                    let j = j1.min(j2);
                    self.wall_v[i][j] ^= true;
                } else {
                    let i = i1.min(i2);
                    let j = j1;
                    self.wall_h[i][j] ^= true;
                }
                let (id, ids, caps) = get_ids(&self.wall_v, &self.wall_h);

                if self.ids[i1][j1] == self.ids[i2][j2] && ids[i1][j1] != ids[i2][j2] {
                    // split
                    let id1 = ids[i1][j1];
                    let id2 = ids[i2][j2];
                    let v = self.vols[self.ids[i1][j1]];
                    let mut vols = vec![0.0; id];
                    let mut colors = vec![[0.0; 3]; id];
                    for i in 0..self.n {
                        for j in 0..self.n {
                            vols[ids[i][j]] = self.vols[self.ids[i][j]];
                            colors[ids[i][j]] = self.colors[self.ids[i][j]];
                        }
                    }
                    vols[id1] = v * caps[id1] as f64 / (caps[id1] + caps[id2]) as f64;
                    vols[id2] = v * caps[id2] as f64 / (caps[id1] + caps[id2]) as f64;
                    self.ids = ids;
                    self.caps = caps;
                    self.vols = vols;
                    self.colors = colors;
                } else {
                    // merge
                    let id = ids[i1][j1];
                    let id1 = self.ids[i1][j1];
                    let id2 = self.ids[i2][j2];
                    let v1 = self.vols[id1];
                    let v2 = self.vols[id2];
                    let c1 = self.colors[id1];
                    let c2 = self.colors[id2];
                    let mut vols = vec![0.0; id];
                    let mut colors = vec![[0.0; 3]; id];
                    for i in 0..self.n {
                        for j in 0..self.n {
                            vols[ids[i][j]] = self.vols[self.ids[i][j]];
                            colors[ids[i][j]] = self.colors[self.ids[i][j]];
                        }
                    }
                    vols[id] = v1 + v2;
                    colors[id] = mix(v1, c1, v2, c2);
                    self.ids = ids;
                    self.caps = caps;
                    self.vols = vols;
                    self.colors = colors;
                }
            }
        }
    }

    fn gen_w(k: usize, rng: &mut ChaCha20Rng) -> Vec<Vec<f64>> {
        let mut res = vec![];
        let alpha = vec![1.0; k];
        for i in 0..8000 {
            let dir = Dirichlet::new(&alpha).unwrap();
            let w = dir.sample(rng);
            res.push(w);
        }
        res
    }

    fn dfs(now: usize, nowcol: [f64; 3], nowvol: i64, cnt: usize, lambda: usize, vol: &mut Vec<i64>, input: &Input, targets: &Vec<usize>, error_min: &mut f64, res: &mut Vec<i64>) {
        if now == input.k {
            if cnt == lambda {
                // ここで誤差を計算して、最小のものを res に保存
                let mut error = 0.0;
                for &target in targets {
                    error += distance(nowcol, input.target[target]);
                }
                if error < *error_min {
                    *error_min = error;
                    res.clear();
                    res.extend(vol.iter().cloned());
                }
            }
        } else {
            for i in 0..=lambda {
                if cnt + i > lambda {
                    break;
                }
                vol[now] = i as i64;
                let nxtcol = mix(nowvol as f64, nowcol, i as f64, input.own[now]);
                let nxtvol = nowvol + i as i64;
                State::dfs(now + 1, nxtcol, nxtvol, cnt + i, lambda, vol, input, targets, error_min, res);
                vol[now] = 0;
            }
        }
    }

    // 乱択でやってるけど lambda が小さかったら全探索できるはず
    fn get_mi_vol(
        lambda: usize,
        targets: &Vec<usize>,
        input: &Input,
        rng: &mut ChaCha20Rng,
    ) -> Vec<i64> {
        // DP に書き換える？
        // k <= 20
        // lambda <= 5
        // k から lambda 個選ぶときの、誤差の最小
        let mut vol = vec![0; input.k];
        let mut res = vec![];
        let mut error_min = f64::MAX;
        State::dfs(
            0,
            [0.0; 3],
            0,
            0,
            lambda,
            &mut vol,
            input,
            targets,
            &mut error_min,
            &mut res,
        );


        // 乱択のコードは残しておく
        // let ww = State::gen_w(input.k, rng);
        // let mut mi = f64::MAX;
        // let mut mi_vol = vec![0; input.k];
        // for w in &ww {
        //     let mut vol = vec![0; input.k];
        //     let mut r = vec![(0.0, 0); input.k];
        //     for j in 0..input.k {
        //         vol[j] = (lambda as f64 * w[j]).floor() as i64;
        //         r[j] = ((lambda as f64 * w[j]) - vol[j] as f64, j);
        //     }
        //     r.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        //     r.reverse();
        //     for j in 0..lambda - vol.iter().sum::<i64>() as usize {
        //         vol[r[j].1] += 1;
        //     }

        //     let mut nowv = vol[0];
        //     let mut nowcol = input.own[0];
        //     for j in 1..input.k {
        //         let nxtv = nowv + vol[j];
        //         let nxtcol = mix(nowv as f64, nowcol, vol[j] as f64, input.own[j]);
        //         nowv = nxtv;
        //         nowcol = nxtcol;
        //     }

        //     let mut error = 0.0;
        //     for &target in targets {
        //         error += distance(nowcol, input.target[target]);
        //     }
        //     if error < mi {
        //         mi = error;
        //         mi_vol = vol.clone();
        //     }
        // }
        res
    }

    fn calc_targets(&mut self, input: &Input, pallet: &Pallet, start_time: Instant) -> (Vec<Vec<Vec<usize>>>, Vec<(usize, usize)>) {
        // ここで焼きなまし
        // get_mi_vol と分けてるけど、使いながらでいいかも
        // さすがにこの State は要素多いので、焼く用の state を作るといいかも
        // 差分更新めんどくせ～～～
        // やっていくしかない
        // 貪欲だし今書いてたやつを初期解に使う
        // targets を焼きなましで得て、targets_rev はここで生やす
        // ビムサ説ない？

        let mut sa_state = SAState::new(input, pallet, self.lambda);
        // let time_limit = 3000 * 10 - 50;
        let time_limit = 0;
        let start_temp = 0.02;
        let end_temp = 0.0001;
        // eprintln!("{}", sa_state.get_score(input, pallet, self.lambda, &mut self.rng));
        loop {
            if start_time.elapsed().as_millis() > time_limit {
                break;
            }

            let temp = start_temp - (start_temp - end_temp) * (start_time.elapsed().as_millis() as f64 / time_limit as f64);
            let mut new_state = sa_state.clone();
            new_state.update(input, pallet, &mut self.rng);
            let new_score = new_state.get_score(input, pallet, self.lambda, &mut self.rng);
            let old_score = sa_state.get_score(input, pallet, self.lambda, &mut self.rng);
            let prob = (-(new_score - old_score) / temp).exp();
            if prob > self.rng.gen_range(0.0..1.0) {
                // eprintln!("{}", sa_state.get_score(input, pallet, self.lambda, &mut self.rng));
                sa_state = new_state;
            }
        }

        (sa_state.targets, sa_state.targets_rev)
    }

    fn solve(&mut self, input: &Input, pallet: &Pallet, start_time: Instant) -> Vec<Action> {
        // クラスタを確定させるところとアクションを実行するところは分けられるはず
        // クラスタ確定 -> ターゲットごとに追加と配達を実行

        // クラスタの表現 id -> に対するsz の配列を作成しておく
        // これがわかると id に対する pos の配列がわかる
        // sz に対して pos を一つ割り当てられればよい？
        // id にたいする残り容量もわかるとよい
        // target_id -> well_id -> pos の順に引ければいい
        // posごとに、ターゲットのリストをサイズごとに持つ

        self.lambda = 5;
        let (targets, targets_rev) = self.calc_targets(input, pallet, start_time);
        let mut actions = vec![];
        let mut well_cnt = vec![0; pallet.sz_v.len()];

        for i in 0..input.h {
            let well_id = targets_rev[i].0;
            let (well_r, well_c) = pallet.pos_v[well_id];
            if well_cnt[well_id] == 0 {
                //eprintln!("Add {} {} {} {}", i, well_id, well_r, well_c);
                // ウェルに絵の具がなければ絵の具を追加
                // 絵の具を追加
                let mi_vol = State::get_mi_vol(
                    self.lambda,
                    &targets[well_id][targets_rev[i].1],
                    input,
                    &mut self.rng,
                );

                for j in 0..input.k {
                    let vol = mi_vol[j];
                    for _ in 0..vol {
                        let add_action = Action::Add {
                            i: well_r,
                            j: well_c,
                            k: j,
                        };
                        actions.push(add_action.clone());
                        self.apply(add_action, input);
                        well_cnt[well_id] += 1;
                    }
                }
            }

            // i 番目を配達
            // eprintln!("{} {} {}", i, well_r, well_c);
            // eprintln!("{}", self.vols[self.ids[well_r][well_c]]);
            let deliver_action = Action::Deliver { i: well_r, j: well_c };
            actions.push(deliver_action.clone());
            self.apply(deliver_action, input);
            well_cnt[well_id] -= 1;
        }

        actions
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

    let start_time = Instant::now();

    let input = input();
    let n = input.n;
    let lambda = 5;

    let (pallet, wall_v, wall_h) = Pallet::new(&input);

    let mut state = State::new(&input, &wall_v, &wall_h, lambda);
    let actions = state.solve(&input, &pallet, start_time);
    output(&wall_v, &wall_h, &actions);
}

use std::time::Instant;

use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{input, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::*;
use std::io::{stdin, stdout, BufReader, Read, Stdin, Write};
use std::str::FromStr;
use std::thread::Thread;
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
