use std::collections::{HashSet, VecDeque};
use std::ops;

mod profiler;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}
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

impl ops::Mul<i64> for Pos {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        pos!(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    pos: Pos,
    dir: Pos,
}

impl Blizzard {
    fn get_position_at(&self, map: &Map, time: i64) -> Pos {
        let p = self.pos + self.dir * time;
        pos!(p.x.rem_euclid(map.width), p.y.rem_euclid(map.height))
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: i64,
    height: i64,
    blizzards: Vec<Blizzard>,
}

impl Map {
    fn has_blizzard_at(&self, pos: Pos, time: i64) -> bool {
        self.blizzards.iter().any(|b| b.get_position_at(self, time) == pos)
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut buff = vec![vec![b'.'; self.width as usize + 2]; self.height as usize + 2];

        for y in 0..buff.len() {
            let row = &mut buff[y];
            let last = row.len() - 1;
            row[0] = b'#';
            row[last] = b'#';
        }
        for x in 0..buff[0].len() {
            let last = buff.len() - 1;
            if x != 1 { buff[0][x] = b'#'; }
            if x != self.width as usize { buff[last][x] = b'#'; }
        }

        for b in &self.blizzards {
            let new_char = match buff[(b.pos.y + 1) as usize][(b.pos.x + 1) as usize] {
                b'2'..=b'8' => buff[(b.pos.y + 1) as usize][(b.pos.x + 1) as usize] + 1,
                b'>' | b'<' | b'v' | b'^' => b'2',
                b'.' => match b.dir {
                    pos!(1, 0) => b'>',
                    pos!(-1, 0) => b'<',
                    pos!(0, 1) => b'v',
                    pos!(0, -1) => b'^',
                    _ => panic!(),
                }
                c => panic!("unexpected character '{}' at ({}, {})", std::char::from_u32(c as u32).unwrap(), b.pos.x, b.pos.y),
            };

            buff[(b.pos.y + 1) as usize][(b.pos.x + 1) as usize] = new_char;
        }

        for row in buff {
            println!("{}", std::str::from_utf8(&row).unwrap());
        }
    }
}

fn pathfind_bfs(map: &Map, start: Pos, end: Pos, start_time: i64) -> Option<i64> {
    type Step = (Pos, i64);

    let width = map.width;
    let height = map.height;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::<Step>::new();
    queue.push_back((start, start_time));
    while let Some((curr, time)) = queue.pop_front() {
        if curr == end {
            return Some(time);
        }

        let next_time = time + 1;
        let is_at_start = curr == start;
        for pos_opt in [
            if !is_at_start && curr.x > 0          { Some(pos!(curr.x - 1, curr.y)) } else { None }, // left
            if !is_at_start && curr.y > 0          { Some(pos!(curr.x, curr.y - 1)) } else { None }, // up
            if !is_at_start && curr.x < width - 1  { Some(pos!(curr.x + 1, curr.y)) } else { None }, // right
            if !is_at_start && curr.y < height - 1 { Some(pos!(curr.x, curr.y + 1)) } else { None }, // down
            Some(curr), // wait
            // special cases for entrance and exit, since those are outside the map bounds
            if curr == pos!(width - 1, height - 1) { Some(pos!(curr.x, curr.y + 1)) } else { None }, // down at bottom-right corner
            if curr == pos!(0, 0)                  { Some(pos!(curr.x, curr.y - 1)) } else { None }, // up at top-left corner
            if curr == pos!(0, -1)                 { Some(pos!(0, 0)) } else { None }, // down at entrance
            if curr == pos!(width - 1, height)     { Some(pos!(width - 1, height - 1)) } else { None }, // up at exit
        ] {
            let Some(pos) = pos_opt else { continue; };
            if !visited.contains(&(pos, next_time)) && !map.has_blizzard_at(pos, next_time) {
                visited.insert((pos, next_time));
                queue.push_back((pos, next_time));
            }
        }
    }

    None
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input24");
    let initial_map = INPUT
        .lines().enumerate()
        .fold(Map { width: 0, height: -2, blizzards: Vec::new() },
        |mut map, (y, row)| {
            row.as_bytes().iter().enumerate().for_each(|(x, &c)| {
                let pos = pos!(x as i64 - 1, y as i64 - 1);
                if let Some(blizzard) = match c {
                    b'>' => Some(Blizzard { pos, dir: pos!(1, 0) }),
                    b'<' => Some(Blizzard { pos, dir: pos!(-1, 0) }),
                    b'^' => Some(Blizzard { pos, dir: pos!(0, -1) }),
                    b'v' => Some(Blizzard { pos, dir: pos!(0, 1) }),
                    _ => None
                } {
                    map.blizzards.push(blizzard);
                }
            });
            if map.width == 0 { map.width = (row.as_bytes().len() - 2) as i64; }
            map.height += 1;
            map
        });

    let start = pos!(0, -1);
    let end = pos!(initial_map.width - 1, initial_map.height);

    let res1 = pathfind_bfs(&initial_map, start, end, 0).unwrap();
    println!("[Part 1] Result is {res1:?}");

    let second_trip = pathfind_bfs(&initial_map, end, start, res1).unwrap();
    let res2 = pathfind_bfs(&initial_map, start, end, second_trip).unwrap();
    println!("[Part 2] Result is {res2:?}");

    // let mut map = initial_map.clone();
    // for i in 0..res1.len() {
    //     println!("==== Minute {} ====", map.time);
    //     map.print();
    //     map = map.next();
    // }
}
