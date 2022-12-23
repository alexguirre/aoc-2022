use std::collections::{HashMap, HashSet};
use std::ops;

mod profiler;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos { x: i64, y: i64 }
macro_rules! pos {
    ($x:expr, $y:expr) => { Pos { x: $x, y: $y }};
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        pos!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        pos!(self.x - rhs.x, self.y - rhs.y)
    }
}

type Map = HashSet<Pos>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir { NW, N, NE, E, SE, S, SW, W }

impl Dir {
    fn vector(self) -> Pos {
        match self {
            Dir::NW => pos!(-1, -1),
            Dir::N  => pos!( 0, -1),
            Dir::NE => pos!(1, -1),
            Dir::E  => pos!(1,  0),
            Dir::SE => pos!(1, 1),
            Dir::S  => pos!( 0, 1),
            Dir::SW => pos!(-1, 1),
            Dir::W  => pos!(-1,  0),
        }
    }
}

fn round(map: &Map, round_index: usize) -> Map {
    const ALL_DIRS: [Dir; 8] = [Dir::NW, Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W];
    const DIRS_TO_CHECK: [[Dir; 3]; 4] = [
        [Dir::N, Dir::NE, Dir::NW],
        [Dir::S, Dir::SE, Dir::SW],
        [Dir::W, Dir::NW, Dir::SW],
        [Dir::E, Dir::NE, Dir::SE],
    ];
    let first_dir_to_check = round_index % DIRS_TO_CHECK.len();

    // first half
    let mut new_pos_to_orig_pos = HashMap::<Pos, Vec<Pos>>::new();
    for elf in map.iter() {
        let mut move_dir = None;
        if ALL_DIRS.iter().any(|d| map.contains(&(*elf + d.vector()))) {
            // if any elf adjacent consider a direction to move to
            for k in 0..DIRS_TO_CHECK.len() {
                let i = (k + first_dir_to_check) % DIRS_TO_CHECK.len();
                let dir = DIRS_TO_CHECK[i];

                if dir.iter().all(|d| !map.contains(&(*elf + d.vector()))) {
                    move_dir = Some(dir[0]);
                    break;
                }
            }
        }

        if let Some(move_dir) = move_dir {
            let new_pos = *elf + move_dir.vector();
            if let Some(orig_positions) = new_pos_to_orig_pos.get_mut(&new_pos) {
                orig_positions.push(*elf);
            } else {
                new_pos_to_orig_pos.insert(new_pos, vec![*elf]);
            }
        } else {
            // the elf doesn't move
            new_pos_to_orig_pos.insert(*elf, vec![*elf]);
        }
    }

    // second half
    let mut res = Map::with_capacity(map.len());
    for (new_pos, elves) in new_pos_to_orig_pos.iter() {
        if elves.len() == 1 {
            res.insert(*new_pos);
        } else {
            // more than one elf want to move to `new_pos`, none of these elves move
            res.extend(elves.iter());
        }
    }
    res
}

fn get_dimensions(map: &Map) -> (i64, i64) {
    let (min_x, max_x, min_y, max_y) = map.iter()
        .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |acc, p|
            (acc.0.min(p.x), acc.1.max(p.x), acc.2.min(p.y), acc.3.max(p.y)));

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    (width, height)
}

fn count_empty_ground_tiles(map: &Map) -> i64 {
    let (w, h) = get_dimensions(map);
    println!("{w} x {h}");
    (w * h) - map.len() as i64
}

fn print_map(map: &Map) {
    let (min_x, min_y) = map.iter()
        .fold((i64::MAX, i64::MAX), |acc, p|
            (acc.0.min(p.x), acc.1.min(p.y)));
    let (w, h) = get_dimensions(map);
    let mut map_str = vec![vec![b'.'; w as usize]; h as usize];
    for elf in map {
        map_str[(elf.y - min_y) as usize][(elf.x - min_x) as usize] = b'#';
    }

    for map_row_str in map_str {
        println!("{}", std::str::from_utf8(&map_row_str).unwrap());
    }
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input23");
    let initial_map = INPUT.lines().enumerate().fold(Map::new(), |mut map, (y, row)| {
        row.as_bytes().iter().enumerate().for_each(|(x, &c)| if c == b'#' { map.insert(pos!(x as i64, y as i64)); });
        map
    });

    {
        let mut final_map = initial_map.clone();
        for i in 0..10 {
            final_map = round(&final_map, i);
        }

        let res1 = count_empty_ground_tiles(&final_map);
        println!("[Part 1] Result is {res1:?}");
    }

    {
        let mut map = initial_map.clone();
        let mut i = 0;
        loop {
            let new_map = round(&map, i);
            if map == new_map {
                break; // no elves moved
            }

            map = new_map;
            i += 1;
        }

        let res2 = i + 1;
        println!("[Part 2] Result is {res2:?}");
    }
}
