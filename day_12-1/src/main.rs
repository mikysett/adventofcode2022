// Pathfinding, A* algo
use colored::Colorize;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::{thread, time};

type Height = usize;

const START: Height = 'a' as Height - 1;
const END: Height = 'z' as Height + 1;

struct Map {
    nodes: HashMap<Pos, Box<Node>>,
    size: Pos,
    start: Pos,
    end: Pos,
}

#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Node {
    h: Height,
    g_cost: usize,
    h_cost: usize,
    f_cost: usize,
    status: NodeStatus,
    neighbors: Vec<Pos>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum NodeStatus {
    Unvisited,
    Visited,
    Calculated,
}

impl Map {
    fn init_from_heights(heights_map: &Vec<Vec<Height>>) -> Map {
        let mut map = Map {
            nodes: HashMap::new(),
            size: Pos::new(heights_map[0].len(), heights_map.len()),
            start: Pos::new(0, 0),
            end: Pos::new(0, 0),
        };

        for y in 0..map.size.y {
            for x in 0..map.size.x {
                let curr_height = heights_map[y][x];
                let curr_pos = Pos::new(x, y);

                match curr_height {
                    START => map.start = curr_pos,
                    END => map.end = curr_pos,
                    _ => (),
                }

                map.nodes.insert(
                    curr_pos,
                    Box::new(Node {
                        h: curr_height,
                        h_cost: 0,
                        g_cost: Height::MAX,
                        f_cost: Height::MAX,
                        status: NodeStatus::Unvisited,
                        neighbors: Map::set_neighbors(heights_map, &map.size, &curr_pos),
                    }),
                );

                for (pos, node) in &mut map.nodes {
                    node.h_cost = Map::set_h_cost(pos, &map.end);
                }
            }
        }

        map
    }

    fn reset_distances(&mut self, start: &Pos) {
        self.start = *start;
        for (pos, node) in &mut self.nodes {
            node.g_cost = Height::MAX;
            node.f_cost = Height::MAX;
            node.status = NodeStatus::Unvisited;
        }

        let mut start_node = self.get_mut_node(start);
        start_node.status = NodeStatus::Visited;
        start_node.g_cost = 0;
        start_node.f_cost = start_node.h_cost;
    }

    fn set_neighbors(heights_map: &Vec<Vec<Height>>, size: &Pos, pos: &Pos) -> Vec<Pos> {
        let mut neighbors: Vec<Pos> = vec![];
        if pos.x != 0 && heights_map[pos.y][pos.x] >= heights_map[pos.y][pos.x - 1] - 1 {
            neighbors.push(Pos::new(pos.x - 1, pos.y))
        }
        if pos.x + 1 < size.x && heights_map[pos.y][pos.x] >= heights_map[pos.y][pos.x + 1] - 1 {
            neighbors.push(Pos::new(pos.x + 1, pos.y))
        }

        if pos.y != 0 && heights_map[pos.y][pos.x] >= heights_map[pos.y - 1][pos.x] - 1 {
            neighbors.push(Pos::new(pos.x, pos.y - 1))
        }
        if pos.y + 1 < size.y && heights_map[pos.y][pos.x] >= heights_map[pos.y + 1][pos.x] - 1 {
            neighbors.push(Pos::new(pos.x, pos.y + 1))
        }

        neighbors
    }

    fn set_h_cost(node: &Pos, end: &Pos) -> usize {
        ((end.x as i32 - node.x as i32).abs() + (end.y as i32 - node.y as i32).abs())
            .try_into()
            .unwrap()
    }

    fn calculate_distance(&mut self, start: Pos) -> usize {
        self.reset_distances(&start);
        let start_node = self.nodes.get_mut(&start).unwrap();
        let mut calculated: Vec<(Pos, usize)> = vec![(start, start_node.f_cost)];

        while self.nodes.get_mut(&self.end).unwrap().g_cost == usize::MAX {
            // No possible paths
            if calculated.len() == 0 {
                return Height::MAX;
            }

            calculated.sort_unstable_by_key(|c_node| -(c_node.1 as i32));

            let mut sel_node = self.nodes.get_mut(&calculated.pop().unwrap().0).unwrap();
            let old_g_cost = sel_node.g_cost;
            sel_node.status = NodeStatus::Visited;
            let neighbors = sel_node.neighbors.iter().map(|x| *x).collect::<Vec<Pos>>();
            for i in neighbors {
                let mut node_to_calc = self.nodes.get_mut(&i).unwrap();

                if node_to_calc.g_cost > old_g_cost + 1 {
                    node_to_calc.g_cost = old_g_cost + 1;
                    node_to_calc.f_cost = node_to_calc.g_cost + node_to_calc.h_cost;
                    node_to_calc.status = NodeStatus::Calculated;
                    calculated.push((i, node_to_calc.f_cost));
                }
            }
        }

        self.nodes.get(&self.end).unwrap().f_cost
    }

    fn get_mut_node(&mut self, pos: &Pos) -> &mut Box<Node> {
        self.nodes.get_mut(pos).unwrap()
    }
}

fn c_to_height(c: char) -> Height {
    match c {
        'S' => START,
        'E' => END,
        _ => c as Height,
    }
}

fn main() {
    let heights_map = fs::read_to_string("input")
        .expect("Error")
        .lines()
        .map(|line| line.chars().map(c_to_height).collect::<Vec<Height>>())
        .collect::<Vec<Vec<Height>>>();

    let mut graph = Map::init_from_heights(&heights_map);

    // Part 1
    let distance = graph.calculate_distance(graph.start);
    println!("{}\n", graph);
    println!("{}", distance);

    // Part 2
    let trail_starts = graph
        .nodes
        .iter()
        .filter(|(_, node)| node.h <= 'a' as Height)
        .map(|(pos, _)| *pos)
        .collect::<Vec<Pos>>();

    let shortest_trail = trail_starts
        .iter()
        .map(|new_start| graph.calculate_distance(*new_start))
        .min()
        .unwrap();
    println!("{}", shortest_trail);
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = self
            .nodes
            .iter()
            .map(|(key, node)| (*key, node.status, char::from_u32(node.h as u32).unwrap()))
            .collect::<Vec<(Pos, NodeStatus, char)>>();
        map.sort_unstable_by_key(|val| Pos::new(val.0.y, val.0.x));

        write!(
            f,
            "{}",
            map.iter()
                .map(|(pos, status, height)| {
                    let eof = if pos.x == self.size.x - 1 { "\n" } else { "" };
                    match status {
                        NodeStatus::Calculated => format!("{}{}", height.to_string().yellow(), eof),
                        NodeStatus::Visited => format!("{}{}", height.to_string().green(), eof),
                        _ => format!("{}{}", height, eof),
                    }
                })
                .collect::<String>()
        )
    }
}
