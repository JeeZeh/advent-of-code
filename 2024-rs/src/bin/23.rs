use core::net;
use std::borrow::Borrow;
use std::iter::once;

use advent_of_code::{lines_no_empty, Pairs};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

fn bron_kerbosch<'a>(
    r: FxHashSet<&'a str>,
    mut p: FxHashSet<&'a str>,
    mut x: FxHashSet<&'a str>,
    neighbors: &FxHashMap<&'a str, FxHashSet<&'a str>>,
    clique: &mut FxHashSet<&'a str>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > clique.len() {
            *clique = r;
        }
        return;
    }

    for &vertex in p.clone().iter() {
        let connected = neighbors.get(vertex).unwrap();
        bron_kerbosch(
            r.iter().copied().chain(once(vertex)).collect(),
            p.intersection(connected).copied().collect(),
            x.intersection(connected).copied().collect(),
            neighbors,
            clique,
        );
        p.remove(vertex);
        x.insert(vertex);
    }
}

pub fn solve(input: &str) -> (Option<String>, Option<String>) {
    let connections = lines_no_empty(input)
        .map(|l| l.split_once("-").unwrap())
        .collect_vec();
    let computers = connections
        .iter()
        .flat_map(|conn| once(conn.0).chain(once(conn.1)))
        .unique()
        .collect_vec();

    let mut network: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    connections.iter().for_each(|(left, right)| {
        network.entry(left).or_default().insert(right);
        network.entry(right).or_default().insert(left);
    });

    let mut combinations_with_t: FxHashSet<[&str; 3]> = FxHashSet::default();
    for &a in &computers {
        for (b, c) in network.get(a).unwrap().iter().collect_vec().pairs() {
            // HINT: Check if pairs are connected.
            if network.get(b).unwrap().contains(c) && network.get(c).unwrap().contains(b) {
                if a.starts_with("t") || c.starts_with("t") || c.starts_with("t") {
                    let mut group = [a, b, c];
                    group.sort();
                    combinations_with_t.insert(group);
                }
            }
        }
    }

    // HINT: Found Bron-Kerbosch, looked for existing Rust implementations.
    let mut largest_clique_sink: FxHashSet<&str> = FxHashSet::default();
    bron_kerbosch(
        Default::default(),
        network.keys().cloned().collect(),
        Default::default(),
        &network,
        &mut largest_clique_sink,
    );
    let password = largest_clique_sink.iter().sorted().join(",");

    if cfg!(debug_assertions) {
        println!("{connections:?}");
        println!("{computers:?}");
        println!("{network:?}");
        println!("{:?}", combinations_with_t.len());
    }

    (
        Some(format!("{}", combinations_with_t.len())),
        Some(password),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            (Some(String::from("7")), Some(String::from("co,de,ka,ta")))
        );
    }
}
