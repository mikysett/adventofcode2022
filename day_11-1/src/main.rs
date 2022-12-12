// Stacks, objects, parsing classes
use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    divisor: u64,
    give_if_true: usize,
    give_if_false: usize,
    count_inspections: usize,
}

#[derive(Debug)]
struct Op {
    kind: OpKind,
    nb: OpNumber,
}

#[derive(Debug)]
enum OpKind {
    Add,
    Mult,
}

#[derive(Debug)]
enum OpNumber {
    Num(u64),
    Old,
}

impl Monkey {
    fn update_worry_level(&mut self, lcm: u64) {
        for item in &mut self.items {
            match self.op.kind {
                OpKind::Add => {
                    match self.op.nb {
                        OpNumber::Num(nb) => *item += nb,
                        OpNumber::Old => *item += *item,
                    }
                },
                OpKind::Mult => {
                    match self.op.nb {
                        OpNumber::Num(nb) => *item *= nb,
                        OpNumber::Old => *item *= *item,
                    }
                }
            }
            // *item /= lcm; // Part 1
            *item %= lcm; // Part 2
        }
    }
    
    fn give_to(&mut self) -> Option<usize> {
        if self.items.is_empty() {
            None
        } else if self.items.last().unwrap() % self.divisor == 0 {
                Some(self.give_if_true)
        } else {
            Some(self.give_if_false)
        }
    }

    fn update_count_inspections(&mut self) {
        self.count_inspections += self.items.len();
    }

    fn parse_items(s: &str) -> Vec<u64> {
        let (_, numbers) = s.split_once(": ").unwrap();

        numbers.split(", ")
            .map(|nb| nb.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
        
    }

    fn parse_op(s: &str) -> Op {
        let (_, op_string) = s.split_once("= ").unwrap();

        let ops;
        let op_kind;
        if op_string.contains('+') {
            op_kind = OpKind::Add;
            ops = op_string.split_once(" + ").unwrap();
        } else {
            op_kind = OpKind::Mult;
            ops = op_string.split_once(" * ").unwrap();
        }

        let op_number = if ops.1 == "old" {
            OpNumber::Old
        } else {
            OpNumber::Num(ops.1.parse::<u64>().unwrap())
        };

        Op { kind: op_kind, nb: op_number }
    }

    fn parse_divisor(s: &str) -> u64 {
        let (_, divisor) = s.split_once(" by ").unwrap();

        divisor.parse().unwrap()
    }

    fn parse_give_if(s: &str) -> usize {
        let (_, monkey_id) = s.split_once(" monkey ").unwrap();

        monkey_id.parse().unwrap()
    }
}

fn least_common_multiple(numbers: Vec<u64>) -> u64 {
    numbers.iter().product()
}

fn main() {
    let mut monkeys = fs::read_to_string("input")
        .expect("Error")
        .split("\n\n")
        .map(|monkey| {
            let lines = monkey.lines().collect::<Vec<&str>>();
            
            Monkey {
                items: Monkey::parse_items(lines[1]),
                op: Monkey::parse_op(lines[2]),
                divisor: Monkey::parse_divisor(lines[3]),
                give_if_true: Monkey::parse_give_if(lines[4]),
                give_if_false: Monkey::parse_give_if(lines[5]),
                count_inspections: 0,
            }
        }).collect::<Vec<Monkey>>();

    // let lcm = 3; // Part 1
    let lcm = least_common_multiple(monkeys.iter().map(|m| m.divisor).collect::<Vec<u64>>()); // Part 2
    for _round in 0..10000 {
        for id in 0..monkeys.len() {
            monkeys[id].update_worry_level(lcm);
            monkeys[id].update_count_inspections();
            while !monkeys[id].items.is_empty() {
                let give_to = monkeys[id].give_to().unwrap();
                let item = monkeys[id].items.pop().unwrap();
                monkeys[give_to].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.count_inspections);
    println!("{}", 
        monkeys.iter().rev().take(2).map(|m| m.count_inspections).product::<usize>());
}
