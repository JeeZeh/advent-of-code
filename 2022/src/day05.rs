use std::str::FromStr;

use itertools::Itertools;

struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

pub fn solve(input: String) -> (usize, usize) {
    let parts = input.split_once("\n\n").unwrap();
    let instructions = parts.1.lines().map(Instruction::from).collect_vec();
    // TODO: parse crates
    (parts.0.len(), instructions.len())
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut parts = s.split(" ");
        let amount = parts.nth(1).unwrap().parse().unwrap();
        let from = parts.nth(1).unwrap().parse().unwrap();
        let to = parts.nth(1).unwrap().parse().unwrap();

        Instruction { amount, from, to }
    }
}
