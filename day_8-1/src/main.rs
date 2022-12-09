use colored::Colorize;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Threes(pub Vec<Vec<(u32, bool)>>);

impl fmt::Display for Threes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, line| {
            result.and_then(|_| {
                line.iter()
                    .fold(Ok(()), |result, (hight, visible)| {
                        result.and_then(|_| {
                            if *visible {
                                write!(f, "{}", format!("{}", hight).green())
                            } else {
                                write!(f, "{}", hight)
                            }
                        })
                    })
                    .and_then(|_| writeln!(f))
            })
        })
    }
}

fn main() {
    let mut nb_visible_threes = 0;
    let mut threes = Threes(
        fs::read_to_string("input")
            .expect("Error")
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| (c.to_digit(10).unwrap(), false))
                    .collect::<Vec<(u32, bool)>>()
            })
            .collect::<Vec<Vec<(u32, bool)>>>(),
    );

    nb_visible_threes += threes
        .0
        .iter_mut()
        .map(|line| count_on_axis(&mut line.iter_mut()) + count_on_axis(&mut line.iter_mut().rev()))
        .sum::<usize>();

    for col in 0..threes.0.first().unwrap().len() {
        let mut three_col = threes
            .0
            .iter_mut()
            .map(|line| line.get_mut(col).unwrap());
        nb_visible_threes += count_on_axis(&mut three_col);
        let three_col = threes
            .0
            .iter_mut()
            .map(|line| line.get_mut(col).unwrap());
            nb_visible_threes += count_on_axis(&mut three_col.rev());
    }
    println!("{}", threes);
    println!("{}", nb_visible_threes);
}

fn count_on_axis<'a, I>(threes_line: &mut I) -> usize
where
    I: Iterator<Item = &'a mut (u32, bool)> + 'a,
{
    let mut bigger_three = 10;

    threes_line.fold(0, |nb_visible, (hight, visible)| {
        if *hight > bigger_three || bigger_three == 10 {
            bigger_three = *hight;
            if !*visible {
                *visible = true;
                return nb_visible + 1;
            }
        }
        nb_visible
    })
}
