use std::fs;

#[derive(Debug)]
struct Conv {
    pattern: Vec<bool>,
    result: bool,
}

impl Conv {
    fn pattern_from_sequence(sequence: &str) -> Vec<bool> {
        sequence
            .chars()
            .map(|c| match c {
                '#' => true,
                _ => false,
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");

    let (mut initial_state, convolutions) =
        parse_input(fs::read_to_string("./src/test").unwrap().as_str());

    let size = initial_state.len() * 2;

    let offset = initial_state.len() / 2;

    let mut state = Vec::with_capacity(size);

    for i in 0..size {
        if i < offset || i >= offset + initial_state.len() {
            state.push(false);
        } else {
            state.push(initial_state[i - offset]);
        }
    }
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<Conv>) {
    let mut lines = input.lines();

    let initial_state =
        Conv::pattern_from_sequence(lines.next().unwrap().split(": ").nth(1).unwrap());

    lines.next();

    let convs = lines
        .map(|l| {
            let mut parts = l.split(" => ");
            let pattern = Conv::pattern_from_sequence(parts.next().unwrap());
            let result = Conv::pattern_from_sequence(parts.next().unwrap())[0];

            Conv { pattern, result }
        })
        .collect();

    return (initial_state, convs);
}
