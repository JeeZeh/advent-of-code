#![feature(let_chains)]
use std::{
    cmp::Ordering,
    collections::BinaryHeap, usize, vec,
};

use advent_of_code::{Direction, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .path
            .len()
            .cmp(&self.path.len())
            .then(self.pos.0.cmp(&other.pos.0))
            .then(self.pos.1.cmp(&other.pos.1))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Down,
    Direction::Right,
    Direction::Up,
    Direction::Left,
];

fn shortest_path(
    memory: &impl Grid<bool>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut cheapest_locations = vec![vec![usize::MAX; memory.width()]; memory.height()];
    // cheapest_locations[start.1][start.0] = 0;
    // memory.show_map(|b| if *b { '#' } else { '.' });
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start,
        path: vec![start],
    });
    while let Some(State { pos, path }) = heap.pop() {
        let best_cost = &mut cheapest_locations[pos.1][pos.0];
        if *best_cost <= path.len() {
            // Stop searching, we've been here before for less!
            continue;
        } else {
            *best_cost = path.len();
        }
        // println!("{pos:?}, {steps}");
        if pos == end {
            return Some(path);
        }

        for dir in DIRECTIONS {
            let new_pos = dir.step_usize(pos);
            if memory.getxy_pos(new_pos) == Some(&false)
                && cheapest_locations[new_pos.1][new_pos.0] > path.len() + 1
            {
                let mut new_path = path.clone();
                new_path.push(new_pos);
                heap.push(State {
                    pos: new_pos,
                    path: new_path,
                });
            }
        }
    }
    None
}

pub fn solve(input: &str) -> (Option<String>, Option<String>) {
    let coords = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| <(_, _) as Pos2D<usize>>::from(l).unwrap())
        .collect_vec();
    let is_example = coords.len() < 30;

    let end = if is_example { (6, 6) } else { (70, 70) };
    let bytecount = if is_example { 12 } else { 1024 };

    // Part 1
    let mut memory = vec![vec![false; end.0 + 1]; end.1 + 1];
    for byte in coords[..bytecount].iter() {
        memory[byte.1][byte.0] = true;
    }
    let mut shortest = shortest_path(&memory, (0, 0), end).unwrap();
    let shortest_path_length = shortest.len();

    // Part 2
    let mut breaking_byte = None;
    for byte in coords[bytecount..].iter() {
        memory[byte.1][byte.0] = true;

        // Only re-compute the shortest path if a byte lands in our original shortest path.
        if let Some(path_idx) = shortest.iter().position(|p| p == byte) {
            // See if we can find a way around the blocking-byte to avoid computing the full path again.
            if let Some(detour) =
                shortest_path(&memory, shortest[path_idx - 1], shortest[path_idx + 1])
            {
                // Splice the detour into the original shortest path
                shortest = [
                    &shortest[..path_idx],
                    &detour,
                    &shortest[(path_idx + detour.len()).min(shortest.len() - 1)..],
                ]
                .concat();
            } else if let Some(fresh_path) = shortest_path(&memory, (0, 0), end) {
                // Re-compute the shortest path if we can't find a detour.
                shortest = fresh_path;
            } else {
                // If no path found, we found the breaking byte.
                breaking_byte = Some(byte);
                break;
            }
        }
    }

    (
        Some(format!("{}", shortest_path_length - 1)),
        breaking_byte.map(|(x, y)| format!("{x},{y}")),
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
            (Some(String::from("22")), Some(String::from("6,1")))
        );
    }
}
