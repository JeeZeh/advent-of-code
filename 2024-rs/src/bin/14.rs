#![feature(int_roundings)]
use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
struct Robot {
    vx: i16,
    vy: i16,
    pos: (i16, i16),
}

impl Robot {
    fn step(&mut self, wrap_at: (i16, i16)) {
        let (wrap_x, wrap_y) = wrap_at;

        self.pos = (
            (self.pos.0 + self.vx).rem_euclid(wrap_x),
            (self.pos.1 + self.vy).rem_euclid(wrap_y),
        );
    }
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" ").unwrap();
        let pos = p.split_once("=").unwrap().1.split_once(",").unwrap();
        let vel = v.split_once("=").unwrap().1.split_once(",").unwrap();

        Ok(Robot {
            pos: (pos.0.parse()?, pos.1.parse()?),
            vx: vel.0.parse()?,
            vy: vel.1.parse()?,
        })
    }
}

fn get_safety_score(robots: &[Robot], wrap_at: (i16, i16)) -> usize {
    let mut quadrants = [0, 0, 0, 0];
    let vert = wrap_at.0.div_floor(2);
    let horz = wrap_at.1.div_floor(2);

    for (x, y) in robots.iter().map(|r| r.pos) {
        if x == vert || y == horz {
            continue;
        }
        match (x < vert, y < horz) {
            (true, true) => quadrants[0] += 1,
            (true, false) => quadrants[1] += 1,
            (false, true) => quadrants[2] += 1,
            (false, false) => quadrants[3] += 1,
        }
    }

    quadrants.iter().product()
}

const MAX_SEARCH_SECONDS: usize = 10000;

// HINTS USED: For finding the tree programmatically:
//  1.  Tree state should be one where no robots overlap since problem space
//      is *probably* generated from non-overlapping robots to start with.
//  2.  Tree state *should* also have a low safety score since most robots
//      will be grouped together in the tree.
pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut robots = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Robot::from_str(l).unwrap())
        .collect_vec();

    let wrap_at = if robots.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };

    let mut states: Vec<(usize, Vec<Robot>)> = Vec::new();
    let mut after_100 = 0;
    for sec in 0..MAX_SEARCH_SECONDS {
        for robot in robots.iter_mut() {
            robot.step(wrap_at);
        }

        // Part 1
        if (sec + 1) % 100 == 0 {
            after_100 = get_safety_score(&robots, wrap_at)
        }

        // Part 2 - Store when no robots overlap.
        if robots.iter().map(|r| r.pos).unique().count() == robots.len() {
            states.push((sec + 1, robots.clone()));
        }
    }

    // Part 2 - Find the lowest safety score of non-overlapping states.
    let (tree_at, _) = states
        .iter()
        .sorted_by(|a, b| get_safety_score(&a.1, wrap_at).cmp(&get_safety_score(&b.1, wrap_at)))
        .next()
        .unwrap();

    (Some(after_100 as u64), Some(*tree_at as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(12), None));
    }
}
