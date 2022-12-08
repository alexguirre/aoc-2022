
mod profiler;

fn priority(item: u8) -> u64 {
    match item {
        97..=122 /* a-z */ => item as u64 - 97 + 1,
        65..=90  /* A-Z */ => item as u64 - 65 + 27,
        _ => unreachable!(),
    }
}

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input3");
    let res1 = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| (a.as_bytes(), b.as_bytes()))
        .map(|(a, b)| {
            for ca in a {
                for cb in b {
                    if ca == cb { return priority(*ca); }
                }
            }
            unreachable!()
        })
        .sum::<u64>();
    println!("[Part 1] Result is {res1}");


    let res2 = input
        .lines()
        .map(|l| l.as_bytes())
        .collect::<Vec<&[u8]>>()
        .chunks(3)
        .map(|group| {
            let a = group[0].iter().map(|i| priority(*i) as usize);
            let b = group[1].iter().map(|i| priority(*i) as usize);
            let c = group[2].iter().map(|i| priority(*i) as usize);
            let mut frequency = [0; 53]; // a-zA-Z
            a.for_each(|i| frequency[i] = 1);
            b.for_each(|i| frequency[i] = if frequency[i] == 1 { 2 } else { frequency[i] });
            c.for_each(|i| frequency[i] = if frequency[i] == 2 { 3 } else { frequency[i] });

            frequency.iter().position(|freq| *freq == 3).unwrap() as u64
        })
        .sum::<u64>();
    println!("[Part 2] Result is {res2}");
}
