use std::fs;

fn main() {
    let mut helves_calories = fs::read_to_string("input")
        .expect("Can't open input file")
        .split("\n\n")
        .map(|s| s.lines().flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .collect::<Vec<u32>>();

    helves_calories.sort();

    println!(
        "3 helves with more calories: {}",
        helves_calories.iter().rev().take(3).sum::<u32>()
    );
}
