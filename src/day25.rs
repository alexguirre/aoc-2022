mod profiler;

fn snafu_digit_value(c: u8) -> i64 {
    match c {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!("unexpected character")
    }
}

fn value_to_snafu_digit(n: i64) -> u8 {
    match n {
        2 => b'2',
        1 => b'1',
        0 => b'0',
        -1 => b'-',
        -2 => b'=',
        _ => panic!("unexpected value '{n}'")
    }
}

fn parse_snafu(s: &str) -> i64 {
    let s = s.as_bytes();
    let mut res = 0i64;
    let mut multiple = 1;
    for &c in s.iter().rev() {
        res += snafu_digit_value(c) * multiple;
        multiple *= 5;
    }
    res
}

fn to_snafu(n: i64) -> String {
    let mut s = Vec::with_capacity(8);
    let mut remaining = n;
    while remaining != 0 {
        remaining += 2;
        let digit = remaining % 5 - 2;
        s.push(value_to_snafu_digit(digit));
        remaining = remaining / 5;
    }
    s.reverse();
    std::str::from_utf8(&s).unwrap().into()
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input25");

    let res1 = INPUT.lines()
        .map(parse_snafu)
        .sum::<i64>();

    println!("[Part 1] Result is {} ({})", to_snafu(res1), res1);
}
