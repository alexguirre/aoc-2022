use std::cmp::max;
mod profiler;

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input1");
    let res = input
        .lines()
        .map(|l| if l.len() > 0 { Some(l.parse::<u32>().unwrap()) } else { None })
        .fold((0u32, 0u32, [0u32; 3]), |mut acc, n| {
            // acc.0 = current group sum
            // acc.1 = max sum (part 1)
            // acc.2 = stack of top 3 max sums (part 2)
            if let Some(num) = n {
                acc.0 += num;
            } else {
                acc.1 = max(acc.0, acc.1);

                if acc.1 > acc.2[0] { // try push new max sum into top 3 stack
                    if acc.2[0] > acc.2[1] {
                        if acc.2[1] > acc.2[2] {
                            acc.2[2] = acc.2[1];
                        }
                        acc.2[1] = acc.2[0];
                    }
                    acc.2[0] = acc.1;
                }

                acc.0 = 0; // reset sum
            }
            acc
        });
    let part1 = res.1;
    let part2 = res.2.iter().copied().reduce(|acc, n| acc + n).unwrap();
    println!("[Part 1] Result is {part1}");
    println!("[Part 2] Result is {part2} ({:?})", res.2);
}
