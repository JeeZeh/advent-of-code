use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Clipping {
    pos: (usize, usize),
    duration: usize,
}

impl Ord for Clipping {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.duration.cmp(&self.duration)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Clipping {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run_race(distances: &impl Grid<usize>, max_duration: usize) -> HashMap<usize, usize> {
    let mut cheats: HashMap<usize, usize> = HashMap::new();

    for (pos, &start_dist) in distances.scan() {
        if start_dist == usize::MAX || start_dist == 0 {
            continue;
        }

        let mut heap: BinaryHeap<Clipping> = BinaryHeap::new();
        let mut seen: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

        for dir in Direction::iterator() {
            let new_pos = dir.step_usize(pos);
            if distances.getxy_pos(new_pos) == Some(&usize::MAX) {
                heap.push(Clipping {
                    pos: new_pos,
                    duration: 1,
                });
            }

            while let Some(Clipping { pos, duration }) = heap.pop() {
                if duration == max_duration {
                    continue;
                }

                for dir in Direction::iterator() {
                    let new_pos = dir.step_usize(pos);
                    if seen.contains(&(pos, new_pos)) {
                        continue;
                    } else {
                        seen.insert((pos, new_pos));
                    }

                    match distances.getxy_pos(new_pos) {
                        Some(&usize::MAX) => heap.push(Clipping {
                            pos: new_pos,
                            duration: duration + 1,
                        }),
                        Some(&shortcut_dist) => {
                            let saved = start_dist as i32 - shortcut_dist as i32 - duration as i32;
                            if saved > 0 {
                                *cheats.entry(saved as usize).or_default() += 1;
                            }
                            heap.push(Clipping {
                                pos: new_pos,
                                duration: duration + 1,
                            });
                        }
                        None => continue,
                    }
                }
            }
        }
    }
    cheats
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let (track, start, end) = parse_track(input);
    let distances = build_distances_from_end(&track, start, end);
    // track.show_map(|&c| if c { '#' } else { '.' });

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
                .filter(|(&k, _)| k >= 74)
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
        assert_eq!(result, (Some(8), None));
    }
}
