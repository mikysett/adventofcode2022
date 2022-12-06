use std::fs;
use std::process::exit;

fn main() {
    let mut marker: Vec<char> = vec![];
    println!(
        "{:?}",
        fs::read_to_string("input")
            .expect("Error")
            .lines()
            .map(|line| {
                line.chars()
            })
            .flat_map(|c| c)
            .fold(1, |acc, c| {
                while marker.contains(&c) {
                    marker.drain(0..1);
                }
                marker.push(c);
                // 4 for part 1 and 14 for part 2
                if marker.len() == 14 {
                    println!("{:?} - {}", marker, acc);
                    exit(0)
                }
                acc + 1
            })
    );
}
