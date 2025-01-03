#![feature(trait_alias)]
use std::{collections::VecDeque, u64};

use advent_of_code::{Direction, DirectionAxes, Grid};
use itertools::Itertools;

advent_of_code::solution!(10);

fn directions(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    DirectionAxes::iterator()
        .map(DirectionAxes::step)
        .map(move |(dx, dy)| ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize))
}

pub fn get_score(forest: &impl Grid<u32>, head: (usize, usize)) -> Vec<(usize, usize)> {
    let mut reachable: Vec<(usize, usize)> = Vec::new();
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::new();

    // Populate with each direction.
    directions(head).for_each(|s| queue.push_back((s, 1)));
    while let Some((this, need)) = queue.pop_front() {
        if let Some(num) = forest.getyx(this.1, this.0) {
            if *num != need {
                continue;
            }

            if *num == 9 {
                reachable.push(this);
            } else {
                directions(this).for_each(|next| queue.push_back((next, need + 1)));
            }
        }
    }

    reachable
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let forest = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let trailheads = forest
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, height)| if height == &0u32 { Some((x, y)) } else { None })
        })
        .collect_vec();

    let trail_score = trailheads
        .iter()
        .map(|head| get_score(&forest, *head))
        .collect_vec();

    (
        Some(
            trail_score
                .iter()
                .map(|t| t.iter().unique().count())
                .sum::<usize>() as u64,
        ),
        Some(trail_score.iter().map(|trails| trails.len() as u64).sum()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(36), None));
    }
}
