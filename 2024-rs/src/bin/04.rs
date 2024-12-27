use std::{collections::VecDeque, fmt::Display};

use advent_of_code::{
    lines_no_empty, template::runner::Solution, Direction, DirectionAll, Grid, Pos2D,
};
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
        if search.getxy_pos(at) == Some(&need) {
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

fn convolve(grid: &impl Grid<char>, kernel: &impl Grid<Option<char>>) -> i32 {
    let mut matches = 0;
    kernel.show_debug();
    for y in 0..grid.height() - (kernel.height() - 1) {
        for x in 0..grid.width() - (kernel.width() - 1) {
            if match_kernel_at(grid, kernel, x, y) {
                matches += 1;
            }
        }
    }

    matches
}

fn match_kernel_at(
    grid: &impl Grid<char>,
    kernel: &impl Grid<Option<char>>,
    x: usize,
    y: usize,
) -> bool {
    for (pos, e) in kernel.scan() {
        let check = pos.add(&(x, y));
        println!("{check:?} {:?} {:?}", grid.getxy_pos(check), e);
        if e.is_some() && grid.getxy_pos(check) != e.as_ref() {
            return false;
        }
    }

    true
}

pub fn solve(input: &str) -> Solution<u64, usize> {
    let search = lines_no_empty(input)
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let xmas_search_out = search
        .scan()
        .map(|(at, _)| find_xmas(&search, at))
        .sum::<usize>();

    let mut kernel: Vec<Vec<Option<char>>> = vec![
        vec![Some('M'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('S')],
    ];
    let mut total = 0;
    for _ in 0..3 {
        total += convolve(&search, &kernel);
        kernel = kernel.rot90();
    }
    total += convolve(&search, &kernel);

    (Some(xmas_search_out as u64), Some(total as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(18), Some(9)));
    }
}
