use std::collections::HashSet;
use std::iter::repeat;

mod profiler;

type Pos = (i32, i32);
struct Bridge<const NUM_KNOTS: usize> {
    knots: [Pos; NUM_KNOTS],
    tail_visited: HashSet<Pos>,
}

impl<const NUM_KNOTS: usize> Bridge<NUM_KNOTS> {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Self { knots: [(0, 0); NUM_KNOTS], tail_visited: visited }
    }

    fn left(&mut self) {
        self.knots[0] = (self.knots[0].0 - 1, self.knots[0].1);
        self.tail_follow(1);
    }

    fn right(&mut self) {
        self.knots[0] = (self.knots[0].0 + 1, self.knots[0].1);
        self.tail_follow(1);
    }

    fn up(&mut self) {
        self.knots[0] = (self.knots[0].0, self.knots[0].1 - 1);
        self.tail_follow(1);
    }

    fn down(&mut self) {
        self.knots[0] = (self.knots[0].0, self.knots[0].1 + 1);
        self.tail_follow(1);
    }

    fn tail_follow(&mut self, tail_knot_index: usize) {
        if self.tail_is_touching(tail_knot_index) { return; }

        let head = self.knots[tail_knot_index - 1];
        let tail = self.knots[tail_knot_index];

        let dist_horizontal = head.0 - tail.0;
        let dist_vertical = head.1 - tail.1;
        self.knots[tail_knot_index] = (tail.0 + dist_horizontal.signum(), tail.1 + dist_vertical.signum());
        if tail_knot_index == NUM_KNOTS - 1 {
            self.tail_visited.insert(self.knots[tail_knot_index]);
        } else {
            self.tail_follow(tail_knot_index + 1);
        }
    }

    fn tail_is_touching(&self, tail_knot_index: usize) -> bool {
        let head = self.knots[tail_knot_index - 1];
        let tail = self.knots[tail_knot_index];
        (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
    }
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input9");

    let mut bridge1: Bridge<2> = Bridge::new();
    let mut bridge2: Bridge<10> = Bridge::new();
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(dir, count)| (dir.chars().next().unwrap(), count.parse::<usize>().unwrap()))
        .flat_map(|(dir, count)| repeat(dir).take(count))
        .for_each(|dir| match dir {
            'L' => { bridge1.left(); bridge2.left(); },
            'R' => { bridge1.right(); bridge2.right(); },
            'U' => { bridge1.up(); bridge2.up(); },
            'D' => { bridge1.down(); bridge2.down(); },
            _ => panic!("unknown move"),
        });
    println!("[Part 1] Result is {}", bridge1.tail_visited.len());
    println!("[Part 2] Result is {}", bridge2.tail_visited.len());
}
