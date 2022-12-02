fn main() {

    const WIN_LUT: [u8; 3]  = [2, 0, 1]; // Rock defeats Scissors, Paper defeats Rock, Scissors defeats Paper
    const LOSS_LUT: [u8; 3] = [1, 2, 0];

    let input = include_str!("data/input2");
    let res1 = input
        .lines()
        .map(|l| {
            let opponent = l.as_bytes()[0] - 'A' as u8;
            let player = l.as_bytes()[2] - 'X' as u8;
            let score = player + 1 + if WIN_LUT[player as usize] == opponent { 6 } else if player == opponent { 3 } else { 0 };
            score as u64
        })
        .sum::<u64>();
    println!("[Part 1] Result is {res1}");

    let res2 = input
        .lines()
        .map(|l| {
            let opponent = l.as_bytes()[0] - 'A' as u8;
            let outcome = l.as_bytes()[2] - 'X' as u8;
            let score = outcome * 3 + match outcome {
                0 /* lose */ => WIN_LUT[opponent as usize] + 1,
                1 /* draw */ => opponent + 1,
                2 /* win  */ => LOSS_LUT[opponent as usize] + 1,
                _ => panic!("unexpected outcome")
            };
            score as u64
        })
        .sum::<u64>();
    println!("[Part 2] Result is {res2}");
}
