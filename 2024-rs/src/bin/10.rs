#![feature(trait_alias)]
use std::collections::{HashSet, VecDeque};

use advent_of_code::{Direction, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(10);

// trait Pos = Pos2D<usize>;
fn directions(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    return Direction::iterator()
        .map(Direction::step)
        .map(move |(dx, dy)| ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize));
}

pub fn get_score(forest: &impl Grid<u32>, head: (usize, usize)) -> usize {
    let mut reachable: HashSet<(usize, usize)> = HashSet::new();
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
                reachable.insert(this);
            } else {
                directions(this).for_each(|next| queue.push_back((next, need + 1)));

                // println!("Next {num}@{next:?}");
            }
        }
    }

    reachable.len()
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let forrest = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    forrest.show_debug();

    let trailheads = forrest
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
        .map(|head| get_score(&forrest, *head))
        .sum::<usize>();
    // trailheads.iter().map(|head| )
    (Some(trail_score as u64), None)
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
