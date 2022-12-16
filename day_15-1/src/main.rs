use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug)]
struct Signal {
    sensor: Pos,
    beacon: Pos,
    distance: i64, // Manhattan distance
}

#[derive(Debug)]
struct Edges {
    t: i64,
    r: i64,
    b: i64,
    l: i64,
}

impl Signal {
    fn new(s: Pos, b: Pos) -> Self {
        Self {
            sensor: s,
            beacon: b,
            distance: Self::get_distance(&s, &b),
        }
    }

    fn get_distance(s: &Pos, b: &Pos) -> i64 {
        (s.x - b.x).abs() + (s.y - b.y).abs()
    }

    fn for_sure_not_beacon(&self, point: &Pos) -> bool {
        if self.beacon == *point {
            false
        } else {
            Self::get_distance(&self.sensor, point) <= self.distance
        }
    }
}

fn main() {
    let signals = fs::read_to_string("input")
        .expect("can't open the file")
        .lines()
        .map(parse_signal)
        .collect::<Vec<Signal>>();

    let edges = get_edges(&signals);

    let mut not_beacons: HashSet<Pos> = HashSet::new();
    // for row in edges.t..edges.b + 1 {
    for row in 2000000..2000000 + 1 {
        for col in edges.l..edges.r + 1 {
            for sig in &signals {
                if sig.for_sure_not_beacon(&Pos::new(col, row)) {
                    not_beacons.insert(Pos::new(col, row));
                    break;
                }
            }
        }
    }

    println!("{:?}", edges);
    // println!("{:?}", signals);
    println!("impossible points: {:?}", not_beacons.len());
}

fn parse_signal(line: &str) -> Signal {
    let (s, b) = line.split_once(": ").unwrap();

    let (s_x, s_y) = s.split_once(", ").unwrap();
    let (_, s_x) = s_x.split_once('=').unwrap();
    let (_, s_y) = s_y.split_once('=').unwrap();

    let (b_x, b_y) = b.split_once(", ").unwrap();
    let (_, b_x) = b_x.split_once('=').unwrap();
    let (_, b_y) = b_y.split_once('=').unwrap();

    Signal::new(
        Pos::new(s_x.parse::<i64>().unwrap(), s_y.parse::<i64>().unwrap()),
        Pos::new(b_x.parse::<i64>().unwrap(), b_y.parse::<i64>().unwrap()),
    )
}

fn get_edges(signals: &[Signal]) -> Edges {
    let top = signals
        .iter()
        .min_by(|a, b| (a.sensor.y - a.distance).cmp(&(b.sensor.y - b.distance)))
        .unwrap();
    let right = signals
        .iter()
        .max_by(|a, b| (a.sensor.x + a.distance).cmp(&(b.sensor.x + b.distance)))
        .unwrap();
    let bottom = signals
        .iter()
        .max_by(|a, b| (a.sensor.y + a.distance).cmp(&(b.sensor.y + b.distance)))
        .unwrap();
    let left = signals
        .iter()
        .min_by(|a, b| (a.sensor.x - a.distance).cmp(&(b.sensor.x - b.distance)))
        .unwrap();

    Edges {
        t: top.sensor.y - top.distance,
        r: right.sensor.x + right.distance,
        b: bottom.sensor.y + bottom.distance,
        l: left.sensor.x - left.distance,
    }
}
