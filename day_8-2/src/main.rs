use std::fs;

struct Forest {
    pub threes: Vec<Vec<u32>>,
    pub size: Point,
}

struct Point(usize, usize);

struct Step(i32, i32);

fn main() {
    let mut forest = Forest {
        threes: fs::read_to_string("input")
            .expect("Error")
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| (c.to_digit(10).unwrap()))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
        size: Point(0, 0),
    };
    forest.size = Point(forest.threes.first().unwrap().len(), forest.threes.len());

    let mut most_views = 0;
    for row in 1..forest.size.0 - 1 {
        for col in 1..forest.size.1 - 1 {
            let curr_views = calculate_views(&forest, Point(row, col));
            if curr_views > most_views {
                most_views = curr_views;
            }
        }
    }
    println!("{}", most_views);
}

fn calculate_views(forest: &Forest, pos: Point) -> usize {
    let directions = vec![Step(-1, 0), Step(0, -1), Step(1, 0), Step(0, 1)];
    let three_hight = forest.threes[pos.0][pos.1];

    directions.iter().fold(1, |nb_visible, direction| {
        nb_visible * count_visible(forest, three_hight, &pos, direction)
    })
}

fn count_visible(forest: &Forest, highest: u32, pos: &Point, step: &Step) -> usize {
    let mut nb_visible = 0;
    let mut pos = Point(pos.0, pos.1);
    while (pos.0 != 0 || step.0 != -1)
        && (pos.0 != forest.size.0 - 1)
        && (pos.1 != 0 || step.1 != -1)
        && (pos.1 != forest.size.1 - 1)
    {
        if step.0 == -1 {
            pos = Point(pos.0 - 1, pos.1);
        } else if step.1 == -1 {
            pos = Point(pos.0, pos.1 - 1);
        } else {
            pos = Point(pos.0 + step.0 as usize, pos.1 + step.1 as usize);
        }

        let curr_three = forest.threes[pos.0][pos.1];
        if curr_three >= highest {
            return nb_visible + 1;
        }

        nb_visible += 1;
    }
    nb_visible
}
