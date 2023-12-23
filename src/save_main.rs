#![allow(unused)]

const BATTERY_CAPACITY: usize = 30;

#[derive(Clone, Debug)]
struct Creature {
    id: usize,
    color: usize,
    ty: usize,
    r: i64,
    c: i64,
    vr: i64,
    vc: i64,
}

impl Creature {
    fn new(id: usize, color: usize, ty: usize) -> Self {
        Self {
            id,
            color,
            ty,
            r: 0,
            c: 0,
            vr: 0,
            vc: 0,
        }
    }

    fn update(&mut self, r: i64, c: i64, vr: i64, vc: i64) {
        self.r = r;
        self.c = c;
        self.vr = vr;
        self.vc = vc;
    }
}

#[derive(Clone, Debug)]
struct Player {
    id: usize,
    score: usize,
    r: i64,
    c: i64,
    battery: usize,
    creatures: Vec<usize>,
}

impl Player {
    fn new(id: usize, r: i64, c: i64) -> Self {
        Self {
            id,
            score: 0,
            r,
            c,
            battery: BATTERY_CAPACITY,
            creatures: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct Drone {
    id: usize,
    r: i64,
    c: i64,
    emergency: usize,
    battery: usize,
}

impl Drone {
    fn new(init_v: Vec<usize>) -> Self {
        Self {
            id: init_v[0],
            r: init_v[2] as i64,
            c: init_v[1] as i64,
            emergency: init_v[3],
            battery: init_v[4],
        }
    }
}

#[derive(Clone, Debug)]
struct Field {
    creatures: HashMap<usize, Creature>,
}

impl Field {
    fn new(creatures: &Vec<Vec<usize>>, creature_mp: &HashMap<usize, Creature>) -> Self {
        let mut res = Self {
            creatures: HashMap::new(),
        };
        for creature_info in creatures {
            let id = creature_info[0];
            let c = creature_info[1] as i64;
            let r = creature_info[2] as i64;
            let vc = creature_info[3] as i64;
            let vr = creature_info[4] as i64;
            let mut creature = creature_mp.get(&id).unwrap().clone();
            creature.update(r, c, vr, vc);
            res.creatures.insert(id, creature.clone());
        }
        res
    }

    fn update(&mut self, creatures: &Vec<Vec<usize>>) {
        for creature_info in creatures {
            let id = creature_info[0];
            let c = creature_info[1] as i64;
            let r = creature_info[2] as i64;
            let vc = creature_info[3] as i64;
            let vr = creature_info[4] as i64;
            let mut creature = self.creatures.get_mut(&id);
            match creature {
                Some(creature) => {
                    creature.update(r, c, vr, vc);
                }
                None => {
                    let mut creature = Creature::new(id, creature_info[1], creature_info[2]);
                    creature.update(r, c, vr, vc);
                    self.creatures.insert(id, creature);
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
            me: Player::new(0, 0, 0),
            enemy: Player::new(0, 0, 0),
            field: Field::new(&Vec::new(), &HashMap::new()),
        }
    }

    fn new(input_data: &InputData, creature_mp: &HashMap<usize, Creature>) -> Self {
        let my_drone = &input_data.my_drones[0];
        let enemy_drone = &input_data.foe_drones[0];
        let creatures = &input_data.visible_creatures;
        Self {
            me: Player::new(my_drone.id, my_drone.r, my_drone.c),
            enemy: Player::new(enemy_drone.id, enemy_drone.r, enemy_drone.c),
            field: Field::new(creatures, &creature_mp),
        }
    }

    fn update(&mut self, input_data: &InputData) {
        let my_drone = &input_data.my_drones[0];
        let enemy_drone = &input_data.foe_drones[0];
        let creatures = &input_data.visible_creatures;
        self.me = Player::new(my_drone.id, my_drone.r, my_drone.c);
        self.enemy = Player::new(enemy_drone.id, enemy_drone.r, enemy_drone.c);
        self.field.update(&creatures);
    }
}

#[derive(Clone, Debug)]
struct InputData {
    my_score: usize,
    foe_score: usize,
    my_scan_count: usize,
    my_scans: Vec<usize>,
    foe_scan_count: usize,
    foe_scans: Vec<usize>,
    my_drone_count: usize,
    my_drones: Vec<Drone>,
    foe_drone_count: usize,
    foe_drones: Vec<Drone>,
    drone_scan_count: usize,
    drone_scans: Vec<Vec<usize>>, // drone_id, creature_id
    visible_creature_count: usize,
    visible_creatures: Vec<Vec<usize>>, // id, x, y, vx, vy
    radar_blip_count: usize,
    radar_blips: Vec<Vec<String>>, // drone_id, creature_id, radar
}

impl InputData {
    fn new() -> Self {
        Self {
            my_score: 0,
            foe_score: 0,
            my_scan_count: 0,
            my_scans: Vec::new(),
            foe_scan_count: 0,
            foe_scans: Vec::new(),
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

    fn read(&mut self, creature_count: usize) -> Result<(), String> {
        self.my_score = input(reader)?;
        self.foe_score = input(reader)?;
        self.my_scan_count = input(reader)?;
        for i in 0..self.my_scan_count {
            let info = input(reader)?;
            self.my_scans.push(info);
        }
        self.foe_scan_count = input(reader)?;
        for i in 0..self.foe_scan_count {
            let info = input(reader)?;
            self.foe_scans.push(info);
        }
        self.my_drone_count = input(reader)?;
        for i in 0..self.my_drone_count {
            let info = input_vec::<usize>(reader)?;
            self.my_drones.push(Drone::new(info));
        }
        self.foe_drone_count = input(reader)?;
        for i in 0..self.foe_drone_count {
            let info = input_vec::<usize>(reader)?;
            self.foe_drones.push(Drone::new(info));
        }
        self.drone_scan_count = input(reader)?;
        for i in 0..self.drone_scan_count {
            let info = input_vec::<usize>(reader)?;
            self.drone_scans.push(info);
        }
        self.visible_creature_count = input(reader)?;
        for i in 0..self.visible_creature_count {
            let info = input_vec::<usize>(reader)?;
            self.visible_creatures.push(info);
        }
        self.radar_blip_count = input(reader)?;
        for i in 0..self.radar_blip_count {
            let info = input_vec::<String>(reader)?;
            self.radar_blips.push(info);
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let creature_count: usize = input(reader)?;
    let mut creature_mp: HashMap<usize, Creature> = HashMap::new();
    for i in 0..creature_count {
        let info = input_vec::<usize>(reader)?;
        creature_mp.insert(info[0], Creature::new(info[0], info[1], info[2]));
    }

    let mut state = State::dummy();
    let mut cnt = 0;
    loop {
        let mut input_data = InputData::new();
        input_data.read(creature_count);
        eprintln!("{:?}", input_data);
        if cnt == 0 {
            state = State::new(&input_data, &creature_mp);
        } else {
            state.update(&input_data);
        }

        // 一番近い魚を探す
        let mut mi = i64::MAX;
        let mut mi_id = usize::MAX;
        let my_r = state.me.r;
        let my_c = state.me.c;
        for (_, creature) in state.field.creatures.iter() {
            let dist = (creature.r - my_r) * (creature.r - my_r)
                + (creature.c - my_c) * (creature.c - my_c);

            if dist < mi {
                mi = dist;
                mi_id = creature.id;
            }
        }

        eprintln!("{:?}", state.field.creatures);
        eprintln!("{} {}", mi, mi_id);

        // 一番近い魚に向かう
        let destination = state.field.creatures.get(&mi_id);
        match destination {
            Some(creature) => {
                println!("MOVE {} {} 1", creature.r, creature.c);
            }
            None => {
                println!("WAIT 1");
            }
        }
    }
}

use std::cmp::{max, min};
use std::collections::*;
use std::fmt::Display;
use std::io::{stdin, stdout, BufReader, Write};
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
fn input<T>() -> Result<T, String>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;
    buffer
        .trim()
        .parse()
        .map_err(|err: <T as FromStr>::Err| err.to_string())
}

/// 一行の複数の値を入力する
fn input_vec<T>() -> Result<Vec<T>, String>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).map_err(|e| e.to_string())?;
    let v = buffer
        .trim()
        .split(" ")
        .map(|e| e.parse::<T>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<_>, String>>();
    v
}

// TODO: 複数の型が入り得る場合を処理したい（どうやって？）
/// 複数行を入力する
fn input_lines<T>(n: usize) -> Result<Vec<T>, String>
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
