use std::collections::HashSet;

mod profiler;

#[derive(Debug, Clone)]
struct Grid {
    min: Pos,
    max: Pos,
    grid: Vec<Vec<u8>>,
    extra_floor: HashSet<Pos>, // for part 2
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos { x: usize, y: usize }
const fn pos(x: usize, y: usize) -> Pos { Pos { x, y } }

const AIR: u8 = b'.';
const ROCK: u8 = b'#';
const SAND: u8 = b'o';

const SAND_SOURCE: Pos = pos(500, 0);

fn build_grid(lines: &Vec<Vec<Pos>>) -> Grid {
    let (min_x, max_x) = lines.iter()
        .flat_map(|l| l.iter().map(|p| p.x))
        .chain([SAND_SOURCE.x])
        .fold((usize::MAX, usize::MIN), |acc, n| (acc.0.min(n), acc.1.max(n)));
    let (min_y, max_y) = lines.iter()
        .flat_map(|l| l.iter().map(|p| p.y))
        .chain([SAND_SOURCE.y])
        .fold((usize::MAX, usize::MIN), |acc, n| (acc.0.min(n), acc.1.max(n)));

    println!("Min: ({}, {})", min_x, min_y);
    println!("Max: ({}, {})", max_x, max_y);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    println!("Width×Height: {}×{}", width, height);
    let mut grid = vec![vec![AIR; width]; height];

    for line in lines {
        for w in line.windows(2) {
            let a = w[0];
            let b = w[1];
            for x in a.x.min(b.x)..=a.x.max(b.x) {
                grid[a.y - min_y][x - min_x] = ROCK;
            }
            for y in a.y.min(b.y)..=a.y.max(b.y) {
                grid[y - min_y][a.x - min_x] = ROCK;
            }
        }
    }

    Grid {
        min: pos(min_x, min_y),
        max: pos(max_x, max_y),
        grid,
        extra_floor: HashSet::new(),
    }
}

fn print_grid(g: &Grid) {
    for row in &g.grid {
        println!("{}", std::str::from_utf8(&row).unwrap());
    }
}

fn pos_in_grid(g: &Grid, p: Pos) -> bool {
    (g.min.x..=g.max.x).contains(&p.x) && (g.min.y..=g.max.y).contains(&p.y)
}

fn sand_can_move_to<const PART: usize>(g: &Grid, new_pos: Pos) -> bool {
    if pos_in_grid(g, new_pos) {
        g.grid[new_pos.y - g.min.y][new_pos.x - g.min.x] == AIR
    } else {
        if PART == 2 {
            if new_pos.y == g.max.y + 2 { // floor
                false
            } else {
                // check sand placed on the floor outside the grid
                !g.extra_floor.contains(&new_pos)
            }
        } else {
            // part 1, flows out of bounds
            true
        }
    }
}

fn sand_flow<const PART: usize>(g: &Grid, p: Pos) -> Option<Pos> {
    for new_pos in [
        pos(p.x, p.y + 1),        // down one step
        pos(p.x - 1, p.y + 1), // one step down and to the left
        pos(p.x + 1, p.y + 1), // one step down and to the right
    ] {
        if sand_can_move_to::<PART>(g, new_pos) {
            return if pos_in_grid(g, new_pos) || PART == 2 {
                Some(new_pos)
            } else {
                None
            }
        }
    }

    Some(p)
}

fn sand_generation<const PART: usize>(g: &mut Grid) -> usize {
    let mut sand_units = 0usize;
    'sand_generation: loop {
        let mut sand_pos = SAND_SOURCE;
        'sand_flow: loop {
            match sand_flow::<PART>(g, sand_pos) {
                None => break 'sand_generation, // part 1 finished when sand is out of bounds
                Some(new_sand_pos) => {
                    if new_sand_pos == sand_pos { // comes to rest
                        sand_units += 1;

                        // place sand in grid
                        if pos_in_grid(g, sand_pos) {
                            g.grid[sand_pos.y - g.min.y][sand_pos.x - g.min.x] = SAND;
                        } else if PART == 2 {
                            g.extra_floor.insert(sand_pos);
                        }

                        // part 2 finishes when the sand source is reached
                        if PART == 2 && sand_pos == SAND_SOURCE {
                            break 'sand_generation;
                        }

                        break 'sand_flow;
                    }

                    sand_pos = new_sand_pos;
                }
            }
        }
    }
    sand_units
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input14");
    let lines = input
        .lines()
        .map(|l| l.split(" -> ")
            .map(|pos_str| pos_str.split_once(',').unwrap())
            .map(|(x_str, y_str)| pos(x_str.parse().unwrap(), y_str.parse().unwrap()))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let mut grid = build_grid(&lines);
    let mut grid2 = grid.clone();
    // print_grid(&grid);

    let res1 = sand_generation::<1>(&mut grid);
    let res2 = sand_generation::<2>(&mut grid2);
    // print_grid(&grid);
    // print_grid(&grid2);
    println!("[Part 1] Result is {res1}");
    println!("[Part 2] Result is {res2}");
}
