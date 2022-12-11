// CPU instruction set, assembly, 2D CRT screen drawing
use std::str::FromStr;
use std::fs;

#[derive(Debug)]
enum Operation {
    Addx(i32),
    Noop,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Operation::Noop)
        } else {
            let (_op_name, op_val) = s.split_once(' ').unwrap();
            Ok(Operation::Addx(op_val.parse::<i32>().unwrap()))
        }
    }
}

fn main() {
    let mut x_register = 1;
    let reg_val: Vec<i32> = fs::read_to_string("input")
        .expect("Error")
        .lines()
        .flat_map(|line| {
            let op = Operation::from_str(line).unwrap();
            let mut reg_val: Vec<i32> = vec![x_register];

            if let Operation::Addx(val) = op {
                reg_val.push(x_register);
                x_register += val;
            }
            reg_val
        })
        .collect::<Vec<i32>>();
    
    println!("Part 1: {:#?}",
        reg_val.iter()
            .enumerate()
            .filter(|(i, _)| *i == 19 || ((i + 21) % 40 == 0 && *i <= 219))
            .map(|(i, reg)| {
                (i as i32 + 1) * reg
            })
            .sum::<i32>());

    println!("Part 2:\n{}",
    reg_val.iter()
        .enumerate()
        .map(|(i, reg)| {
            let screen_pos = i as i32;
            let mut pixel: String = "".to_string();
            if screen_pos == 40 {
                println!("\n{} - {}", reg, screen_pos);
            }
            if *reg >= screen_pos % 40 - 1 && *reg <= screen_pos % 40 + 1 {
                pixel.push('#');
            } else {
                pixel.push('.');
            }
            if (screen_pos + 1) % 40 == 0 {
                pixel.push('\n');
            }
            pixel
        })
        .collect::<String>());
}
