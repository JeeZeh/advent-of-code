use ahash::AHashMap;
use primes::{PrimeSet, Sieve};
use std::collections::VecDeque;

use itertools::Itertools;

const START: i32 = -2;
const END: i32 = -3;

pub fn solve(lines: Vec<String>) -> (u32, u32) {
    let connections = parse_connections(&lines);

    (
        explore_rec(
            &connections,
            START.abs() as u64,
            START,
            false,
            false,
            &mut AHashMap::new(),
        ),
        explore_rec(
            &connections,
            START.abs() as u64,
            START,
            false,
            true,
            &mut AHashMap::new(),
        ),
    )
}

fn explore_rec(
    connections: &Vec<Vec<(i32, u64)>>,
    seen: u64,
    current: i32,
    visited_twice: bool,
    allowed_two_visits: bool,
    cache: &mut AHashMap<(i32, u64), u32>,
) -> u32 {
    if let Some(cached_paths) = cache.get(&(current, seen)) {
        return *cached_paths;
    }

    let mut paths = 0;

    if current == END {
        paths += &1;
    } else {
        for (connects, abs) in &connections[current.abs() as usize] {
            let visited_before = seen % abs == 0;
            if !visited_before
                || allowed_two_visits && (!visited_twice && *connects != START && *connects != END)
            {
                paths += explore_rec(
                    connections,
                    if *connects < 0 { seen * abs } else { seen },
                    *connects,
                    visited_before || visited_twice,
                    allowed_two_visits,
                    cache,
                );
            }
        }
    }

    cache.insert((current, seen), paths);

    return paths;
}

fn parse_connections(lines: &[String]) -> Vec<Vec<(i32, u64)>> {
    let mut ids: AHashMap<String, i32> = AHashMap::new();
    ids.insert(String::from("start"), START);
    ids.insert(String::from("end"), END);

    // Represent each cave as a prime number, with small caves being negative primes
    // This lets me quickly check if a cave has been visited as primes a*b*c can be
    // checked for 'visited' by checking % (a, b or c).
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

    let mut connections: Vec<Vec<(i32, u64)>> = vec![Vec::new(); size as usize];

    for line in lines {
        let mut parts = line.split('-');
        let from = ids.get(parts.next().unwrap()).unwrap();
        let to = ids.get(parts.next().unwrap()).unwrap();

        connections[from.abs() as usize].push((*to, to.abs() as u64));
        connections[to.abs() as usize].push((*from, from.abs() as u64));
    }

    connections
}
