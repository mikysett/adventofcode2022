// Tic tac toes
use std::fs;

#[derive(PartialEq)]
enum Play {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

fn main() {
    let total = fs::read_to_string("input")
        .expect("Can't open file")
        .lines()
        .map(get_score)
        .sum::<u32>();

    println!("{}", total);
}

fn get_score(s: &str) -> u32 {
    let plays = s
        .split(' ')
        .map(string_to_play)
        .collect::<Vec<Play>>();

    final_score(&plays[0], &plays[1])
}

fn string_to_play(s: &str) -> Play {
    if s == "A" || s == "X" {
        Play::Rock(1)
    } else if s == "B" || s == "Y" {
        Play::Paper(2)
    } else {
        Play::Scissors(3)
    }
}

fn final_score(other: &Play, me: &Play) -> u32 {
    let points = match me {
        Play::Rock(points) | Play::Paper(points) | Play::Scissors(points) => *points
    };

    if other == me {
        points + 3
    } else if matches!(other, Play::Rock(_)) && matches!(me, Play::Paper(_))
        || matches!(other, Play::Paper(_)) && matches!(me, Play::Scissors(_))
        || matches!(other, Play::Scissors(_)) && matches!(me, Play::Rock(_)) {
        points + 6
    } else {
        points
    }
}
