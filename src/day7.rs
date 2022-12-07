extern crate core;

use std::collections::HashMap;
use std::ops::Add;

fn main() {
    let input = include_str!("data/input7");

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut wd: String = "/".into();

    let input_lines = input.lines().collect::<Vec<_>>();
    let mut i: usize = 0;
    while i < input_lines.len() {
        let line = input_lines[i];
        let mut parts = line.split(' ');
        match (parts.next(), parts.next()) {
            (Some("$"), Some("cd")) => wd = match parts.next().unwrap() {
                "/" => "/".into(),
                ".." => wd.rsplit_once('/').unwrap().0.into(),
                dir_name => wd.add("/").add(dir_name),
            },
            (Some("$"), Some("ls")) => {
                while (i + 1) < input_lines.len() && input_lines[i + 1].chars().next() != Some('$') {
                    i += 1;
                    let (size_or_dir, _) = input_lines[i].split_once(' ').unwrap();
                    if size_or_dir == "dir" {
                        // empty
                    } else {
                        // add file to directory
                        let file_size = size_or_dir.parse::<usize>().unwrap();
                        if let Some(size) = dir_sizes.get(&wd) {
                            dir_sizes.insert(wd.clone(), size + file_size);
                        } else {
                            dir_sizes.insert(wd.clone(), file_size);
                        }

                        // update size of parents
                        let mut curr_parent_dir = wd.clone();
                        while let Some((parent_dir_path, _)) = curr_parent_dir.rsplit_once('/') {
                            if let Some(parent_dir_size) = dir_sizes.get(parent_dir_path) {
                                dir_sizes.insert(parent_dir_path.into(), parent_dir_size + file_size);
                            } else {
                                dir_sizes.insert(parent_dir_path.into(), file_size);
                            }
                            curr_parent_dir = parent_dir_path.into();
                        }
                    }
                }
            }
            invalid_input => panic!("invalid input: {:?}", invalid_input)
        }
        i += 1;
    }

    const LIMIT_SIZE: usize = 100_000;
    let res1 = dir_sizes.values().filter(|&&s| s <= LIMIT_SIZE).sum::<usize>();
    println!("[Part 1] Result is {res1}");

    const DISK_SIZE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;
    let used_space = *dir_sizes.get("/").unwrap();
    println!("[Part 2] {used_space} / {DISK_SIZE} (unused {})", DISK_SIZE - used_space);
    let mut sizes = dir_sizes.into_values().collect::<Vec<usize>>();
    sizes.sort_unstable();
    println!("[Part 2] {:?}", sizes);
    let res2 = sizes.into_iter().find(|&s| (DISK_SIZE - used_space + s) >= REQUIRED_SPACE).unwrap();
    println!("[Part 2] Result is {res2}");
}
