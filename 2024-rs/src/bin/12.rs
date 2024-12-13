#![feature(let_chains)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    i32,
};

use advent_of_code::{Direction, Grid};
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

fn get_costs(region: &HashSet<(usize, usize)>) -> (usize, usize) {
    let mut sides: HashMap<(Direction, i32), Vec<i32>> = HashMap::new();
    let mut perimeter: usize = 0;
    for pos in region {
        for dir in Direction::iterator() {
            let step = dir.step();
            let check_bound = ((pos.0 as i32 + step.0), (pos.1 as i32 + step.1));
            if !region.contains(&(check_bound.0 as usize, check_bound.1 as usize)) {
                perimeter += 1;
                match dir {
                    Direction::Up | Direction::Down => sides
                        .entry((*dir, check_bound.1))
                        .or_default()
                        .push(check_bound.0),
                    Direction::Left | Direction::Right => sides
                        .entry((*dir, check_bound.0))
                        .or_default()
                        .push(check_bound.1),
                }
            }
        }
    }

    let side_count = sides
        .values()
        .map(|sides| count_sides(sides))
        .sum::<usize>();
    (perimeter * region.len(), side_count * region.len())
}

fn count_sides(slice: &Vec<i32>) -> usize {
    let mut last = i32::MIN;
    let mut sides: usize = 1;
    for num in slice.iter().sorted() {
        if last != i32::MIN && *num != last + 1 && *num != last {
            sides += 1;
        }
        last = *num;
    }
    sides
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
    let costs: Vec<(usize, usize)> = regions
        .iter()
        .map(|(_, region)| get_costs(region))
        .collect_vec();

    (
        Some(costs.iter().map(|(p, _)| p).sum::<usize>() as u64),
        Some(costs.iter().map(|(_, s)| s).sum::<usize>() as u64),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result: (Option<u64>, Option<u64>) =
            solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(1930), Some(1206)));
    }
}
