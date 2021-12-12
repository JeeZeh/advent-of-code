use primes::{PrimeSet, Sieve};
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub fn solve(lines: Vec<String>) -> (u32, u32) {
    let connections = parse_connections(&lines);

    (
        explore_rec(&connections, 2, -2, false, false),
        explore_rec(&connections, 2, -2, false, true),
    )
}

fn explore_rec(
    connections: &Vec<Vec<i32>>,
    seen: u64,
    current: i32,
    visited_twice: bool,
    allowed_two_visits: bool,
) -> u32 {
    let mut paths = 0;

    if current == -3 {
        paths += &1;
    } else {
        for connects in &connections[current.abs() as usize] {
            let to_comp = connects.abs() as u64;
            let visited_before = seen % to_comp == 0;
            if !visited_before
                || allowed_two_visits && (!visited_twice && *connects != -2 && *connects != -3)
            {
                paths += explore_rec(
                    connections,
                    if *connects < 0 { seen * to_comp } else { seen },
                    *connects,
                    visited_before || visited_twice,
                    allowed_two_visits,
                );
            }
        }
    }

    return paths;
}

fn parse_connections(lines: &[String]) -> Vec<Vec<i32>> {
    let mut ids: HashMap<String, i32> = HashMap::new();
    ids.insert(String::from("start"), -2);
    ids.insert(String::from("end"), -3);

    let unique_caves: Vec<String> = lines
        .iter()
        .map(|l| l.split('-').map(String::from).collect::<Vec<String>>())
        .flatten()
        .unique()
        .collect();

    let num_caves = unique_caves.len();
    let mut primes: VecDeque<u64> = Sieve::new()
        .iter()
        .filter(|p| *p > 3)
        .take(num_caves)
        .collect();

    for cave in unique_caves {
        if !ids.contains_key(&cave) {
            ids.insert(
                String::from(&cave),
                (primes.pop_front().unwrap()) as i32
                    * if cave == cave.to_lowercase() { -1 } else { 1 },
            );
        }
    }

    let size = ids.values().map(|id| id.abs()).max().unwrap() + 1;

    let mut connections: Vec<Vec<i32>> = vec![Vec::new(); dbg!(size) as usize];

    for line in lines {
        let mut parts = line.split('-');
        let from = ids.get(parts.next().unwrap()).unwrap();
        let to = ids.get(parts.next().unwrap()).unwrap();

        connections.get_mut(from.abs() as usize).unwrap().push(*to);
        connections.get_mut(to.abs() as usize).unwrap().push(*from);
    }

    connections
}
