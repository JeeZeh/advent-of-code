use std::collections::HashMap;

pub fn solve(line: String) -> (i32, i32) {
    let crab_positions: Vec<i32> = line.split(",").map(|p| p.parse().unwrap()).collect();
    let bins = build_crab_bins(&crab_positions);

    let part_one_fuel: i32 = bins
        .keys()
        .map(|pos| get_fuel_for_pos(&bins, *pos, false))
        .min()
        .unwrap();

    let part_two_fuel: i32 = (*bins.keys().min().unwrap()..*bins.keys().max().unwrap())
        .map(|pos| get_fuel_for_pos(&bins, pos, true))
        .min()
        .unwrap();

    (part_one_fuel, part_two_fuel)
}

fn get_fuel_for_pos(crab_bins: &HashMap<i32, i32>, pos: i32, sequence: bool) -> i32 {
    crab_bins
        .iter()
        .map(|(p2, count)| ((pos - p2).abs(), count))
        .map(|(dist, count)| {
            if sequence {
                (dist * (dist + 1) / 2) * count
            } else {
                dist
            }
        })
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
