use std::collections::HashMap;

use itertools::Itertools;

use crate::aocutil::Grid;

pub fn solve(input: String) -> (u64, u64) {
    let mut map: HashMap<Pos, bool> = HashMap::new();
    let moves = input.lines().map(Pos::from).for_each(|p| {
        map.insert(p, true);
    });

    (0, 0)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos(i32, i32, i32);

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let parts = value.split(',').map(|p| p.parse::<i32>().unwrap());

        Pos(
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        )
    }
}
