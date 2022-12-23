mod profiler;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn turn_right(self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    fn turn_left(self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile { Open = b'.', Wall = b'#', Void = b' ' }

impl From<u8> for Tile {
    fn from(c: u8) -> Self {
        match c {
            b'.' => Tile::Open,
            b'#' => Tile::Wall,
            b' ' => Tile::Void,
            _ => panic!("unexpected character '{}'", std::char::from_u32(c as u32).unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn new(map_str: &str) -> Self {
        Self {
            map: map_str
                .lines()
                .map(|l| l.as_bytes().iter().map(|c| Tile::from(*c)).collect())
                .collect::<Vec<Vec<Tile>>>()
        }
    }

    fn get_start(&self) -> (i64, i64) {
        (self.map[0].iter().position(|t| *t == Tile::Open).unwrap() as i64, 0)
    }

    fn at(&self, x: i64, y: i64) -> Tile {
        if x < 0 || y < 0 {
            return Tile::Void;
        }

        let x = x as usize;
        let y = y as usize;
        if y >= self.map.len() || x >= self.map[y].len() {
            Tile::Void
        } else {
            self.map[y][x]
        }
    }

    fn wrap_right(&self, _x: i64, y: i64) -> Option<(i64, i64)> {
        let new_x = self.map[y as usize].iter().position(|t| *t != Tile::Void)?;
        match self.map[y as usize][new_x as usize] {
            Tile::Open => Some((new_x as i64, y)),
            _ => None
        }
    }

    fn wrap_down(&self, x: i64, _y: i64) -> Option<(i64, i64)> {
        let new_y = self.map.iter().position(|row| (x as usize) < row.len() && row[x as usize] != Tile::Void)?;
        match self.map[new_y as usize][x as usize] {
            Tile::Open => Some((x, new_y as i64)),
            _ => None
        }
    }

    fn wrap_left(&self, _x: i64, y: i64) -> Option<(i64, i64)> {
        let new_x = self.map[y as usize].iter().rposition(|t| *t != Tile::Void)?;
        match self.map[y as usize][new_x as usize] {
            Tile::Open => Some((new_x as i64, y)),
            _ => None
        }
    }

    fn wrap_up(&self, x: i64, _y: i64) -> Option<(i64, i64)> {
        let new_y = self.map.iter().rposition(|row| (x as usize) < row.len() && row[x as usize] != Tile::Void)?;
        match self.map[new_y as usize][x as usize] {
            Tile::Open => Some((x, new_y as i64)),
            _ => None
        }
    }

    // cube handling hardcoded for real input, not example
    fn cube_wrap_right(&self, _x: i64, y: i64) -> Option<(i64, i64, Facing)> {
        let m = match y {
            0..=49 => (99, 149 - y, Facing::Left),
            50..=99 => (100 + y - 50, 49, Facing::Up),
            100..=149 => (149, 49 - (y - 100), Facing::Left),
            150..=199 => (50 + y - 150, 149, Facing::Up),
            _ => panic!("invalid y coord '{y}'"),
        };
        match self.map[m.1 as usize][m.0 as usize] {
            Tile::Open => Some(m),
            _ => None
        }
    }

    fn cube_wrap_left(&self, _x: i64, y: i64) -> Option<(i64, i64, Facing)> {
        let m = match y {
            0..=49 => (0, 149 - y, Facing::Right),
            50..=99 => (y - 50, 100, Facing::Down),
            100..=149 => (50, 49 - (y - 100), Facing::Right),
            150..=199 => (50 + y - 150, 0, Facing::Down),
            _ => panic!("invalid y coord '{y}'"),
        };
        match self.map[m.1 as usize][m.0 as usize] {
            Tile::Open => Some(m),
            _ => None
        }
    }

    fn cube_wrap_down(&self, x: i64, _y: i64) -> Option<(i64, i64, Facing)> {
        let m = match x {
            0..=49 => (x + 100, 0, Facing::Down),
            50..=99 => (49, 150 + x - 50, Facing::Left),
            100..=149 => (99, 50 + x - 100, Facing::Left),
            _ => panic!("invalid x coord '{x}'"),
        };
        match self.map[m.1 as usize][m.0 as usize] {
            Tile::Open => Some(m),
            _ => None
        }
    }

    fn cube_wrap_up(&self, x: i64, _y: i64) -> Option<(i64, i64, Facing)> {
        let m = match x {
            0..=49 => (50, 50 + x, Facing::Right),
            50..=99 => (0, 150 + x - 50, Facing::Right),
            100..=149 => (x - 100, 199, Facing::Up),
            _ => panic!("invalid x coord '{x}'"),
        };
        match self.map[m.1 as usize][m.0 as usize] {
            Tile::Open => Some(m),
            _ => None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    x: i64,
    y: i64,
    facing: Facing,
}

fn move_forward(s: State, map: &Map, distance: i32) -> State {
    let mut new_x = s.x;
    let mut new_y = s.y;
    let mut distance_moved = 0;
    'move_loop: while distance_moved < distance {
        let (next_x, next_y) = match s.facing {
            Facing::Right => (new_x + 1, new_y),
            Facing::Down  => (new_x, new_y + 1),
            Facing::Left  => (new_x - 1, new_y),
            Facing::Up    => (new_x, new_y - 1),
        };

        let Some((next_x, next_y)) = (match map.at(next_x, next_y) {
            Tile::Void => match s.facing {
                Facing::Right => map.wrap_right(next_x, next_y),
                Facing::Down => map.wrap_down(next_x, next_y),
                Facing::Left => map.wrap_left(next_x, next_y),
                Facing::Up => map.wrap_up(next_x, next_y),
            },
            Tile::Open => Some((next_x, next_y)),
            Tile::Wall => None,
        }) else {
            break 'move_loop;
        };

        (new_x, new_y) = (next_x, next_y);
        distance_moved += 1;
    }

    State {
        x: new_x,
        y: new_y,
        ..s
    }
}


fn move_forward_cube(s: State, map: &Map, distance: i32) -> State {
    let mut new_x = s.x;
    let mut new_y = s.y;
    let mut new_facing = s.facing;
    let mut distance_moved = 0;
    'move_loop: while distance_moved < distance {
        let (next_x, next_y) = match new_facing {
            Facing::Right => (new_x + 1, new_y),
            Facing::Down  => (new_x, new_y + 1),
            Facing::Left  => (new_x - 1, new_y),
            Facing::Up    => (new_x, new_y - 1),
        };

        let Some((next_x, next_y, next_facing)) = (match map.at(next_x, next_y) {
            Tile::Void => match s.facing {
                Facing::Right => map.cube_wrap_right(next_x, next_y),
                Facing::Down => map.cube_wrap_down(next_x, next_y),
                Facing::Left => map.cube_wrap_left(next_x, next_y),
                Facing::Up => map.cube_wrap_up(next_x, next_y),
            },
            Tile::Open => Some((next_x, next_y, new_facing)),
            Tile::Wall => None,
        }) else {
            break 'move_loop;
        };

        (new_x, new_y, new_facing) = (next_x, next_y, next_facing);
        distance_moved += 1;
    }

    State {
        x: new_x,
        y: new_y,
        facing: new_facing,
    }
}

fn turn(s: State, dir: u8) -> State {
    State {
        facing: match dir {
            b'L' => s.facing.turn_left(),
            b'R' => s.facing.turn_right(),
            _ => panic!("unexpected character '{}'", std::char::from_u32(dir as u32).unwrap()),
        },
        ..s
    }
}

fn follow_path<const CUBE: bool>(mut s: State, map: &Map, path: &str) -> State {
    let mut distance_str_buffer = Vec::new();
    let path = path.as_bytes();
    let mut i = 0;
    loop {
        distance_str_buffer.clear();
        while i < path.len() && (b'0'..=b'9').contains(&path[i]) {
            distance_str_buffer.push(path[i]);
            i += 1;
        }

        let distance = std::str::from_utf8(&distance_str_buffer).unwrap().parse().unwrap();
        s = if CUBE { move_forward_cube(s, map, distance) } else { move_forward(s, map, distance) };

        if i >= path.len() {
            break;
        }

        let turn_dir = path[i];
        i += 1;
        s = turn(s, turn_dir);
    }

    s
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input22");
    let (map_str, path_str) = INPUT.split_once("\n\n").unwrap();
    let map = Map::new(map_str);
    let path = path_str.trim();

    let (start_x, start_y) = map.get_start();
    let start_state = State {
        x: start_x,
        y: start_y,
        facing: Facing::Right,
    };
    let final_state = follow_path::<false>(start_state, &map, path);
    let final_state2 = follow_path::<true>(start_state, &map, path);

    let res1 = 1000 * (final_state.y + 1) + 4 * (final_state.x + 1) + final_state.facing as i64;
    let res2 = 1000 * (final_state2.y + 1) + 4 * (final_state2.x + 1) + final_state2.facing as i64;
    println!("[Part 1] Result is {res1:?}");
    println!("[Part 2] Result is {res2:?}");
}
