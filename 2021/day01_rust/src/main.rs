use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part 1: {}", get_sliding_sums(&input, 1));
    println!("Part 2: {}", get_sliding_sums(&input, 3));
}

fn get_sliding_sums(readings: &Vec<i32>, window_size: usize) -> usize {
    readings
        .windows(window_size)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .filter(|w| w.first() < w.last())
        .count()
}
