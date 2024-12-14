#![feature(int_roundings)]
use std::{num::ParseIntError, str::FromStr};

use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug)]
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

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut robots = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Robot::from_str(l).unwrap())
        .collect_vec();

    // println!("{robots:?}");
    let wrap_at = if robots.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };

    for sec in 0..100 {
        let mut grid_display = vec![vec!['.'; wrap_at.0 as usize]; wrap_at.1 as usize];
        for robot in robots.iter_mut() {
            robot.step(wrap_at);
            *grid_display
                .getyx_mut(robot.pos.1 as usize, robot.pos.0 as usize)
                .unwrap() = '#';
        }
        // println!("====================");
        // println!("   Seconds: {sec}   ");
        // println!("====================");
        // grid_display.show_display();
    }
    let after_100 = get_safety_score(&robots, wrap_at);

    // println!("{robots:?}");
    (Some(after_100 as u64), None)
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
