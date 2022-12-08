mod profiler;

fn main() {
    let _p = profiler::profile();

    let input = include_str!("data/input4");
    let input_pairs = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)|
            (a.split_once('-').unwrap(),
             b.split_once('-').unwrap()))
        .map(|((a_min, a_max), (b_min, b_max))|
            ((a_min.parse::<i64>().unwrap(), a_max.parse::<i64>().unwrap()),
             (b_min.parse::<i64>().unwrap(), b_max.parse::<i64>().unwrap())))
        .collect::<Vec<_>>();

    let res1 = input_pairs.iter()
        .filter(|((a_min, a_max), (b_min, b_max))|
            (*a_min >= *b_min && *a_max <= *b_max) ||
            (*b_min >= *a_min && *b_max <= *a_max))
        .count();
    println!("[Part 1] Result is {res1}");

    let res2 = input_pairs.iter()
        .filter(|((a_min, a_max), (b_min, b_max))|
            (*a_min >= *b_min && *a_min <= *b_max) ||
            (*a_max >= *b_min && *a_max <= *b_max) ||
            (*b_min >= *a_min && *b_min <= *a_max) ||
            (*b_max >= *a_min && *b_max <= *a_max))
        .count();
    println!("[Part 2] Result is {res2}");
}
