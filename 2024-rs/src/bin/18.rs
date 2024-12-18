use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    mem, usize, vec,
};

use advent_of_code::{Direction, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
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
) -> Option<usize> {
    let mut cheapest_locations = vec![vec![usize::MAX; memory.width()]; memory.height()];
    // cheapest_locations[start.1][start.0] = 0;
    // memory.show_map(|b| if *b { '#' } else { '.' });
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State {
        pos: start,
        steps: 0,
    });
    while let Some(State { pos, steps }) = heap.pop() {
        let best_cost = &mut cheapest_locations[pos.1][pos.0];
        if *best_cost <= steps {
            // Stop searching, we've been here before for less!
            continue;
        } else {
            *best_cost = steps;
        }
        // println!("{pos:?}, {steps}");
        if pos == end {
            return Some(steps);
        }

        for dir in DIRECTIONS {
            let new_pos = dir.step_usize(pos);
            if memory.getxy_pos(new_pos) == Some(&false)
                && cheapest_locations[new_pos.1][new_pos.0] > steps + 1
            {
                heap.push(State {
                    pos: new_pos,
                    steps: steps + 1,
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
        .map(|l| <(_, _) as Pos2D<i32>>::from(l).unwrap())
        .collect_vec();
    let is_example = coords.len() < 30;

    let end = if is_example { (6, 6) } else { (70, 70) };
    let bytecount = if is_example { 12 } else { 1024 };

    let mut memory = vec![vec![false; end.0 + 1]; end.1 + 1];
    for byte in coords[..bytecount].iter() {
        memory[byte.1 as usize][byte.0 as usize] = true;
    }
    let shortest = shortest_path(&memory, (0, 0), end);

    let mut breaking_byte = None;
    for (idx, (x, y)) in coords.iter().enumerate().skip(bytecount) {
        memory[*y as usize][*x as usize] = true;
        if shortest_path(&memory, (0, 0), end).is_none() {
            breaking_byte = Some(format!("{x},{y}"));
            break;
        }
    }

    (
        Some(format!("{}", shortest.unwrap())),
        Some(breaking_byte.unwrap()),
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
