use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

// #[derive(PartialEq, Eq, Clone)]
// struct Cave {
//     is_big: bool,
//     id: String,
//     connects: Vec<Cave>,
// }

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let connections = parse_connections(&lines);
    let all_paths = explore(&connections);
    (all_paths.len(), all_paths.last().unwrap().len())
}

fn explore(connections: &HashMap<String, HashSet<String>>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(String::from("start"));

    let mut queue: VecDeque<(Vec<String>, HashSet<String>, bool)> = VecDeque::new();
    queue.push_back((vec![String::from("start")], seen, false));

    while let Some((path, seen, visited_twice)) = queue.pop_front() {
        let current = path.last().unwrap();

        if current == "end" {
            paths.push(path);
        } else {
            for connects in connections.get(current).unwrap() {
                let visited_before = seen.contains(connects);
                if !visited_before || (!visited_twice && connects != "start" && connects != "end") {
                    let mut path = path.clone();
                    let mut seen = seen.clone();
                    if &connects.to_ascii_lowercase() == connects {
                        seen.insert(connects.to_string());
                    }
                    path.push(connects.to_string());
                    queue.push_back((path, seen, visited_before || visited_twice));
                }
            }
        }
    }

    // dbg!(paths.iter().map(|p| p.join(",")).sorted());

    paths
}

fn parse_connections(lines: &[String]) -> HashMap<String, HashSet<String>> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    // let mut caves: HashMap<String, Cave> = HashMap::new();

    for line in lines {
        let mut parts = line.split('-');
        let from_id = String::from(parts.next().unwrap());
        let to_id = String::from(parts.next().unwrap());

        connections
            .entry(from_id.to_string())
            .or_insert(HashSet::new())
            .insert(to_id.to_string());

        connections
            .entry(to_id.to_string())
            .or_insert(HashSet::new())
            .insert(from_id.to_string());
    }

    // for (entry, _) in connections {
    //     let is_big = entry.to_ascii_uppercase() == entry;
    //     caves.insert(
    //         String::from(&entry),
    //         Cave {
    //             is_big,
    //             id: entry,
    //             connects: Vec::new(),
    //         },
    //     );
    // }

    connections
}
