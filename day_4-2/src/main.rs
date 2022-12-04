use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("input")
            .expect("Error")
            .lines()
            .flat_map(|line| line.split(',').collect::<Vec<_>>())
            .flat_map(|range| range.split('-').collect::<Vec<_>>())
            .map(|sector| sector.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .chunks(4)
            .fold(0, |acc, range| {
                if (range[0] >= range[2] && range[0] <= range[3])
                    || (range[1] >= range[2] && range[1] <= range[3])
                    || (range[2] >= range[0] && range[2] <= range[1])
                    || (range[3] >= range[0] && range[3] <= range[1])
                {
                    acc + 1
                } else {
                    acc
                }
            })
    );
}
