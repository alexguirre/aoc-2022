use std::collections::VecDeque;

mod profiler;

type Grid = Vec<Vec<u8>>;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pos { x: usize, y: usize }
fn pos(x: usize, y: usize) -> Pos { Pos { x, y } }

fn pathfind_bfs(grid: &Grid, start: Pos, end: Pos) -> Option<usize> {
    type Step = (Pos, usize);

    let height = grid.len();
    let width = grid[0].len();
    let mut visited_grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut queue = VecDeque::<Step>::new();
    visited_grid[start.y][start.x] = true;
    queue.push_back((start, 0));
    while let Some((curr, number_of_steps)) = queue.pop_front() {
        if curr == end {
            return Some(number_of_steps);
        }
        for np in [
            if curr.x > 0 { Some(pos(curr.x - 1, curr.y)) } else { None }, // left
            if curr.y > 0 { Some(pos(curr.x, curr.y - 1)) } else { None }, // up
            if curr.x < width - 1 { Some(pos(curr.x + 1, curr.y)) } else { None }, // right
            if curr.y < height - 1 { Some(pos(curr.x, curr.y + 1)) } else { None }, // down
        ]  {
            let (nx, ny) = if let Some(Pos{x: nx, y: ny}) = np { (nx, ny) } else { continue; };
            if !visited_grid[ny][nx] && grid[ny][nx] <= (grid[curr.y][curr.x] + 1) {
                visited_grid[ny][nx] = true;
                // println!("({}, {}) -> ({}, {})   (steps: {})", curr.x, curr.y, nx, ny, number_of_steps + 1);
                queue.push_back((pos(nx, ny), number_of_steps + 1));
            }
        };
    }

    return None;
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input12");
    let mut start = pos(0, 0);
    let mut end = pos(0, 0);
    let grid: Grid = input
        .lines().enumerate()
        .map(|(y, l)| l.bytes().enumerate().map(|(x, n)| match n {
            b'S' => { start = pos(x, y); b'a' },
            b'E' => { end = pos(x, y); b'z' },
            _ => n,
        }).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let res1 = pathfind_bfs(&grid, start, end);
    println!("[Part 1] Result is {res1:?}");

    let res2 = grid.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate()
            .filter_map(move |(x, n)| if *n == b'a' { Some(pos(x, y)) } else { None })
        ).map(|new_start| pathfind_bfs(&grid, new_start, end))
        .filter_map(|num_steps| num_steps)
        .min();
    println!("[Part 2] Result is {res2:?}");
}
