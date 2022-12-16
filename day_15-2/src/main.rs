use std::fs;

const MAX_SIZE: i64 = 4000000;

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
    distance: i64, // Manhattan distance

    top: Pos,
    right: Pos,
    bottom: Pos,
    left: Pos,
}

impl Signal {
    fn new(s: Pos, beacon: Pos) -> Self {
        let distance = Self::get_distance(&s, &beacon);

        Self {
            sensor: s,
            distance,
            // First edges after the perimeter
            top: Pos::new(s.x, s.y - distance - 1),
            right: Pos::new(s.x + distance + 1, s.y),
            bottom: Pos::new(s.x, s.y + distance + 1),
            left: Pos::new(s.x - distance - 1, s.y),
        }
    }

    fn get_distance(s: &Pos, b: &Pos) -> i64 {
        (s.x - b.x).abs() + (s.y - b.y).abs()
    }

    fn is_in_range(&self, point: &Pos) -> bool {
        Self::get_distance(&self.sensor, point) <= self.distance
    }

    fn get_edges(&self) -> Vec<Pos> {
        let mut edges = vec![];
        edges.append(&mut Self::get_single_edge(&self.top, &self.right));
        edges.append(&mut Self::get_single_edge(&self.right, &self.bottom));
        edges.append(&mut Self::get_single_edge(&self.bottom, &self.left));
        edges.append(&mut Self::get_single_edge(&self.left, &self.top));

        edges
    }
    fn get_single_edge(start: &Pos, end: &Pos) -> Vec<Pos> {
        let mut edges = vec![];

        let step = get_step(start, end);
        let mut curr_pos = Pos::new(start.x, start.y);

        while curr_pos != *end {
            if curr_pos.x >= 0 && curr_pos.y >= 0
                && curr_pos.y <= MAX_SIZE && curr_pos.y <= MAX_SIZE {
                edges.push(Pos::new(curr_pos.x, curr_pos.y));
            }
            curr_pos = step(&curr_pos);
        }

        edges
    }
}

fn main() {
    let signals = fs::read_to_string("input")
        .expect("can't open the file")
        .lines()
        .map(parse_signal)
        .collect::<Vec<Signal>>();

    'get_sig_edges: for sig in &signals {
        'check_edges: for edge in sig.get_edges().iter() {
            for sig in &signals {
                if sig.is_in_range(&Pos::new(edge.x, edge.y)) {
                    continue 'check_edges;
                }
            }
            println!(
                "point: {:?}, tuning frequency: {}",
                Pos::new(edge.x, edge.y),
                edge.x * 4000000 + edge.y
            );
            break 'get_sig_edges;
        }
    }
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

fn get_step(start: &Pos, end: &Pos) -> Box<dyn Fn(&Pos) -> Pos> {
    let x_step = if start.x > end.x { -1 } else { 1 };
    let y_step = if start.y > end.y { -1 } else { 1 };

    Box::new(move |curr| Pos::new(curr.x + x_step, curr.y + y_step))
}
