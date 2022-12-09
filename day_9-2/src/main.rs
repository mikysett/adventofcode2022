use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Pos(i32, i32);

fn main() {
    let mut t_pos: [Pos; 10] = [Pos(0, 0); 10];

    let mut visited_pos = fs::read_to_string("input")
        .expect("Error")
        .lines()
        .flat_map(|line| {
            let mut visited_pos: Vec<Pos> = vec![];
            let (dir, nb_moves) = line.split_once(' ').unwrap();
            let nb_moves = nb_moves.parse::<i32>().unwrap();

            let step = match dir {
                "U" => Pos(0, -1),
                "R" => Pos(1, 0),
                "D" => Pos(0, 1),
                _ => Pos(-1, 0),
            };
            for _ in 0..nb_moves {
                t_pos[0] = Pos(t_pos[0].0 + step.0, t_pos[0].1 + step.1);
                for i in 1..9 {
                    t_pos[i] = new_tail_position(&t_pos[i], &t_pos[i - 1]);
                }
                let last_pos = new_tail_position(&t_pos[9], &t_pos[8]);
                if last_pos != t_pos[9] {
                    t_pos[9] = last_pos;
                    visited_pos.push(t_pos[9]);
                }
            }
            visited_pos
        })
        .collect::<Vec<Pos>>();

    visited_pos.sort();
    visited_pos.dedup();
    println!("{:?}", visited_pos.len());
}

fn new_tail_position(t_pos: &Pos, h_pos: &Pos) -> Pos {
    let mut t_pos = Pos(t_pos.0, t_pos.1);

    if t_pos.0 == h_pos.0 {
        if t_pos.1 > h_pos.1 + 1 {
            t_pos.1 -= 1;
        } else if t_pos.1 < h_pos.1 - 1 {
            t_pos.1 += 1;
        }
    } else if t_pos.1 == h_pos.1 {
        if t_pos.0 > h_pos.0 + 1 {
            t_pos.0 -= 1;
        } else if t_pos.0 < h_pos.0 - 1 {
            t_pos.0 += 1;
        }
    } else if (t_pos.0 - h_pos.0).abs() + (t_pos.1 - h_pos.1).abs() > 2 {
        if t_pos.1 > h_pos.1 {
            t_pos.1 -= 1;
        } else if t_pos.1 < h_pos.1 {
            t_pos.1 += 1;
        }
        if t_pos.0 > h_pos.0 {
            t_pos.0 -= 1;
        } else if t_pos.0 < h_pos.0 {
            t_pos.0 += 1;
        }
    }
    t_pos
}
