fn main() {
    let input = include_str!("data/input6");

    for (part, &marker_length) in [4usize, 14].iter().enumerate() {
        let (start_index, _) = input.as_bytes()
            .windows(marker_length)
            .enumerate()
            .find(|(_, w)| w.iter().all(|&n| w.iter().filter(|&&m| m == n).count() == 1))
            .unwrap();
        let res = start_index + marker_length;
        println!("[Part {}] Result is {res}", part + 1);
    }
}
