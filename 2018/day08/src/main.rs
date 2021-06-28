use std::{fs, iter::Peekable, str::Split, time::Instant};

struct Header {
    n_child: i32,
    n_meta: i32,
}

struct Node {
    header: Header,
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn new() -> Node {
        Node {
            header: Header {
                n_child: 0,
                n_meta: 0,
            },
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

fn main() {
    let instant = Instant::now();

    let license: Node = parse_license(fs::read_to_string("./src/input").unwrap());
    println!("Part 1: {}", part_one(&license));
    println!("Part 1: {}", part_two(&license));

    println!("{}ms", instant.elapsed().as_micros() as f32 / 1000.0)
}

fn part_one(node: &Node) -> i32 {
    // What is the sum of all metadata entries?
    node.metadata.iter().sum::<i32>() + node.children.iter().map(part_one).sum::<i32>()
}

fn part_two(node: &Node) -> i32 {
    // What is the value of the root node?
    match node.children.len() {
        0 => node.metadata.iter().sum::<i32>(),
        t => node
            .metadata
            .iter()
            .map(|m| m - 1)
            .filter(|m| *m < t as i32)
            .map(|idx| part_two(&node.children[idx as usize]))
            .sum::<i32>(),
    }
}

fn parse_license(file: String) -> Node {
    let mut parts = file.split(" ").peekable();

    parse_tree(&mut parts)
}

fn parse_tree(stream: &mut Peekable<Split<&str>>) -> Node {
    let mut node = Node::new();

    node.header.n_child = stream.next().unwrap().parse().unwrap();
    node.header.n_meta = stream.next().unwrap().parse().unwrap();

    for _ in 0..node.header.n_child {
        node.children.push(parse_tree(stream))
    }

    for _ in 0..node.header.n_meta {
        node.metadata.push(stream.next().unwrap().parse().unwrap());
    }

    node
}
