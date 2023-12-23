const BATTERY_CAPACITY: usize = 30;

#[derive(Clone, Debug)]
struct CreatureInfo {
    id: usize,
    color: i32,
    ty: i32,
}

impl CreatureInfo {
    fn new(id: usize, color: i32, ty: i32) -> Self {
        Self { id, color, ty }
    }
}

#[derive(Clone, Debug)]
struct Creature {
    id: usize,
    r: i64,
    c: i64,
    vr: i64,
    vc: i64,
}

impl Creature {
    fn new(id: usize, r: i64, c: i64, vr: i64, vc: i64) -> Self {
        Self { id, r, c, vr, vc }
    }

    fn update(&mut self, r: i64, c: i64, vr: i64, vc: i64) {
        self.r = r;
        self.c = c;
        self.vr = vr;
        self.vc = vc;
    }
}

#[derive(Clone, Debug)]
struct Drone {
    id: usize,
    r: i64,
    c: i64,
    emergency: usize,
    battery: usize,
    scans: HashSet<usize>,
}

impl Drone {
    fn new(init_v: &Vec<usize>) -> Self {
        Self {
            id: init_v[0],
            r: init_v[2] as i64,
            c: init_v[1] as i64,
            emergency: init_v[3],
            battery: init_v[4],
            scans: HashSet::new(),
        }
    }

    fn update(&mut self, r: i64, c: i64, emergency: usize, battery: usize, scans: HashSet<usize>) {
        self.r = r;
        self.c = c;
        self.emergency = emergency;
        self.battery = battery;
        self.scans = scans;
    }
}

#[derive(Clone, Debug)]
struct Player {
    score: usize,
    drones: HashMap<usize, Drone>,
    saved_scans: HashSet<usize>,
}

