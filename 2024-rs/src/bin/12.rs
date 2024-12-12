#![feature(let_chains)]
use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{Direction, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(12);

fn explore_region(grid: &impl Grid<char>, from: (usize, usize)) -> HashSet<(usize, usize)> {
    let region = grid.getyx(from.1, from.0).unwrap();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(from);
    while let Some(this) = queue.pop_front() {
        if seen.contains(&this) {
            continue;
        }
        if let Some(c) = grid.getyx(this.1, this.0)
            && c == region
        {
            seen.insert(this);
            println!("{this:?}");
            for step in Direction::iterator().map(Direction::step) {
                let next = (
                    (this.0 as i32 + step.0) as usize,
                    (this.1 as i32 + step.1) as usize,
                );
                if !seen.contains(&next) {
                    queue.push_back(next);
                }
            }
        }
    }

    seen
}

fn get_fence_cost(region: &HashSet<(usize, usize)>) -> usize {
    let mut perimeter: usize = 0;
    for pos in region {
        for step in Direction::iterator().map(Direction::step) {
            let check_bound = (
                (pos.0 as i32 + step.0) as usize,
                (pos.1 as i32 + step.1) as usize,
            );
            if !region.contains(&check_bound) {
                perimeter += 1;
            }
        }
    }

    perimeter * region.len()
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let grid = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut explored: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<(char, HashSet<(usize, usize)>)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let pos = (x, y);
            if explored.contains(&pos) {
                continue;
            }

            let new_exploration = explore_region(&grid, pos);
            explored.extend(&new_exploration);
            regions.push((*c, new_exploration));
        }
    }
    let total_cost: usize = regions
        .iter()
        .map(|(_, region)| get_fence_cost(region))
        .sum();

    println!("{regions:?}");
    (Some(total_cost as u64), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result: (Option<u64>, Option<u64>) =
            solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(1930), None));
    }
}
