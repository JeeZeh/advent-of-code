use std::{collections::HashMap, fs, iter::Peekable, str::Split, time::Instant};

use fn_cache::{FnCache, HashCache};

#[derive(Eq, Hash)]
struct Node {
    id: usize,
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let instant = Instant::now();

    let license: Node = parse_license(&fs::read_to_string("./src/input").unwrap());

    println!("Part 1: {}", part_one(&license));

    println!("Part 2: {}", part_two(&license));

    println!("{}ms", instant.elapsed().as_micros() as f32 / 1000.0)
}

fn part_one(node: &Node) -> i32 {
    // What is the sum of all metadata entries?
    node.metadata.iter().sum::<i32>() + node.children.iter().map(part_one).sum::<i32>()
}

fn part_two(node: &Node) -> i32 {
    let len_children = node.children.len();

    if node.children.len() == 0 {
        return node.metadata.iter().sum();
    }

    let mut cache_lol = [-1; 11];

    let iter = node // Otherwise, each metadata entry
        .metadata // refers to a child whose value
        .iter() // should be calculated and summed.
        .map(|m| (m - 1) as usize) // Index of child = (metadata - 1)
        .filter(|m| *m < len_children); // Only valid indexes allowed

    let mut sum: i32 = 0;

    // What is the value of a node?
    for idx in iter {
        if cache_lol[idx] == -1 {
            cache_lol[idx] = part_two(&node.children[idx])
        }

        sum += cache_lol[idx];
    }

    sum
}

fn parse_license(file: &str) -> Node {
    parse_tree(0, &mut file.split(" "))
}

fn parse_tree<'a>(id: usize, stream: &mut impl Iterator<Item = &'a str>) -> Node {
    let child_count = stream.next().unwrap().parse().unwrap();
    let metadata_count = stream.next().unwrap().parse().unwrap();

    let mut node = Node {
        id: id,
        children: Vec::with_capacity(child_count),
        metadata: Vec::with_capacity(metadata_count),
    };

    for child_id in 0..child_count {
        node.children.push(parse_tree(id + child_id + 1, stream));
    }

    for _ in 0..metadata_count {
        node.metadata.push(stream.next().unwrap().parse().unwrap());
    }

    node
}
