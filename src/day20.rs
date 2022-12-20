mod profiler;

fn mix(v: &Vec<i64>, decryption_key: i64, num_rounds: usize) -> Vec<i64> {
    let mut indices = (0..v.len()).collect::<Vec<_>>();
    for _ in 0..num_rounds {
        for orig_index in 0..v.len() {
            let old_index = indices.iter().position(|&idx| idx == orig_index).unwrap() as isize;
            let new_index = (old_index + (v[orig_index] * decryption_key) as isize).rem_euclid(v.len() as isize - 1);
            indices.remove(old_index as usize);
            indices.insert(new_index as usize, orig_index);
        }
    }
    indices.iter()
        .map(|&idx| v[idx] * decryption_key)
        .collect()
}

fn sum_grove_coords(v: &Vec<i64>) -> i64 {
    let zero_index = v.iter().position(|&n| n == 0).unwrap();
    [1000, 2000, 3000].into_iter()
        .map(|offset| (zero_index + offset) % v.len())
        .map(|i| v[i])
        .sum::<i64>()
}

fn main() {
    let _p = profiler::profile();

    const INPUT: &str = include_str!("data/input20");
    let numbers = INPUT.lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let res1 = sum_grove_coords(&mix(&numbers, 1, 1));
    println!("[Part 1] Result is {res1}");

    let res2 = sum_grove_coords(&mix(&numbers, 811589153, 10));
    println!("[Part 2] Result is {res2}");

}
