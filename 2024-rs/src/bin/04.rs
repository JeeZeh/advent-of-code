use std::collections::VecDeque;

use advent_of_code::{lines_no_empty, Direction, DirectionAll, Grid};
use itertools::Itertools;

advent_of_code::solution!(4);

fn find_xmas(search: &impl Grid<char>, from: (usize, usize)) -> usize {
    if search.getxy_pos(from) != Some(&'X') {
        return 0;
    }

    let mut count = 0;

    let mut queue = VecDeque::new();
    DirectionAll::iterator().for_each(|d| queue.push_front(('X', from, d)));

    while let Some((need, at, dir)) = queue.pop_back() {
        if search.getxy_pos(from) == Some(&need) {
            if need == 'S' {
                count += 1;
                continue;
            }

            let next = match need {
                'X' => 'M',
                'M' => 'A',
                'A' => 'S',
                _ => panic!("Disallowed need: {}", need),
            };

            queue.push_front((next, dir.step_usize(at), dir));
        }
    }

    count
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let search = lines_no_empty(input)
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let xmas_search_out = search.scan().map(|(at, _)| find_xmas(&search, at)).sum::<usize>();
    (Some(xmas_search_out as u64), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (None, None));
    }
}
