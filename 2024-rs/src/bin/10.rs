#![feature(trait_alias)]
use std::collections::VecDeque;

use advent_of_code::{Direction, Grid};
use itertools::Itertools;

advent_of_code::solution!(10);

// trait Pos = Pos2D<usize>;
fn directions(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    Direction::iterator()
        .map(Direction::step)
        .map(move |(dx, dy)| ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize))
}

pub fn get_score(forest: &impl Grid<u32>, head: (usize, usize)) -> Vec<(usize, usize)> {
    let mut reachable: Vec<(usize, usize)> = Vec::new();
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::new();

    // Populate with each direction.
    directions(head).for_each(|s| queue.push_back((s, 1)));
    while let Some((this, need)) = queue.pop_front() {
        if let Some(num) = forest.getyx(this.1, this.0) {
            // println!("Looking at {num}@{next:?}");
            // println!("Looking at {num}@{next:?}");
            if *num != need {
                continue;
            }

            // println!("Found {num}@{this:?}");
            if *num == 9 {
                reachable.push(this);
            } else {
                directions(this).for_each(|next| queue.push_back((next, need + 1)));

                // println!("Next {num}@{next:?}");
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
                .flat_map(Vec::<_>::iter)
                .map(|iter| iter.unique().count())
                .sum(),
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
