// Parsing, lists, cons
use std::fs;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
enum Node {
    Num(i32),
    List(Vec<Node>),
}

#[derive(Debug)]
enum Ordered {
    Yes,
    No,
    Maybe,
}

fn main() {
    let mut lists = fs::read_to_string("input")
        .expect("can't open the file")
        .split("\n\n")
        .flat_map(|pair| pair.lines())
        .map(|line| parse_list(line, 0).1)
        .collect::<Vec<Vec<Node>>>();
    
    part1(&lists);
    part2(&mut lists);
}

fn parse_list(s: &str, mut i: usize) -> (usize, Vec<Node>) {
    let mut list: Vec<Node> = vec![];
    while i < s.len() {
        if s.chars().nth(i).unwrap() == '[' {
            let (new_i, sub_list) = parse_list(s, i + 1);
            list.push(Node::List(sub_list));
            i = new_i;
        } else if s.chars().nth(i).unwrap() == ']' {
            return (i + 1, list);
        } else {
            let first_sublist = s[i..s.len()].chars().position(|c| c == '[');
            let list_end = s[i..s.len()].chars().position(|c| c == ']').unwrap();

            let nb_end = i + match first_sublist {
                Some(sub_i) => if sub_i < list_end { sub_i } else { list_end },
                None => list_end,
            };

            s[i..nb_end]
                .split(',')
                .for_each(|nb| {
                    if !nb.is_empty() {
                        list.push(Node::Num(nb.parse::<i32>().unwrap()));
                    }
                });
            i = nb_end;
        }
    }

    (i, list)
}

fn part1(lists: &Vec<Vec<Node>>) {
    let is_ordered = lists.chunks(2)
        .map(|list_pair| {
            is_right_order(&list_pair[0], &list_pair[1])
        }).collect::<Vec<bool>>();

    println!("Ordered index sum: {}", 
        is_ordered.iter()
            .enumerate()
            .fold(0, |i_sum, (i, is_ordered)| if *is_ordered { i_sum + i + 1 } else { i_sum }));
}

fn part2(lists: &mut Vec<Vec<Node>>) {
    lists.push(vec![Node::List(vec![Node::List(vec![Node::Num(2)])])]);
    lists.push(vec![Node::List(vec![Node::List(vec![Node::Num(6)])])]);
    
    lists.sort_by(|a, b| match is_right_order(a, b) { false => Ordering::Greater, true => Ordering::Less });

    let p1_pos = lists.iter().position(|list| list == &vec![Node::List(vec![Node::List(vec![Node::Num(2)])])]).unwrap() + 1;
    let p2_pos = lists.iter().position(|list| list == &vec![Node::List(vec![Node::List(vec![Node::Num(6)])])]).unwrap() + 1;
    println!("Decoder key: {}", p1_pos * p2_pos);
}

fn is_right_order(l: &Vec<Node>, r: &Vec<Node>) -> bool {
    !matches!(compare_lists(l, r), Ordered::No)
}

fn compare_lists(l: &Vec<Node>, r: &Vec<Node>) -> Ordered {
    for i in 0..l.len() {
        if i == r.len() {
            return Ordered::No;
        }
        if matches!(l[i], Node::List(_)) || matches!(r[i], Node::List(_)) {
            let l_list_tmp;
            let r_list_tmp;
            
            let new_l = match &l[i] { Node::List(l_list) => l_list, Node::Num(l_nb) => {l_list_tmp = vec![Node::Num(*l_nb)]; &l_list_tmp} };
            let new_r = match &r[i] { Node::List(r_list) => r_list, Node::Num(r_nb) => {r_list_tmp = vec![Node::Num(*r_nb)]; &r_list_tmp} };
            let is_ordered = compare_lists(new_l, new_r);
            match is_ordered {
                Ordered::Yes => return Ordered::Yes,
                Ordered::No => return Ordered::No,
                _ => (),
            }
        }
        else if l[i] < r[i] {
            return Ordered::Yes;
        } else if l[i] > r[i] {
            return Ordered::No;
        }
    }
    if l.len() < r.len() {
        Ordered::Yes
    } else {
        Ordered::Maybe
    }
}
