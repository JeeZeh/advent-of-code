use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

pub fn solve(input: String) -> (i32, u32) {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();
    let part_one = blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as i32 * get_best_possible_geodes(b, 24))
        .max()
        .unwrap();

    (part_one, 0)
}

/// Uses backtracking to find all possible paths and result pressures for a given set of distances, achievable
/// in a max_time
fn get_best_possible_geodes<'a>(blueprint: &Blueprint, max_time: i32) -> i32 {
    let mut best = 0;
    // Stack holds (remaining time, materials, robots)
    let mut states: VecDeque<(i32, [i32; 4], [i32; 4])> = VecDeque::new();
    states.push_back((max_time, [0, 0, 0, 0], [1, 0, 0, 0]));
    while let Some((remaining, materials, robots)) = states.pop_front() {
        dbg!(states.get(states.len().checked_sub(1).unwrap_or(0)));
        let mut additional_states = Vec::new();
        for (_type, need_material) in [
            blueprint.ore_robot,
            blueprint.clay_robot,
            blueprint.obsidian_robot,
            blueprint.geode_robot,
        ]
        .iter()
        .enumerate()
        // Heuristic: try build the best robot at any point
        .rev()
        {
            if remaining == 0 {
                continue;
            }

            let mut new_materials = materials.clone();
            let mut new_robots = robots.clone();
            if try_build(&mut new_materials, need_material) {
                new_robots[_type] += 1;
                // Our new robot will not be ready yet, so collect materials based on our old bots
                collect_materials(&mut new_materials, robots);
                additional_states.push((remaining - 1, new_materials, new_robots));
            }
        }
        if !additional_states.is_empty() {
            states.extend(additional_states);
        } else if remaining > 0 {
            let mut new_materials = materials.clone();
            collect_materials(&mut new_materials, robots);
            states.push_back((remaining - 1, new_materials, robots.clone()));
        } else {
            best = best.max(materials[3]);
        }
    }

    // paths_and_pressures
    best
}

fn should_build(_type: usize, robots: &[i32; 4], blueprint: &Blueprint) -> bool {
    false
}

fn collect_materials(have: &mut [i32; 4], robots: [i32; 4]) {
    have[0] += robots[0];
    have[1] += robots[1];
    have[2] += robots[2];
    have[3] += robots[3];
}

fn try_build(have: &mut [i32; 4], need: &[i32; 4]) -> bool {
    if have[0] >= need[0] && have[1] >= need[1] && have[2] >= need[2] && have[3] >= need[3] {
        have[0] -= need[0];
        have[1] -= need[1];
        have[2] -= need[2];
        have[3] -= need[3];
        return true;
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blueprint {
    ore_robot: [i32; 4],
    clay_robot: [i32; 4],
    obsidian_robot: [i32; 4],
    geode_robot: [i32; 4],
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let mut parts = value.split("Each ");
        let ore_ore = parts
            .nth(1)
            .unwrap()
            .split_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let clay_ore = parts
            .next()
            .unwrap()
            .split_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let mut obsidian_parts = parts.next().unwrap().split_whitespace();
        let obsidian_ore = obsidian_parts.nth(3).unwrap().parse().unwrap();
        let obsidian_clay = obsidian_parts.nth(2).unwrap().parse().unwrap();
        let mut obsidian_parts = parts.next().unwrap().split_whitespace();
        let geode_ore = obsidian_parts.nth(3).unwrap().parse().unwrap();
        let geode_obsidian = obsidian_parts.nth(2).unwrap().parse().unwrap();

        Blueprint {
            ore_robot: [ore_ore, 0, 0, 0],
            clay_robot: [clay_ore, 0, 0, 0],
            obsidian_robot: [obsidian_ore, obsidian_clay, 0, 0],
            geode_robot: [geode_ore, 0, geode_obsidian, 0],
        }
    }
}
