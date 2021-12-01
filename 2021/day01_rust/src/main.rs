use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part 1: {}", get_depth_changes(&input));
    println!("Part 2: {}", get_sliding_sums(&input, 3));
}

fn get_sliding_sums(readings: &Vec<i32>, window_size: usize) -> i32 {
    assert!(readings.len() >= window_size);

    let mut increases = 0;
    let mut last = readings[0..window_size].iter().sum();

    for i in 1..readings.len() - window_size + 1 {
        let sum: i32 = readings[i..i + window_size].iter().sum();
        if sum > last {
            increases += 1;
        }
        last = sum;
    }

    increases
}

fn get_depth_changes(readings: &Vec<i32>) -> i32 {
    let mut increases = 0;
    let mut last = readings.first().unwrap();

    for reading in readings {
        if reading > last {
            increases += 1;
        }

        last = reading;
    }

    increases
}
