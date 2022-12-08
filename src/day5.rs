mod profiler;

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input5");

    let input_stacks = input.lines().take_while(|l| !l.is_empty()).collect::<Vec<_>>();
    let num_stacks = input_stacks.last().unwrap()
                                        .rsplit_once(' ').unwrap().1
                                        .parse::<usize>().unwrap();

    let mut stacks1 = vec![Vec::<char>::new(); num_stacks];

    for input_row in input_stacks.iter().rev().skip(1) {
        for i in 0..num_stacks {
            let offset = i * 4 + 1;
            let crate_id = input_row.chars().nth(offset).unwrap_or(' ');
            if crate_id != ' ' {
                stacks1[i].push(crate_id);
            }
        }
    }

    let mut stacks2 = stacks1.clone(); // for part 2

    let input_commands = input.lines().skip_while(|l| !l.is_empty()).skip(1).collect::<Vec<_>>();
    for cmd in input_commands {
        let parts = cmd.split(' ').collect::<Vec<_>>();
        let num = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        // part 1
        for _ in 0..num {
            let crate_id = stacks1[from].pop().unwrap();
            stacks1[to].push(crate_id);
        }

        // part 2
        let num_remaining = stacks2[from].len() - num;
        for i in 0..num {
            let crate_id = stacks2[from][num_remaining + i];
            stacks2[to].push(crate_id);
        }
        stacks2[from].truncate(num_remaining);
    }

    let res1 = stacks1.iter().map(|s| *s.last().unwrap()).collect::<String>();
    let res2 = stacks2.iter().map(|s| *s.last().unwrap()).collect::<String>();
    println!("[Part 1] Result is {res1}");
    println!("[Part 2] Result is {res2}");
}
