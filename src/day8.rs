mod profiler;

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input8");
    let grid = input
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<_>>();
    let res1 =
        grid.iter().enumerate().map(|(y, &row)| {
            row.iter().enumerate().filter(|&(x, &tree)| {
                let shorter_than_tree = |other: u8| other < tree;
                row[..x].iter().map(|c| *c).all(&shorter_than_tree) ||      // left
                row[x+1..].iter().map(|c| *c).all(&shorter_than_tree) ||    // right
                grid[..y].iter().map(|r| r[x]).all(&shorter_than_tree) || // up
                grid[y+1..].iter().map(|r| r[x]).all(&shorter_than_tree)  // down
            }).count()
        }).sum::<usize>();
    println!("[Part 1] Result is {res1}");

    let res2 =
        grid.iter().enumerate().map(|(y, &row)| {
            row.iter().enumerate().map(|(x, &tree)| {
                fn distance_until_blocked<I>(it: I, tree: u8) -> Option<usize> where I: Iterator<Item = u8> {
                    it.enumerate()
                      .find_map(|(i, other)| if other >= tree { Some(i + 1) } else { None })
                }

                let dist_to_edge_left = x;
                let dist_to_edge_right = row.len() - x - 1;
                let dist_to_edge_up = y;
                let dist_to_edge_down = grid.len() - y - 1;

                let left = distance_until_blocked(row[..x].iter().rev().map(|c| *c), tree).unwrap_or(dist_to_edge_left);
                let right = distance_until_blocked(row[x+1..].iter().map(|c| *c), tree).unwrap_or(dist_to_edge_right);
                let up = distance_until_blocked(grid[..y].iter().rev().map(|r| r[x]), tree).unwrap_or(dist_to_edge_up);
                let down = distance_until_blocked(grid[y+1..].iter().map(|r| r[x]), tree).unwrap_or(dist_to_edge_down);

                left * right * up * down
            }).max().unwrap()
        }).max().unwrap();
    println!("[Part 2] Result is {res2}");
}
