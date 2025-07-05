use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("input/example");

    let connectors = parse_input(input);
    println!("{:?}", connectors);

    let max_strength = find_max_strength(&connectors);
    println!("The maximum strength of the bridge is: {}", max_strength);
}

fn find_max_strength(connectors: &[Connector]) -> u32 {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let initial = connectors.iter().find(|c| c.is_initial()).unwrap();

    heap.push(State {
        bridge: vec![initial],
        strength: todo!(),
        connectors: todo!(),
    });
    0
}

struct State<'a> {
    bridge: Vec<&'a Connector>,
    strength: u32,
    connectors: Vec<&'a Connector>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.strength.partial_cmp(&other.strength)
    }
}

fn parse_input(input: &str) -> Vec<Connector> {
    input.lines().map(Connector::from_line).collect()
}

#[derive(Debug, Clone, Copy)]
struct Connector(u8, u8);

impl Connector {
    fn is_initial(self) -> bool {
        self.0 == 0 || self.1 == 0
    }

    fn can_connect(&self, other: &Self) -> bool {
        self.1 == other.0
    }

    fn flip(&self) -> Connector {
        Connector(self.1, self.0)
    }

    fn from_line(line: &str) -> Connector {
        let (a, b) = line.split_once('/').unwrap();
        return match (a.parse(), b.parse()) {
            (Ok(a), Ok(b)) => Connector(a, b),
            _ => panic!("Invalid connector format"),
        };
    }
}
