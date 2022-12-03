use std::fs;

#[derive(PartialEq, Clone)]
enum Play {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

impl Play {
    fn to_lose(&self)-> Play {
        match self {
            Play::Rock(_) => Play::Scissors(3),
            Play::Paper(_) => Play::Rock(1),
            Play::Scissors(_) => Play::Paper(2),
        }
    }

    fn to_win(&self) -> Play {
        match self {
            Play::Scissors(_) => Play::Rock(1),
            Play::Rock(_) => Play::Paper(2),
            Play::Paper(_) => Play::Scissors(3),
        }
    }
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
        .collect::<Vec<&str>>();

    let other_play = string_to_play(plays[0]);
    let my_play = get_my_play(&other_play, plays[1]);
    final_score(&other_play, &my_play)
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

fn get_my_play(other: &Play, expected_score: &str) -> Play {
    if expected_score == "X" {
        other.to_lose()
    } else if expected_score == "Y" {
        other.clone()
    } else {
        other.to_win()
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
