use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::{Display, Write},
};

use advent_of_code::{Direction, Grid};
use itertools::Itertools;

advent_of_code::solution!(16);

fn parse_maze(input: &str) -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let mut maze = Vec::new();

    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' | 'E' | 'S' => Tile::Space,
                '#' => Tile::Wall,
                _ => panic!("Unknown tile: {}", c),
            };
            row.push(tile);
            if c == 'S' {
                start = Some((x, y));
            }
            if c == 'E' {
                end = Some((x, y));
            }
        }
        maze.push(row);
    }

    (
        maze,
        start.expect("Never found start!"),
        end.expect("Never found end!"),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Space,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => f.write_char('.'),
            Tile::Wall => f.write_char('#'),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
    position: (usize, usize),
    heading: usize,
    cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

fn find_shortest_path(
    maze: &impl Grid<Tile>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    heap.push(Reindeer {
        position: start,
        heading: 0,
        cost: 0,
    });

    let mut cheapest_locations = HashMap::new();
    cheapest_locations.insert((start, 0), 0);
    while let Some(Reindeer {
        cost,
        position,
        heading,
    }) = heap.pop()
    {
        let best_cost = cheapest_locations
            .entry((position, heading))
            .or_insert(cost);
        if *best_cost < cost {
            // Stop searching, we've been here before for less!
            continue;
        } else {
            *best_cost = cost;
        }

        for rot in [-1, 0, 1] {
            let new_heading = ((heading as i32 + rot) as usize).rem_euclid(4);
            let step_forward = DIRECTIONS[new_heading].step_usize(position);
            if maze.getxy_pos(step_forward).unwrap() == &Tile::Space {
                heap.push(Reindeer {
                    position: step_forward,
                    heading: new_heading,
                    cost: cost + 1 + if rot != 0 { 1000 } else { 0 },
                });
            }
        }
    }

    Some(
        *cheapest_locations
            .iter()
            .filter(|((pos, _), _)| pos == &end)
            .sorted_by(|a, b| a.1.cmp(b.1))
            .next()
            .unwrap()
            .1,
    )
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (maze, start, end) = parse_maze(input);

    println!("Start = {start:?}, end = {end:?}");
    maze.show_display();
    let shortest = find_shortest_path(&maze, start, end).unwrap() as u64;

    (Some(shortest), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(11048), None));
    }
}
