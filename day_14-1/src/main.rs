// Recursion, sand dynamics, closures
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Edges {
    l: usize,
    r: usize,
    b: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

fn main() {
    let mut walls = fs::read_to_string("input")
        .expect("can't open the file")
        .lines()
        .flat_map(|line| get_shape(&get_points(line)))
        .collect::<HashSet<Pos>>();

    let edges = get_edges(&walls);

    let mut sand = Pos::new(500, 0);
    let mut sand_travel: Vec<Pos> = vec![sand];
    let mut sand_count = 0;
    'next_sand: while !walls.contains(&Pos::new(500, 0)) {
        if sand.y == edges.b + 1 {
            walls.insert(sand);
            sand_count += 1;
            sand = sand_travel.pop().unwrap();
        } else {
            for x in [sand.x, sand.x - 1, sand.x + 1] {
                if !walls.contains(&Pos::new(x, sand.y + 1)) {
                    sand_travel.push(sand);
                    sand.x = x;
                    sand.y += 1;
                    continue 'next_sand;
                }
            }
            walls.insert(sand);
            sand_count += 1;
            sand = sand_travel.pop().unwrap();
        }
    }

    print_map(&walls, &edges);
    println!();

    println!("sand count: {}", sand_count);
}

fn get_points(s: &str) -> Vec<Pos> {
    s.split(" -> ")
        .map(|pos| {
            let (x, y) = pos.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            Pos::new(x, y)
        })
        .collect()
}

fn get_shape(points: &Vec<Pos>) -> Vec<Pos> {
    let mut shape: Vec<Pos> = vec![points[0]];

    for i in 0..points.len() - 1 {
        let mut curr = points[i];
        let end = points[i + 1];

        shape.push(curr);
        let step = get_step(&curr, &end);
        while curr != end {
            curr = step(&curr);
            shape.push(curr);
        }
    }
    shape
}

fn get_edges(walls: &HashSet<Pos>) -> Edges {
    walls.iter().fold(
        Edges {
            l: usize::MAX,
            r: 0,
            b: 0,
        },
        |edges: Edges, wall| {
            let mut new_edges = edges;

            if wall.x < new_edges.l {
                new_edges.l = wall.x;
            } else if wall.x > new_edges.r {
                new_edges.r = wall.x;
            }

            if wall.y > new_edges.b {
                new_edges.b = wall.y;
            }
            new_edges
        },
    )
}

fn get_step(start: &Pos, end: &Pos) -> Box<dyn Fn(&Pos) -> Pos> {
    if start.x > end.x {
        Box::new(|curr| Pos::new(curr.x - 1, curr.y))
    } else if start.x < end.x {
        Box::new(|curr| Pos::new(curr.x + 1, curr.y))
    } else if start.y > end.y {
        Box::new(|curr| Pos::new(curr.x, curr.y - 1))
    } else if start.y < end.y {
        Box::new(|curr| Pos::new(curr.x, curr.y + 1))
    } else {
        Box::new(|curr| Pos::new(curr.x, curr.y))
    }
}

fn print_map(walls: &HashSet<Pos>, edges: &Edges) {
    let col_count = edges.r - edges.l + 1;
    let row_count = edges.b + 3;
    let mut map: Vec<Vec<char>> = vec![vec!['.'; col_count]; row_count];

    for wall in walls {
        if edges.l <= wall.x && edges.r >= wall.x {
            map[wall.y][wall.x - edges.l] = '#';
        }
    }

    map[0][500 - edges.l] = 'O';

    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
