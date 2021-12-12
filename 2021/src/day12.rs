use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// #[derive(PartialEq, Eq, Clone)]
// struct Cave {
//     is_big: bool,
//     id: String,
//     connects: Vec<Cave>,
// }

pub fn solve(lines: Vec<String>) -> (u32, u32) {
    let connections = parse_connections(&lines);
    let mut seen = HashSet::new();
    seen.insert(-1);
    let all_paths = explore_rec(&connections, &mut seen, -1, false);
    (all_paths, 0)
}

fn explore_rec(
    connections: &HashMap<i8, Vec<i8>>,
    seen: &mut HashSet<i8>,
    current: i8,
    visited_twice: bool,
) -> u32 {
    let mut paths = 0;
    // dbg!(&seen, current, visited_twice);

    if current == -2 {
        paths += &1;
    } else {
        for connects in connections.get(&current).unwrap() {
            let visited_before = seen.contains(connects);
            // println!("{} ? {} ({})", current, connects, visited_before);
            if !visited_before || (!visited_twice && *connects != -1 && *connects != -1) {
                let mut seen = seen.clone();
                if *connects < 0 {
                    seen.insert(*connects);
                }
                paths += explore_rec(
                    connections,
                    &mut seen,
                    *connects,
                    visited_before || visited_twice,
                );
                if *connects < 0 {
                    seen.remove(&connects);
                }
            }
        }
    }

    return paths;
}

// fn explore(connections: &HashMap<i8, Vec<i8>>) -> Vec<Vec<String>> {
//     let mut paths: Vec<Vec<String>> = Vec::new();
//     let mut seen: HashSet<String> = HashSet::new();
//     seen.insert(String::from("start"));

//     let mut queue: VecDeque<(Vec<String>, HashSet<String>, bool)> = VecDeque::new();
//     queue.push_back((vec![String::from("start")], seen, false));

//     while let Some((path, seen, visited_twice)) = queue.pop_front() {
//         let current = path.last().unwrap();

//         if current == "end" {
//             paths.push(path);
//         } else {
//             for connects in connections.get(current).unwrap() {
//                 let visited_before = seen.contains(connects);
//                 if !visited_before || (!visited_twice && connects != "start" && connects != "end") {
//                     let mut path = path.clone();
//                     let mut seen = seen.clone();
//                     if &connects.to_ascii_lowercase() == connects {
//                         seen.insert(connects.to_string());
//                     }
//                     path.push(connects.to_string());
//                     queue.push_back((path, seen, visited_before || visited_twice));
//                 }
//             }
//         }
//     }

//     // dbg!(paths.iter().map(|p| p.join(",")).sorted());

//     paths
// }

fn parse_connections(lines: &[String]) -> HashMap<i8, Vec<i8>> {
    let mut connections: HashMap<i8, Vec<i8>> = HashMap::new();
    let mut ids: HashMap<String, i8> = HashMap::new();
    ids.insert(String::from("start"), -1);
    ids.insert(String::from("end"), -2);
    // let mut caves: HashMap<String, Cave> = HashMap::new();

    for cave in lines
        .iter()
        .map(|l| l.split('-').map(String::from).collect::<Vec<String>>())
        .flatten()
        .unique()
    {
        if !ids.contains_key(&cave) {
            ids.insert(
                String::from(&cave),
                (ids.len() + 1) as i8 * if cave == cave.to_lowercase() { -1 } else { 1 },
            );
        }
    }

    for line in lines {
        let mut parts = line.split('-');
        let from = ids.get(parts.next().unwrap()).unwrap();
        let to = ids.get(parts.next().unwrap()).unwrap();

        connections.entry(*from).or_insert(Vec::new()).push(*to);

        connections.entry(*to).or_insert(Vec::new()).push(*from);
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
