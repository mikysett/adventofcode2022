// execute commands, cd and ls, filesystem
use std::fs;

#[derive(Debug)]
enum Three {
    Dir(Directory),
    File(File),
}

#[derive(Debug)]
struct Directory {
    name: String,
    elements: Vec<Three>,
    size: usize,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            name,
            elements: vec![],
            size: 0,
        }
    }

    fn add_dir(&mut self, name: String) {
        self.elements.push(Three::Dir(Directory::new(name)))
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.elements.push(Three::File(File { name, size }));
    }

    fn dir_exist(&self, name: &str) -> bool {
        self.elements.iter().any(|el| {
            if let Three::Dir(dir) = el {
                dir.name == name
            } else {
                false
            }
        })
    }

    fn file_exist(&self, name: &str) -> bool {
        self.elements.iter().any(|el| {
            if let Three::File(file) = el {
                file.name == name
            } else {
                false
            }
        })
    }

    fn get_dir(&mut self, name: &str) -> Option<&mut Directory> {
        for el in &mut self.elements {
            if let Three::Dir(dir) = el {
                if dir.name == name {
                    return Some(dir);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

fn main() {
    let mut file_system = Directory::new("/".to_string());
    let lines = fs::read_to_string("input")
        .expect("Error")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    create_three(&mut file_system, &lines, 0);

    calculate_dir_size(&mut file_system);
    println!("{}", tot_size_dir_below_100000(&mut file_system));

    let space_taken = file_system.size;
    let space_to_free = 30000000 - (70000000 - space_taken);
    println!(
        "{}",
        smallest_dir_size_to_free_space(&mut file_system, space_to_free, space_taken)
    );
}

fn create_three(curr_dir: &mut Directory, lines: &Vec<String>, line_pos: usize) -> usize {
    let mut line_pos = line_pos;

    while line_pos < lines.len() {
        let line_args = lines[line_pos].split(' ').collect::<Vec<_>>();
        if line_args[0] == "$" {
            if line_args[1] == "cd" {
                match line_args[2] {
                    ".." => return line_pos,
                    "/" => (),
                    _ => {
                        if curr_dir.dir_exist(line_args[2]) {
                            line_pos = create_three(
                                curr_dir.get_dir(line_args[2]).unwrap(),
                                lines,
                                line_pos + 1,
                            );
                        } else {
                            println!("FAILED TO CHANGE DIR TO: {}", line_args[2]);
                        }
                    }
                }
            }
        } else if line_args[0] == "dir" {
            if !curr_dir.dir_exist(line_args[1]) {
                curr_dir.add_dir(line_args[1].to_string());
            }
        } else if !curr_dir.file_exist(line_args[1]) {
            curr_dir.add_file(
                line_args[1].to_string(),
                line_args[0].parse::<usize>().unwrap(),
            );
        }
        line_pos += 1;
    }
    line_pos
}

fn calculate_dir_size(curr_dir: &mut Directory) -> usize {
    for el in &mut curr_dir.elements {
        match el {
            Three::Dir(dir) => curr_dir.size += calculate_dir_size(dir),
            Three::File(file) => curr_dir.size += file.size,
        }
    }
    curr_dir.size
}

fn tot_size_dir_below_100000(curr_dir: &mut Directory) -> usize {
    let mut tot_size;

    if curr_dir.size < 100000 {
        tot_size = curr_dir.size;
    } else {
        tot_size = 0;
    }

    for el in &mut curr_dir.elements {
        if let Three::Dir(dir) = el {
            tot_size += tot_size_dir_below_100000(dir);
        }
    }
    tot_size
}

fn smallest_dir_size_to_free_space(
    curr_dir: &mut Directory,
    space_to_free: usize,
    smallest: usize,
) -> usize {
    let mut curr_smallest;

    if curr_dir.size >= space_to_free && curr_dir.size < smallest {
        curr_smallest = curr_dir.size;
    } else {
        curr_smallest = smallest;
    }
    for el in &mut curr_dir.elements {
        if let Three::Dir(dir) = el {
            let res = smallest_dir_size_to_free_space(dir, space_to_free, curr_smallest);
            if res >= space_to_free && res < curr_smallest {
                curr_smallest = res;
            }
        }
    }
    curr_smallest
}
