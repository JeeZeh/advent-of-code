use std::fmt::{Display, Write};

use advent_of_code::{Direction, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(21);

#[derive(Debug, PartialEq, Eq)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl Key {
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

const ACTIVATE: u8 = u8::MAX;
const EMPTY: u8 = u8::MAX - 1;

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

fn get_numeric_inputs(keycode: &[char]) -> Vec<Key> {
    // If on 0, first move can't be left
    // If on 1, first move can't be down

    // Start on A
    let mut current = (2, 3);
    let mut sequence = Vec::new();

    for key in keycode {
        let key_dest = NUMERIC_KEYPAD
            .scan()
            .find(|(_, k)| *k == key)
            .expect("Couldn't find numeric key position")
            .0;

        while let Some(diff) = diff(key_dest, current) {
            let mut press = None;

            press = match diff.0.signum() {
                -1 => {
                    if current != (1, 3) {
                        Some(Key::Left)
                    } else {
                        None
                    }
                }
                1 => Some(Key::Right),
                _ => None,
            };

            if press.is_none() {
                press = match diff.1.signum() {
                    -1 => Some(Key::Up),
                    1 => {
                        if current != (0, 2) {
                            Some(Key::Down)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
            }

            // println!("{current:?}, {diff:?}, {press:?}");
            current = press.as_ref().unwrap().step(current);
            sequence.push(press.unwrap());
        }
        sequence.push(Key::Activate);
    }

    sequence
}

fn diff(a: (usize, usize), b: (usize, usize)) -> Option<(i32, i32)> {
    if a == b {
        return None;
    }
    Some((a.0 as i32 - b.0 as i32, a.1 as i32 - b.1 as i32))
}

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

fn get_directional_inputs(keycode_inputs: &[Key]) -> Vec<Key> {
    // If on ^, first move can't be left
    // If on <, first move can't be up

    // Start on A
    let mut current = (2, 0);
    let mut sequence = Vec::new();

    for key in keycode_inputs.iter().map(Key::as_char) {
        let key_dest = DIRECTIONAL_KEYPAD
            .scan()
            .find(|(_, &k)| k == key)
            .expect("Couldn't find directional key position")
            .0;

        while let Some(diff) = diff(key_dest, current) {
            let mut press = None;

            press = match diff.0.signum() {
                -1 => {
                    if current != (1, 0) {
                        Some(Key::Left)
                    } else {
                        None
                    }
                }
                1 => Some(Key::Right),
                _ => None,
            };

            if press.is_none() {
                press = match diff.1.signum() {
                    -1 => {
                        if current != (0, 1) {
                            Some(Key::Up)
                        } else {
                            None
                        }
                    }
                    1 => Some(Key::Down),
                    _ => None,
                };
            }

            // println!("{current:?}, {diff:?}, {press:?}");
            current = press.as_ref().unwrap().step(current);
            sequence.push(press.unwrap());
        }
        sequence.push(Key::Activate);
    }

    sequence
}

pub fn solve(input: &str) -> (Option<usize>, Option<usize>) {
    let keycodes = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut complexity = 0;
    for keycode in keycodes {
        let numeric = get_numeric_inputs(&keycode);
        println!("{numeric:?}");
        let directional = get_directional_inputs(&numeric);
        println!("{directional:?}");
        let remote = get_directional_inputs(&directional);
        println!("{}", remote.iter().map(Key::as_char).join(""));
        println!("{}", remote.len());
        complexity += remote.len()
            * keycode
                .iter()
                .filter(|c| c.is_numeric())
                .join("")
                .parse::<usize>()
                .unwrap();
    }

    (Some(complexity), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(126384), None));
    }

    #[test]
    fn test_keypad() {
        let possible = vec![
            "<A^A>^^AvvvA".chars().map(Key::from).collect_vec(),
            "<A^A^>^AvvvA".chars().map(Key::from).collect_vec(),
            "<A^A^^>AvvvA".chars().map(Key::from).collect_vec(),
        ];
        assert!(possible.contains(&get_numeric_inputs(&"029A".chars().collect_vec())));
    }
}
