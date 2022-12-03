use std::fs;

fn main() {
    let bags = fs::read_to_string("input")
        .expect("Can't read file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut bags_per_group: Vec<Vec<&str>> = vec![];
    for i in (0..bags.len() - 2).step_by(3) {
        bags_per_group.push(vec![&bags[i], &bags[i + 1], &bags[i + 2]]);
    }

    println!(
        "{:?}",
        bags_per_group
            .iter()
            .map(|group| calculate_badge(group))
            .sum::<u32>()
    );
}

fn calculate_badge(group: &[&str]) -> u32 {
    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            return char_to_priority(c);
        }
    }

    0
}

fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}