impl Player {
    fn new(drones: &HashMap<usize, Drone>, saved_scans: &HashSet<usize>) -> Self {
        Self {
            score: 0,
            drones: drones.clone(),
            saved_scans: saved_scans.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct Field {
    visible_creatures: HashMap<usize, Creature>,
}

impl Field {
    fn new(visible_creatures: &Vec<Vec<i64>>) -> Self {
        let mut res = Self {
            visible_creatures: HashMap::new(),
        };
        for creature_info in visible_creatures {
            let id = creature_info[0] as usize;
            let c = creature_info[1];
            let r = creature_info[2];
            let vc = creature_info[3];
            let vr = creature_info[4];
            res.visible_creatures
                .insert(id, Creature::new(id, r, c, vr, vc));
        }
        res
    }

    fn update(&mut self, visible_creatures: &Vec<Vec<i64>>) {
        for creature_info in visible_creatures {
            let id = creature_info[0] as usize;
            let c = creature_info[1];
            let r = creature_info[2];
            let vc = creature_info[3];
            let vr = creature_info[4];
            let creature = self.visible_creatures.get_mut(&id);
            match creature {
                Some(creature) => {
                    creature.update(r, c, vr, vc);
                }
                None => {
                    self.visible_creatures
                        .insert(id, Creature::new(id, r, c, vr, vc));
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    me: Player,
    enemy: Player,
    field: Field,
}

impl State {
    fn dummy() -> Self {
        Self {
            me: Player::new(&HashMap::new(), &HashSet::new()),
            enemy: Player::new(&HashMap::new(), &HashSet::new()),
            field: Field::new(&Vec::new()),
        }
    }

    fn new(input_data: &InputData) -> Self {
        let visible_creatures = &input_data.visible_creatures;
        let my_drones = input_data
            .my_drones
            .iter()
            .map(|v| (v[0], Drone::new(v)))
            .collect::<HashMap<_, _>>();
        let foe_drones = input_data
            .foe_drones
            .iter()
            .map(|v| (v[0], Drone::new(v)))
            .collect::<HashMap<_, _>>();
        let mut res = Self {
            me: Player::new(&my_drones, &input_data.my_scans),
            enemy: Player::new(&foe_drones, &input_data.foe_scans),
            field: Field::new(visible_creatures),
        };

        for drone_scan in &input_data.drone_scans {
            let drone_id = drone_scan[0];
            let creature_id = drone_scan[1];
            res.update_drone_scan(drone_id, creature_id);
        }

        res
    }

    fn update(&mut self, input_data: &InputData) {
        let visible_creatures = &input_data.visible_creatures;
        let my_drones = input_data
            .my_drones
            .iter()
            .map(|v| (v[0], Drone::new(v)))
            .collect::<HashMap<_, _>>();
        let foe_drones = input_data
            .foe_drones
            .iter()
            .map(|v| (v[0], Drone::new(v)))
            .collect::<HashMap<_, _>>();
        self.me = Player::new(&my_drones, &input_data.my_scans);
        self.enemy = Player::new(&foe_drones, &input_data.foe_scans);
        self.field.update(visible_creatures);

        for drone_scan in &input_data.drone_scans {
            let drone_id = drone_scan[0];
            let creature_id = drone_scan[1];
            self.update_drone_scan(drone_id, creature_id);
        }
    }

    fn update_drone_scan(&mut self, drone_id: usize, creature_id: usize) {
        let drone = self.me.drones.get_mut(&drone_id);
        match drone {
            Some(drone) => {
                drone.scans.insert(creature_id);
            }
            None => (),
        }
    }
}

#[derive(Clone, Debug)]
struct InputData {
    my_score: usize,
    foe_score: usize,
    my_scan_count: usize,
    my_scans: HashSet<usize>,
    foe_scan_count: usize,
    foe_scans: HashSet<usize>,
    my_drone_count: usize,
    my_drones: Vec<Vec<usize>>, // drone_id, drone_x, drone_y, emergency, battery
    foe_drone_count: usize,
    foe_drones: Vec<Vec<usize>>,
    drone_scan_count: usize,
    drone_scans: Vec<Vec<usize>>, // drone_id, creature_id
    visible_creature_count: usize,
    visible_creatures: Vec<Vec<i64>>, // id, x, y, vx, vy
    radar_blip_count: usize,
    radar_blips: Vec<Vec<String>>, // drone_id, creature_id, radar
}

impl InputData {
    fn new() -> Self {
        Self {
            my_score: 0,
            foe_score: 0,
            my_scan_count: 0,
            my_scans: HashSet::new(),
            foe_scan_count: 0,
            foe_scans: HashSet::new(),
            my_drone_count: 0,
            my_drones: Vec::new(),
            foe_drone_count: 0,
            foe_drones: Vec::new(),
            drone_scan_count: 0,
            drone_scans: Vec::new(),
            visible_creature_count: 0,
            visible_creatures: Vec::new(),
            radar_blip_count: 0,
            radar_blips: Vec::new(),
        }
    }

    fn read(&mut self, reader: &mut impl BufRead) -> Result<(), String> {
        self.my_score = input(reader)?;
        self.foe_score = input(reader)?;
        self.my_scan_count = input(reader)?;
        for _ in 0..self.my_scan_count {
            let info = input(reader)?;
            self.my_scans.insert(info);
        }
        self.foe_scan_count = input(reader)?;
        for _ in 0..self.foe_scan_count {
            let info = input(reader)?;
            self.foe_scans.insert(info);
        }
        self.my_drone_count = input(reader)?;
        for _ in 0..self.my_drone_count {
            let info = input_vec::<usize>(reader)?;
            self.my_drones.push(info);
        }
        self.foe_drone_count = input(reader)?;
        for _ in 0..self.foe_drone_count {
            let info = input_vec::<usize>(reader)?;
            self.foe_drones.push(info);
        }
        self.drone_scan_count = input(reader)?;
        for _ in 0..self.drone_scan_count {
            let info = input_vec::<usize>(reader)?;
            self.drone_scans.push(info);
        }
        self.visible_creature_count = input(reader)?;
        for _ in 0..self.visible_creature_count {
            let info = input_vec::<i64>(reader)?;
            self.visible_creatures.push(info);
        }
        self.radar_blip_count = input(reader)?;
        for _ in 0..self.radar_blip_count {
            let info = input_vec::<String>(reader)?;
            self.radar_blips.push(info);
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let mut reader = BufReader::new(stdin().lock());
    let creature_count: usize = input(&mut reader)?;
    let mut creature_mp: HashMap<usize, CreatureInfo> = HashMap::new();
    for _ in 0..creature_count {
        let info = input_vec::<i32>(&mut reader)?;
        creature_mp.insert(info[0] as usize, CreatureInfo::new(info[0] as usize, info[1], info[2]));
    }


    let mut points = vec![vec![
        (2000, 8650),
        (8000, 8650),
        (5000, 500),
        (2000, 6250),
        (8000, 6250),
        (5000, 500),
        (2000, 3650),
        (8000, 3650),
        (5000, 500),
    ], vec![
        (8000, 8650),
        (2000, 8650),
        (5000, 500),
        (8000, 6250),
        (2000, 6250),
        (5000, 500),
        (8000, 3650),
        (2000, 3650),
        (5000, 500),
    ]
    ];
    let mut next_p = 0;

    let mut id_mp = HashMap::new();
    let mut id = 0;

    let mut state = State::dummy();
    let mut cnt = 0;
    loop {
        cnt += 1;
        eprintln!("Start turn: {}", cnt);
        let mut input_data = InputData::new();
        input_data.read(&mut reader)?;
        //eprintln!("{:?}", input_data);
        if cnt == 1 {
            state = State::new(&input_data);
            for drone in input_data.my_drones.iter() {
                let i = drone[0];
                eprintln!("i: {}, id: {}", i, id);
                id_mp.insert(i, id);
                id += 1;
            }
        } else {
            state.update(&input_data);
        }

        // 初期解
        // (2000, 8650) から (8000, 8650) まで走査
        // (5000, 500) へ浮上
        // (2000, 6250) から (8000, 6250) まで走査
        // (5000, 500) へ浮上
        // (2000, 3650) から (8000, 3650) まで走査
        // (5000, 500) へ浮上

        let mut upd = true;
        for (&id, drone) in state.me.drones.iter() {
            // 今は一個だけ
            let idx = *id_mp.get(&id).unwrap();
            let (next_c, next_r) = points[idx][next_p];
            eprintln!("id: {}, next_c: {}, next_r: {}", id, next_c, next_r);
            eprintln!("drone_r: {}, drone_c: {}", drone.r, drone.c);
            if !(drone.r == next_r && drone.c == next_c) {
                upd = false;
            }
        }
        eprintln!("upd: {}", upd);
        if upd {
            next_p += 1;
            if next_p == points[0].len() {
                next_p = 0;
            }
        }


        let mut ans = vec![String::new(); 2];
        for &id in state.me.drones.keys() {
            let idx = *id_mp.get(&id).unwrap();
            let (next_c, next_r) = points[idx][next_p];
            ans[idx] = format!("MOVE {} {} 1", next_c, next_r);
        }
        println!("{}", ans[0]);
        println!("{}", ans[1]);

        // 方針
        // レーダーを使えばクリーチャーの場所は大体わかる
        // (切り替わるところで直進方向にクリーチャーがいることが判明するから、201 ずつ変化するように動けばいい)

        // type2 から順に、全部揃えたボーナスを狙っていく
        // 開始直後または save 直後であれば、(2000, Ymin + 1250) と (10000 - 2000, Ymin + 1250) の近いほうまで移動する (前者の場合について書く)
        // もし左側にクリーチャーがいれば、回収する
        // まっすぐ右へ進んで、魚をすべて回収する。
        // type を全部揃えたか敵のY座標が5500 を超えた時点で一度浮上し、セーブする。
        // もし全部揃っていなければ同じ戦略でもう一度探索する。
        // type1, type0 を同じ戦略で探索する。ただしtypeを全部集めるまで浮上しない

        eprintln!("start output");
        eprintln!("end turn: {}", cnt);
    }
}

use std::cmp::{max, min};
use std::collections::*;
use std::fmt::Display;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::str::FromStr;

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
fn input<T>(reader: &mut impl BufRead) -> Result<T, String>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).map_err(|e| e.to_string())?;
    buffer
        .trim()
        .parse()
        .map_err(|err: <T as FromStr>::Err| err.to_string())
}

/// 一行の複数の値を入力する
fn input_vec<T>(reader: &mut impl BufRead) -> Result<Vec<T>, String>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).map_err(|e| e.to_string())?;
    let v = buffer
        .trim()
        .split(" ")
        .map(|e| e.parse::<T>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<_>, String>>();
    v
}

// TODO: 複数の型が入り得る場合を処理したい（どうやって？）
/// 複数行を入力する
fn input_lines<T>(n: usize, reader: &mut impl BufRead) -> Result<Vec<T>, String>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let mut v: Vec<T> = Vec::new();
    for i in 0..n {
        let val = input(reader)?;
        v.push(val);
    }
    Ok(v)
}
