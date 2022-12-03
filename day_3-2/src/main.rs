use std::fs;

fn main() {
    println!(
        "{:?}",
        fs::read_to_string("input")
            .expect("Can't read file")
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
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
