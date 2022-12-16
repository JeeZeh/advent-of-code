use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

/// Based on a partially faulty approach from https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j0glv1y/
pub fn solve(input: String) -> (u64, u64) {
    let mut valves = HashMap::new();
    input.lines().map(Valve::from).for_each(|v| {
        valves.insert(v.name.clone(), v);
    });

    let distances_between_valves = compute_distances(&valves);
    let all_paths_solo = compute_all_paths(&valves, &distances_between_valves, 30);
    let part_one = all_paths_solo.iter().max_by_key(|p| p.0).unwrap();

    let binding = compute_all_paths(&valves, &distances_between_valves, 26);
    let ordered_paths = binding.iter().sorted_by_key(|p| p.0).rev().collect_vec();

    let mut best_pressure = 0;

    // Thanks to https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j0gw6v7/ for simplification
    for (human_idx, (human_score, human_opens)) in ordered_paths.iter().enumerate() {
        if human_score + ordered_paths[human_idx + 1].0 < best_pressure {
            break;
        }
        for (elephant_score, elephant_opens) in ordered_paths.iter().skip(human_idx + 1) {
            if !human_opens.iter().any(|v| elephant_opens.contains(v)) {
                best_pressure = best_pressure.max(human_score + elephant_score);
            }
        }
    }

    (part_one.0, best_pressure)
}

/// Uses backtracking to find all possible paths and result pressures for a given set of distances, achievable
/// in a max_time
fn compute_all_paths<'a>(
    valves: &'a HashMap<String, Valve>,
    distances: &'a HashMap<&'a String, HashMap<&'a String, u32>>,
    max_time: i32,
) -> Vec<(u64, Vec<&String>)> {
    let mut paths_and_pressures: Vec<(u64, Vec<&'a String>)> = Vec::new();
    // Stack holds (remaining time, pressure released, path)
    let mut choices: VecDeque<(i32, u64, Vec<&'a String>, &'a String)> = VecDeque::new();
    let mut initial_path = Vec::new();
    initial_path.push(&valves.get("AA").unwrap().name);
    choices.push_back((max_time, 0, initial_path, &valves.get("AA").unwrap().name));
    while let Some((remaining, pressure, path, current)) = choices.pop_back() {
        let mut additional_choices = Vec::new();
        for (reachable, distance) in distances.get(current).unwrap() {
            // Don't take this path if we won't have time to open the valve
            // or if we've already been to that valve
            if *distance as i32 > remaining - 2 || path.contains(reachable) {
                continue;
            }
            let new_remaining = remaining - *distance as i32 - 1; // 1 to open the valve
            let new_pressure =
                pressure + (valves.get(*reachable).unwrap().flow * new_remaining as u64);
            let mut new_path = path.clone();
            new_path.push(*reachable);
            additional_choices.push((new_remaining, new_pressure, new_path, *reachable));
            // println!(
            //     "At {}, checking {} next, time left {} after path {:?}",
            //     current, reachable, remaining, path
            // );
        }
        if !additional_choices.is_empty() {
            choices.extend(additional_choices);
        } else {
            paths_and_pressures.push((pressure, path[1..].to_vec()));
        }
    }

    paths_and_pressures
}

fn compute_distances<'a>(
    valves: &'a HashMap<String, Valve>,
) -> HashMap<&'a String, HashMap<&'a String, u32>> {
    let mut distances: HashMap<&String, HashMap<&String, u32>> = HashMap::new();
    // We only care about turning on and navigating between valves with a non-zero flow rate (and the starting one of course)
    let valves_to_consider = valves
        .values()
        .filter(|v| v.flow > 0 || v.name == "AA")
        .collect_vec();
    for from_valve in &valves_to_consider {
        let distances_from_current = bfs(&valves, &from_valve.name, &valves_to_consider);
        for to_valve in &valves_to_consider {
            if from_valve.name == to_valve.name {
                continue;
            }
            if let Some(dist) = distances_from_current.get(&to_valve.name) {
                distances
                    .entry(&from_valve.name)
                    .or_default()
                    .insert(&to_valve.name, *dist);
            }
        }
    }
    distances
}

fn bfs<'a>(
    valves: &'a HashMap<String, Valve>,
    start: &'a String,
    targets: &'a Vec<&Valve>,
) -> HashMap<&'a String, u32> {
    let mut distance_to_target: HashMap<&'a String, u32> = HashMap::new();
    distance_to_target.insert(start, 0);
    let mut seen: HashSet<&'a String> = HashSet::new();
    seen.insert(start);
    let mut queue: VecDeque<&'a String> = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() && targets.iter().any(|t| !seen.contains(&t.name)) {
        let current_valve = queue.pop_front().unwrap();
        for next_valve in &valves.get(current_valve).unwrap().reachable {
            if !seen.contains(next_valve) {
                seen.insert(next_valve);
                distance_to_target.insert(
                    next_valve,
                    distance_to_target.get(current_valve).unwrap() + 1,
                );
                queue.push_back(next_valve);
            }
        }
    }
    distance_to_target
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow: u64,
    reachable: Vec<String>,
}

impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once("; ").unwrap();
        let name = left.split_whitespace().nth(1).unwrap().to_string();
        let flow = left.split("rate=").nth(1).unwrap().parse().unwrap();
        let leads_to = right
            .split("valve")
            .nth(1)
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .split(", ")
            .map(|v| v.to_string())
            .collect_vec();

        Valve {
            name,
            flow,
            reachable: leads_to,
        }
    }
}
