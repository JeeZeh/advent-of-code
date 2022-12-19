use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

pub fn solve(input: String) -> (u32, u32) {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();

    (0, 0)
}

fn quality_level(blueprint: &Blueprint) -> usize {
    0
}

/// Uses backtracking to find all possible paths and result pressures for a given set of distances, achievable
/// in a max_time
fn compute_all_paths<'a>(blueprint: &Blueprint, max_time: i32) {
    // Stack holds (remaining time, materials, robots)
    let mut states: VecDeque<(i32, [usize; 4], [usize; 4])> = VecDeque::new();
    states.push_back((max_time, 0, [1, 0, 0, 0]));
    while let Some((remaining, materials, robots)) = states.pop_back() {
        let mut additional_choices = Vec::new();
        for (_type, need_material) in [
            blueprint.ore_robot,
            blueprint.clay_robot,
            blueprint.obsidian_robot,
            blueprint.geode_robot,
        ]
        .iter()
        .enumerate()
        {
            // Don't take this path if we won't have time to open the valve
            // or if we've already been to that valve
            if *distance as i32 > remaining - 2 || path.contains(reachable) {
                continue;
            }
            let new_remaining = remaining - *distance as i32 - 1; // 1 to open the valve
            let new_pressure =
                pressure + (valves.get(*reachable).unwrap().flow * new_remaining as u32);
            let mut new_path = path.clone();
            new_path.push(*reachable);
            additional_choices.push((new_remaining, new_pressure, new_path, *reachable));
            // println!(
            //     "At {}, checking {} next, time left {} after path {:?}",
            //     current, reachable, remaining, path
            // );
        }
        if !additional_choices.is_empty() {
            states.extend(additional_choices);
        } else {
            paths_and_pressures.push((pressure, path[1..].to_vec()));
        }
    }

    // paths_and_pressures
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Material {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
    Geode(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blueprint {
    ore_robot: Vec<Material>,
    clay_robot: Vec<Material>,
    obsidian_robot: Vec<Material>,
    geode_robot: Vec<Material>,
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
            ore_robot: vec![Material::Ore(ore_ore)],
            clay_robot: vec![Material::Ore(clay_ore)],
            obsidian_robot: vec![Material::Ore(obsidian_ore), Material::Clay(obsidian_clay)],
            geode_robot: vec![Material::Ore(geode_ore), Material::Obsidian(geode_obsidian)],
        }
    }
}
