use std::collections::HashMap;

mod profiler;

#[derive(Debug, Copy, Clone)]
struct Rock {
    shape: [u8; 4],
}

impl Rock {
    fn push_left(self) -> Rock {
        let mut res = self;
        for row in &mut res.shape {
            *row <<= 1;
        }
        res
    }

    fn push_right(self) -> Rock {
        let mut res = self;
        for row in &mut res.shape {
            *row >>= 1;
        }
        res
    }
}

const ROCKS: [Rock; 5] = [
    Rock {
        shape: [
            0b0000000,
            0b0000000,
            0b0000000,
            0b0011110,
        ]
    },
    Rock {
        shape: [
            0b0000000,
            0b0001000,
            0b0011100,
            0b0001000,
        ]
    },
    Rock {
        shape: [
            0b0000000,
            0b0000100,
            0b0000100,
            0b0011100,
        ]
    },
    Rock {
        shape: [
            0b0010000,
            0b0010000,
            0b0010000,
            0b0010000,
        ]
    },
    Rock {
        shape: [
            0b0000000,
            0b0000000,
            0b0011000,
            0b0011000,
        ]
    },
];

fn rock_will_collide_left_wall(r: Rock) -> bool {
    r.shape.iter().map(|n| n.leading_zeros()).min().unwrap() <= 1
}

fn rock_will_collide_right_wall(r: Rock) -> bool {
    r.shape.iter().map(|n| n.trailing_zeros()).min().unwrap() == 0
}

fn rock_overlaps_rocks(r: Rock, stack: &Vec<u8>, y: usize) -> bool {
    for h in 0..r.shape.len() {
        let rock_row = r.shape[r.shape.len() - 1 - h];
        if rock_row == 0 {
            break;
        }

        let Some(&stack_row) = stack.get(y + h) else {
            break;
        };

        if (stack_row & rock_row) != 0 {
            return true;
        }
    }

    return false;
}

fn rock_can_move_left(r: Rock, stack: &Vec<u8>, y: usize) -> bool {
    !rock_will_collide_left_wall(r) && !rock_overlaps_rocks(r.push_left(), stack, y)
}

fn rock_can_move_right(r: Rock, stack: &Vec<u8>, y: usize) -> bool {
    !rock_will_collide_right_wall(r) && !rock_overlaps_rocks(r.push_right(), stack, y)
}

fn rock_can_move_down(r: Rock, stack: &Vec<u8>, y: usize) -> bool {
    if y == 0 {
        return false;
    }

    !rock_overlaps_rocks(r, stack, y - 1)
}

fn rock_place(r: Rock, stack: &mut Vec<u8>, y: usize) {
    for h in 0..r.shape.len() {
        let rock_row = r.shape[r.shape.len() - 1 - h];
        if rock_row == 0 {
            break;
        }

        if (y + h) >= stack.len() {
            stack.push(rock_row);
        } else {
            stack[y + h] |= rock_row;
        }
    }
}

const INPUT: &str = include_str!("data/input17");

fn generate_rocks(num_rocks_to_generate: usize) -> Vec<u8> {
    let pattern = INPUT.trim().as_bytes();
    let mut stack = Vec::new();
    let mut rock_idx = 0usize;
    let mut pattern_idx = 0usize;
    let mut num_rocks = 0usize;
    'rock_generator: loop {
        num_rocks += 1;

        let mut rock = ROCKS[rock_idx];
        rock_idx = (rock_idx + 1) % ROCKS.len();

        'rock_falling: for y in (0..=(stack.len() + 3)).rev() {
            // println!(" -- y:{y} -- ");
            // println!("====================");
            // let mut stack_c = stack.clone();
            // for _ in 0..5 { stack_c.push(0); }
            // rock_place(rock, &mut stack_c, y);
            // for row in stack_c.iter().rev() {
            //     println!("{:07b}", *row);
            // }
            // println!("====================");

            match pattern[pattern_idx] {
                b'<' => {
                    if rock_can_move_left(rock, &stack, y) {
                        rock = rock.push_left();
                    }
                },
                b'>' => {
                    if rock_can_move_right(rock, &stack, y) {
                        rock = rock.push_right();
                    }
                },
                c => panic!("unexpected character in pattern '{:?}'", std::char::from_u32(c as u32))
            }
            pattern_idx = (pattern_idx + 1) % pattern.len();

            if !rock_can_move_down(rock, &stack, y) {
                rock_place(rock, &mut stack, y);
                break 'rock_falling;
            }
        }

        if num_rocks == num_rocks_to_generate {
            break 'rock_generator;
        }
    }

    stack
}

fn generate_rocks2(num_rocks_to_generate: usize) -> usize {
    let pattern = INPUT.trim().as_bytes();
    let mut stack = Vec::new();
    let mut rock_idx = 0usize;
    let mut pattern_idx = 0usize;
    let mut num_rocks = 0usize;

    struct CycleState { num_rocks: usize, height: usize }
    let mut cycle_cache = HashMap::new();

    'rock_generator: loop {
        num_rocks += 1;

        let start_rock_idx = rock_idx;
        let start_pattern_idx = pattern_idx;

        let mut rock = ROCKS[rock_idx];
        rock_idx = (rock_idx + 1) % ROCKS.len();

        'rock_falling: for y in (0..=(stack.len() + 3)).rev() {
            match pattern[pattern_idx] {
                b'<' => {
                    if rock_can_move_left(rock, &stack, y) {
                        rock = rock.push_left();
                    }
                },
                b'>' => {
                    if rock_can_move_right(rock, &stack, y) {
                        rock = rock.push_right();
                    }
                },
                c => panic!("unexpected character in pattern '{:?}'", std::char::from_u32(c as u32))
            }
            pattern_idx = (pattern_idx + 1) % pattern.len();

            if !rock_can_move_down(rock, &stack, y) {
                rock_place(rock, &mut stack, y);
                break 'rock_falling;
            }
        }

        if let Some(prev) = cycle_cache.insert((start_rock_idx, start_pattern_idx), CycleState {
            num_rocks,
            height: stack.len(),
        }) {
            let rocks_per_cycle = num_rocks - prev.num_rocks;
            let remaining_rocks = num_rocks_to_generate - num_rocks;
            if remaining_rocks % rocks_per_cycle == 0 { // start of a new cycle
                let remaining_cycles = remaining_rocks / rocks_per_cycle;
                let cycle_height = stack.len() - prev.height;
                let remaining_height = remaining_cycles * cycle_height;
                let total_height = stack.len() + remaining_height;
                return total_height;
            }
        }

        if num_rocks == num_rocks_to_generate {
            break 'rock_generator;
        }
    }

    stack.len()
}

fn main() {
    let _p = profiler::profile();

    {
        let res1_stack = generate_rocks(2022);

        println!("====================");
        for row in res1_stack.iter().rev() {
            println!("{:07b}", *row);
        }
        println!("====================");

        let res1 = res1_stack.len();
        println!("[Part 1] Result is {res1}");
    }
    {
        let res2 = generate_rocks2(1000000000000);
        println!("[Part 2] Result is {res2}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision() {
        let test_stack = vec![
            0b0011110,
            0b0001000,
            0b0011100,
            0b0001000,
        ];
        let rock = Rock {
            shape: [
                0b0000000,
                0b0010000,
                0b0010000,
                0b1110000,
            ]
        };
        assert!(!rock_can_move_right(rock, &test_stack, 3));
    }
}