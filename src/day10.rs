mod profiler;

#[derive(Debug, Copy, Clone)]
struct CPU {
    cycle: i64,
    x: i64,
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input10");

    let mut cpu = CPU { cycle: 1, x: 1 };
    let mut screen = [['.'; 40]; 6];
    let res1 = input
        .lines()
        .flat_map(|l| match &l[..4] {
            "noop" => vec![
                { cpu = CPU { cycle: cpu.cycle + 1, ..cpu }; cpu },
            ],
            "addx" => vec![
                { cpu = CPU { cycle: cpu.cycle + 1, ..cpu }; cpu },
                { cpu = CPU { cycle: cpu.cycle + 1, x: cpu.x + l[5..].parse::<i64>().unwrap() }; cpu },
            ],
            _=> panic!("unknown instruction"),
        })
        .map(|cpu| {
            // part 1
            let signal_strength = if (cpu.cycle - 20) % 40 == 0 { cpu.cycle * cpu.x } else { 0 };

            // part 2
            {
                // not perfect, we skip the first cycle so the top-left corner is always '.'
                let pixel_x = (cpu.cycle - 1) % 40;
                let pixel_y = (cpu.cycle - 1) / 40;
                if pixel_x >= cpu.x - 1 && pixel_x <= cpu.x + 1 {
                    screen[pixel_y as usize][pixel_x as usize] = '#';
                }
            }

            signal_strength
        })
        .sum::<i64>();
    println!("[Part 1] Result is {res1}");
    println!("[Part 2] Result is:");
    for line in screen {
        println!("\t{}", line.iter().collect::<String>());
    }
}
