use std::collections::HashMap;

type FuelFn = dyn Fn(i32, i32) -> i32;

pub fn solve(line: String) -> (i32, i32) {
    let crab_positions: Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect();
    let bins = build_crab_bins(&crab_positions);

    (
        get_fuel_for_pos(&bins, get_median(&bins), &linear),
        get_fuel_for_pos(&bins, get_mean(&crab_positions), &divergent),
    )
}

fn divergent(dist: i32, count: i32) -> i32 {
    (dist * (dist + 1) / 2) * count
}

fn linear(dist: i32, count: i32) -> i32 {
    dist * count
}

fn get_fuel_for_pos(crab_bins: &HashMap<i32, i32>, pos: i32, fuel_fn: &FuelFn) -> i32 {
    crab_bins
        .iter()
        .map(|(p2, count)| fuel_fn((pos - p2).abs(), *count))
        .sum()
}

fn build_crab_bins(crabs: &Vec<i32>) -> HashMap<i32, i32> {
    let mut bins = HashMap::new();

    for crab in crabs {
        let entry = bins.entry(*crab).or_insert(0);
        *entry += 1;
    }

    bins
}

// Optimal position for Part 2 is the median
fn get_median(bins: &HashMap<i32, i32>) -> i32 {
    *bins.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
}

// Optimal position for Part 2 is the mean
// https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
fn get_mean(crabs: &Vec<i32>) -> i32 {
    let mean = crabs.iter().sum::<i32>() / crabs.len() as i32;
    mean
}
