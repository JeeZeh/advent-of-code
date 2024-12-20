use std::{collections::HashMap, usize};

use advent_of_code::{Direction, Grid};

advent_of_code::solution!(20);

fn parse_track(input: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let mut grid = vec![];
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut row = vec![false; line.len()];
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => row[x] = true,
                'S' => start = Some((x, y)),
                'E' => end = Some((x, y)),
                _ => panic!("Unknown char: {c}"),
            };
        }
        grid.push(row);
    }

    (
        grid,
        start.expect("Didn't find start!"),
        end.expect("Didn't find end!"),
    )
}

fn build_distances_from_end(
    track: &impl Grid<bool>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX; track.width()]; track.height()];

    let mut current = end;
    let mut steps = 0;
    while current != start {
        for dir in Direction::iterator() {
            let next = dir.step_usize(current);
            if track.getxy_pos(next) == Some(&false)
                && distances.getxy_pos(next).unwrap() == &usize::MAX
            {
                *distances.getxy_pos_mut(current).unwrap() = steps;
                current = next;
                steps += 1;
                break;
            }
        }
    }

    *distances.getxy_pos_mut(start).unwrap() = steps;

    distances
}

fn run_race(distances: &impl Grid<usize>, max_duration: usize) -> HashMap<usize, usize> {
    let mut cheats: HashMap<usize, usize> = HashMap::new();

    for (start, &start_dist) in distances.scan() {
        if start_dist == usize::MAX {
            continue;
        }

        for dy in (start.1 as i32 - max_duration as i32).max(0)
            ..(start.1 as i32 + max_duration as i32 + 1).min(distances.height() as i32)
        {
            for dx in (start.0 as i32 - max_duration as i32).max(0)
                ..(start.0 as i32 + max_duration as i32 + 1).min(distances.width() as i32)
            {
                let check = (dx as usize, dy as usize);
                let abs_dist = start.0.abs_diff(check.0) + start.1.abs_diff(check.1);
                if abs_dist > max_duration {
                    continue;
                }

                match distances.getxy_pos(check) {
                    Some(&usize::MAX) | None => continue,
                    Some(&end_dist) => {
                        let saved = (start_dist as i32 - end_dist as i32) - abs_dist as i32;

                        // println!("{saved:?}");
                        if saved > 0 {
                            *cheats.entry(saved as usize).or_default() += 1
                        }
                    }
                }
            }
        }
    }
    cheats
}

// HINT USED: I had lots of trouble with BFS edge cases, found that Manhattan distance can be used
// instead.
pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (track, start, end) = parse_track(input);
    let distances = build_distances_from_end(&track, start, end);

    let saving_cmp = if distances.height() < 20 { 12 } else { 100 };

    let short_cheats = run_race(&distances, 2);
    let long_cheats = run_race(&distances, 20);

    (
        Some(
            short_cheats
                .iter()
                .filter(|(&k, _)| k >= saving_cmp)
                .map(|(_, v)| v)
                .sum::<usize>(),
        ),
        Some(
            long_cheats
                .iter()
                .filter(|(&k, _)| k >= 100)
                .map(|(_, v)| v)
                .sum::<usize>(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(8), Some(41)));
    }
}
