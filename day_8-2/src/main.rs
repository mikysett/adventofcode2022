use std::fs;

struct Forest {
    pub threes: Vec<Vec<(u32, usize)>>,
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
                    .map(|c| (c.to_digit(10).unwrap(), 0))
                    .collect::<Vec<(u32, usize)>>()
            })
            .collect::<Vec<Vec<(u32, usize)>>>(),
            size: Point(0, 0),
        };
    forest.size = Point(forest.threes.first().unwrap().len(), forest.threes.len());

    let mut most_views = 0;
    for row in 1..forest.size.0 - 1 {
        for col in 1..forest.size.1 - 1 {
            let curr_views = calculate_views(&forest, Point(row, col));
            forest.threes[row][col].1 = curr_views;
            if curr_views > most_views {
                most_views = curr_views;
            }
        }
    }
    println!("{}", most_views);
}

fn calculate_views(forest: &Forest, pos: Point) -> usize {
    let three_hight = forest.threes[pos.0][pos.1].0;

    count_visible(forest, three_hight, &pos, Step(-1, 0))
        * count_visible(forest, three_hight, &pos, Step(0, -1))
        * count_visible(forest, three_hight, &pos, Step(1, 0))
        * count_visible(forest, three_hight, &pos, Step(0, 1))
}

fn count_visible(forest: &Forest, highest: u32, pos: &Point, step: Step) -> usize {
    // If next threes will be out of the array
    if (pos.0 == 0 && step.0 == -1)
    || (pos.0 == forest.size.0 - 1)
    || (pos.1 == 0 && step.1 == -1)
    || (pos.1 == forest.size.1 - 1) {
        return 0;
    }

    let new_pos;
    if step.0 == -1 {
        new_pos = Point(pos.0 - 1, pos.1);
    } else if step.1 == -1 {
        new_pos = Point(pos.0, pos.1 - 1);
    } else {
        new_pos = Point(pos.0 + step.0 as usize, pos.1 + step.1 as usize);
    }

    let curr_three = forest.threes[new_pos.0][new_pos.1];
    if curr_three.0 >= highest {
        return 1;
    }

    return count_visible(forest, highest, &new_pos, step) + 1;
}