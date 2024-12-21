use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::{Display, Write},
};

use advent_of_code::{Direction, Grid};
use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Key {
    Up,
    Right,
    Down,
    Left,
    Activate,
}

impl Key {
    fn from_movement(from: (usize, usize), to: (usize, usize)) -> Self {
        match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
            (std::cmp::Ordering::Less, _) => Key::Left,
            (std::cmp::Ordering::Greater, _) => Key::Right,
            (_, std::cmp::Ordering::Less) => Key::Up,
            (_, std::cmp::Ordering::Greater) => Key::Down,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => Key::Activate,
        }
    }

    fn from(c: char) -> Key {
        match c {
            '^' => Key::Up,
            'v' => Key::Down,
            '<' => Key::Left,
            '>' => Key::Right,
            'A' => Key::Activate,
            _ => panic!("Unknown character {c}"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Key::Up => '^',
            Key::Down => 'v',
            Key::Left => '<',
            Key::Right => '>',
            Key::Activate => 'A',
        }
    }

    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Key::Up => Direction::Up.step_usize(pos),
            Key::Down => Direction::Down.step_usize(pos),
            Key::Left => Direction::Left.step_usize(pos),
            Key::Right => Direction::Right.step_usize(pos),
            Key::Activate => panic!("Cannot translate 'Activate' to a step"),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

#[rustfmt::skip]
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

#[rustfmt::skip]
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>'],
];

/// The cost of pressing a button on the directional pad with `level` levels of indirection
fn build_press_costs(grid: &impl Grid<char>, level: i32) -> [[usize; 5]; 5] {
    use Key::*;
    if level == 0 {
        // uniform of 1 cost for user
        [[1; 5]; 5]
    } else {
        let mut current_press_costs = [[0; 5]; 5];
        let previous_press_costs = build_press_costs(grid, level - 1);

        for from in [Up, Right, Down, Left, Activate] {
            // seen does not guarantee that each button will be evaluated just once; just that
            // we'll stop evaluating them quickly
            let mut seen = [false; 5];
            let from_pos = grid
                .find_item(from.as_char())
                .expect("Couldn't find numeric key position");

            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), from_pos, Activate));
            while let Some((Reverse(cost), pos, button)) = queue.pop() {
                let cd = Key::from(*grid.getxy_pos(pos).unwrap());
                if cost > 0 && button == Activate {
                    // first cost will be optimal, so if we have one don't update
                    if current_press_costs[from as usize][cd as usize] == 0 {
                        current_press_costs[from as usize][cd as usize] = cost;
                    }
                } else {
                    queue.push((
                        Reverse(cost + previous_press_costs[button as usize][Activate as usize]),
                        pos,
                        Activate,
                    ));
                }
                seen[cd as usize] = true;

                for (n, &check) in grid.neighbours_axis(pos) {
                    if check == ' ' {
                        continue;
                    }

                    let d = Key::from(check);
                    if seen[d as usize] {
                        continue;
                    }
                    let next_button = Key::from_movement(pos, n);
                    queue.push((
                        Reverse(cost + previous_press_costs[button as usize][next_button as usize]),
                        n,
                        next_button,
                    ));
                }
            }
        }
        current_press_costs
    }
}

/// Computes the shortest path between two numerical buttons taking into account
/// the cost of pressing the directional buttons from the previously-calculated
/// costs (based on levels of indirection).
fn shortest_path(
    grid: &impl Grid<char>,
    press_costs: [[usize; 5]; 5],
    from_n: char,
    to_n: char,
) -> usize {
    use Key::*;
    let from = grid.find_item(from_n).unwrap();
    let to = grid.find_item(to_n).unwrap();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), from, Activate));

    while let Some((Reverse(cost), pos, button)) = queue.pop() {
        if pos == to {
            if button == Activate {
                return cost;
            } else {
                queue.push((
                    Reverse(cost + press_costs[button as usize][Activate as usize]),
                    pos,
                    Activate,
                ));
            }
        } else {
            for (n, &check) in grid.neighbours_axis(pos) {
                if check == ' ' {
                    continue;
                }
                let nd = Key::from_movement(pos, n);
                queue.push((
                    Reverse(cost + press_costs[button as usize][nd as usize]),
                    n,
                    nd,
                ));
            }
        }
    }

    unreachable!()
}

fn total_input_costs(grid: &impl Grid<char>, press_costs: [[usize; 5]; 5], line: &[char]) -> usize {
    // Starting on A
    let moves = [&['A'], line].concat();
    let steps = moves
        .windows(2)
        .map(|w| shortest_path(grid, press_costs, w[0], w[1]))
        .sum::<usize>();

    steps
        * line
            .iter()
            .filter(|c| c.is_numeric())
            .join("")
            .parse::<usize>()
            .unwrap()
}

// HINT (SOLUTION) USED: https://www.reddit.com/r/adventofcode/comments/1hj2odw/2024_day_21_solutions/m34qus1/
// Tried a faulty approach for part one, didn't have the intuition or motivation to figure it out from scratch.
pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let keycodes = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let input_costs = build_press_costs(&DIRECTIONAL_KEYPAD, 2);
    let part_one: usize = keycodes
        .iter()
        .map(|keycode| total_input_costs(&NUMERIC_KEYPAD, input_costs, keycode))
        .sum();

    let input_costs = build_press_costs(&DIRECTIONAL_KEYPAD, 25);
    let part_two: usize = keycodes
        .iter()
        .map(|keycode| total_input_costs(&NUMERIC_KEYPAD, input_costs, keycode))
        .sum();

    (Some(part_one), Some(part_two))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(126384), None));
    }
}
