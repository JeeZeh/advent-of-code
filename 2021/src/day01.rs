pub fn solve(input: Vec<i32>) -> (usize, usize) {
    (get_sliding_sums(&input, 1), get_sliding_sums(&input, 3))
}

fn get_sliding_sums(readings: &[i32], window_size: usize) -> usize {
    readings
        .windows(window_size)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>()
        .as_slice()
        .windows(2)
        .filter(|w| w.first() < w.last())
        .count()
}
