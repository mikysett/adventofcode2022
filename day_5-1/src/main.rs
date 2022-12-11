// Parsing problem, moving containers on stacks
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut stacks: Vec<Vec<char>> = vec![];
        let mut is_move = false;
        for line in lines.flatten() {
            if line.is_empty() {
                is_move = true;
                stacks = stacks
                    .into_iter()
                    .map(|stack| stack.into_iter().rev().collect())
                    .collect::<Vec<Vec<char>>>();
            } else if !is_move {
                update_stacks(&line, &mut stacks);
            } else {
                let m = parse_move(&line);
                for _ in 0..m.quantity {
                    if let Some(moving_crate) = stacks[m.from].pop() {
                        stacks[m.to].push(moving_crate);
                    }
                }
            }
        }

        println!(
            "{}",
            stacks
                .iter()
                .map(|stack| stack.iter().last().unwrap())
                .collect::<String>()
        );
    }
}

fn update_stacks(line: &str, stacks: &mut Vec<Vec<char>>) {
    for (i, c) in line.chars().enumerate() {
        if c.is_alphabetic() {
            if stacks.is_empty() {
                for _ in 0..line.len() / 4 + 1 {
                    stacks.push(vec![]);
                }
            }
            stacks[(i - 1) / 4].push(c);
        }
    }
}

fn parse_move(line: &str) -> Move {
    let words = line.split(' ').collect::<Vec<_>>();
    Move {
        quantity: words[1].parse::<usize>().unwrap(),
        from: words[3].parse::<usize>().unwrap() - 1,
        to: words[5].parse::<usize>().unwrap() - 1,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
