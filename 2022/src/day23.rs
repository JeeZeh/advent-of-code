use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Add, AddAssign, Sub},
};

use crate::aocutil::Grid;

const NORTH_WEST: Pos = Pos(-1, -1);
const NORTH: Pos = Pos(0, -1);
const NORTH_EAST: Pos = Pos(1, -1);
const EAST: Pos = Pos(1, 0);
const SOUTH_EAST: Pos = Pos(1, 1);
const SOUTH: Pos = Pos(0, 1);
const SOUTH_WEST: Pos = Pos(-1, 1);
const WEST: Pos = Pos(-1, 0);

const DIRECTIONS: [Pos; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];

const CHECK: [fn([bool; 8]) -> bool; 4] = [
    |s: [bool; 8]| s[0] & s[1] & s[7], // North
    |s: [bool; 8]| s[3] & s[4] & s[5], // South
    |s: [bool; 8]| s[5] & s[6] & s[7], // West
    |s: [bool; 8]| s[1] & s[2] & s[3], // East
];

const CHECK_RESULT: [Pos; 4] = [NORTH, SOUTH, WEST, EAST];

pub fn solve(input: String) -> (usize, usize) {
    let mut state = parse_initial_state(input);

    let mut round = 0;
    while round < 10 {
        play_state(&mut state, round);
        round += 1;
    }

    let part_one = count_empty_bounded(&state);

    while !play_state(&mut state, round) {
        round += 1;
    }

    (part_one, round + 1)
}

fn count_empty_bounded(state: &HashSet<Pos>) -> usize {
    let min_x = state.iter().min_by_key(|p| p.0).unwrap().0;
    let min_y = state.iter().min_by_key(|p| p.1).unwrap().1;
    let max_x = state.iter().max_by_key(|p| p.0).unwrap().0;
    let max_y = state.iter().max_by_key(|p| p.1).unwrap().1;

    ((min_y.abs_diff(max_y) + 1) * (min_x.abs_diff(max_x) + 1)) as usize - state.len()
}

fn print_state(state: &HashSet<Pos>) {
    let min_x = state.iter().min_by_key(|p| p.0).unwrap().0 - 1;
    let min_y = state.iter().min_by_key(|p| p.1).unwrap().1 - 1;
    let max_x = state.iter().max_by_key(|p| p.0).unwrap().0 + 1;
    let max_y = state.iter().max_by_key(|p| p.1).unwrap().1 + 1;

    let mut to_print = Vec::new();

    for y in min_y - 1..max_y + 1 {
        let mut row = Vec::new();
        for x in min_x - 1..max_x + 1 {
            if state.contains(&Pos(x, y)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        to_print.push(row);
    }

    to_print.show_display();
}

fn check_around(state: &HashSet<Pos>, pos: &Pos, round: usize) -> Option<Pos> {
    let mut empty = [true; 8];
    for (i, dir) in DIRECTIONS.iter().enumerate() {
        empty[i] = !state.contains(&(*dir + *pos));
    }

    if empty.iter().all(|e| *e) {
        return None;
    }

    for check_idx in round..round + 4 {
        let wrapped = check_idx.rem_euclid(4);
        if CHECK[wrapped](empty) {
            return Some(*pos + CHECK_RESULT[wrapped]);
        }
    }
    None
}

fn play_state(state: &mut HashSet<Pos>, round: usize) -> bool {
    let mut proposed_moves: HashMap<Pos, Option<Pos>> = HashMap::new();

    for elf in state.iter() {
        if let Some(new_pos) = check_around(state, elf, round) {
            // If something already proposes this space, mark it as None to indicate
            // that no movement to this position is allowed
            if let Some(already_present) = proposed_moves.get_mut(&new_pos) {
                *already_present = None;
            } else {
                // Otherwise store the elf and its new proposed position
                proposed_moves.insert(new_pos, Some(*elf));
            }
        }
    }

    let mut stagnant = true;

    proposed_moves
        .iter()
        .filter(|(_, e)| e.is_some())
        .for_each(|(new, old)| {
            stagnant = false;
            state.remove(&old.unwrap());
            state.insert(*new);
        });

    stagnant
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos(i32, i32);

impl Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

fn parse_initial_state(input: String) -> HashSet<Pos> {
    let mut state = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                state.insert(Pos(x as i32, y as i32));
            }
        }
    }

    state
}
