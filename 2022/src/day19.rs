use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub fn solve(input: String) -> (i32, i32) {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();
    let mut part_one = 0;
    part_one = dbg!(blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as i32 * dbg!(get_best_possible_geodes(b, 24)))
        .sum());

    let mut part_two = 0;
    part_two = blueprints
        .iter()
        .take(3)
        .map(|b| dbg!(get_best_possible_geodes(b, 32)))
        .reduce(|acc, new| acc * new)
        .unwrap();

    (part_one, part_two)
}

/// Uses backtracking to find all possible paths and result pressures for a given set of distances, achievable
/// in a max_time
fn get_best_possible_geodes<'a>(blueprint: &Blueprint, max_time: i32) -> i32 {
    let mut best = 0;
    // Stack holds (remaining time, materials, robots, built_last)
    let mut states: VecDeque<(i32, [i32; 4], [i32; 4], bool, [i32; 4])> = VecDeque::new();
    states.push_back((max_time, [0, 0, 0, 0], [1, 0, 0, 0], false, [0, 0, 0, 0]));
    let mut best_states: HashMap<i32, i32> = HashMap::new();
    for i in 0..=max_time {
        best_states.insert(i, 0);
    }
    while let Some((remaining, materials, robots, built_last_time, previous_materials)) =
        states.pop_front()
    {
        let best_at_time = *best_states.get(&remaining).unwrap();
        let geodes = materials[3];
        if geodes < best_at_time {
            continue;
        }
        if geodes > best_at_time {
            best_states.insert(remaining, geodes);
        }
        if remaining == 0 {
            continue;
        }

        if can_build(&materials, &blueprint.geode_robot) {
            let mut new_materials = [
                materials[0] - &blueprint.geode_robot[0],
                materials[1] - &blueprint.geode_robot[1],
                materials[2] - &blueprint.geode_robot[2],
                materials[3] - &blueprint.geode_robot[3],
            ];
            let mut new_robots = robots.clone();
            collect_materials(&mut new_materials, robots);
            new_robots[3] += 1;
            states.push_back((
                remaining - 1,
                new_materials,
                new_robots,
                true,
                materials.clone(),
            ));
            continue;
        }

        // Try just collecting new materials
        let mut new_materials = materials.clone();
        collect_materials(&mut new_materials, robots);
        states.push_back((
            remaining - 1,
            new_materials,
            robots.clone(),
            false,
            materials.clone(),
        ));

        for robot in 0..=2 {
            let requires = &blueprint.get_target_requirements(robot);
            if can_build(&materials, requires)
                && should_build(
                    blueprint,
                    robot,
                    &robots,
                    requires,
                    built_last_time,
                    &previous_materials,
                )
            {
                let mut new_materials = [
                    materials[0] - requires[0],
                    materials[1] - requires[1],
                    materials[2] - requires[2],
                    materials[3] - requires[3],
                ];
                let mut new_robots = robots.clone();
                collect_materials(&mut new_materials, robots);
                new_robots[robot] += 1;
                states.push_back((
                    remaining - 1,
                    new_materials,
                    new_robots,
                    true,
                    materials.clone(),
                ));
            }
        }
    }

    *best_states.values().max().unwrap()
}

fn should_build(
    blueprint: &Blueprint,
    robot: usize,
    robots: &[i32; 4],
    requires: &[i32; 4],
    built_last: bool,
    previous_materials: &[i32; 4],
) -> bool {
    if robot == 3 {
        return true;
    }

    let material_cost = |c: [i32; 4]| match robot {
        0 => c[0],
        1 => c[1],
        2 => c[2],
        _ => unreachable!(),
    };

    let max_cost = [
        blueprint.ore_robot,
        blueprint.clay_robot,
        blueprint.obsidian_robot,
        blueprint.geode_robot,
    ]
    .into_iter()
    .map(material_cost)
    .max()
    .unwrap_or(0);

    let still_needed = robots[robot] < max_cost;

    if !built_last {
        let skipped = can_build(previous_materials, requires);
        still_needed && !skipped
    } else {
        false
    }
}

fn collect_materials(have: &mut [i32; 4], robots: [i32; 4]) {
    have[0] += robots[0];
    have[1] += robots[1];
    have[2] += robots[2];
    have[3] += robots[3];
}

fn can_build(have: &[i32; 4], need: &[i32; 4]) -> bool {
    have[0] >= need[0] && have[1] >= need[1] && have[2] >= need[2] && have[3] >= need[3]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blueprint {
    ore_robot: [i32; 4],
    clay_robot: [i32; 4],
    obsidian_robot: [i32; 4],
    geode_robot: [i32; 4],
    max_needed: [i32; 4],
}

impl Blueprint {
    fn get_target_requirements(&self, target: usize) -> [i32; 4] {
        match target {
            0 => self.ore_robot,
            1 => self.clay_robot,
            2 => self.obsidian_robot,
            3 => self.geode_robot,
            _ => panic!("Unknown target"),
        }
    }
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
            max_needed: [
                ore_ore.max(clay_ore).max(obsidian_ore).max(geode_ore),
                obsidian_clay,
                geode_obsidian,
                i32::MAX,
            ],
        }
    }
}
