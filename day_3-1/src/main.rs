use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("input")
            .expect("Can't read file")
            .lines()
            .map(calculate_priority)
            .sum::<u32>()
    );
}

fn calculate_priority(s: &str) -> u32 {
    let half_size = s.len() / 2;
    let first_half = &s[..half_size];
    let second_half = &s[half_size..];

    for c in first_half.chars() {
        if second_half.contains(c) {
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
