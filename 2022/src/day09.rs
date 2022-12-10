use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos(i32, i32);

impl From<&Direction> for Pos {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Up => Pos(0, 1),
            Direction::Down => Pos(0, -1),
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
        }
    }
}

impl Pos {
    fn follow(&mut self, target: &Pos) {
        let Pos(rel_x, rel_y) = *target - *self;

        if rel_x == 0 && rel_y == 0 {
            return;
        }
        // Offset by 1 diagonally is okay
        if rel_x.abs() == 1 && rel_y.abs() == 1 {
            return;
        }
        // Offset by 1 horizontally or vertically is okay
        if (rel_x.abs() == 1 && rel_y == 0) || (rel_y.abs() == 1 && rel_x == 0) {
            return;
        }

        // Otherwise move towards the target (one step in either/both axes)
        *self += Pos(rel_x.signum(), rel_y.signum())
    }
}

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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction(Direction, usize);

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(" ").unwrap();
        let direction = match left {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let steps = right.parse().unwrap();

        Instruction(direction, steps)
    }
}

pub fn solve(input: String) -> (usize, usize) {
    let instructions = input.lines().map(Instruction::from).collect_vec();

    (
        simulate(&instructions, &mut vec![Pos(0, 0); 2]),
        simulate(&instructions, &mut vec![Pos(0, 0); 10]),
    )
}

fn simulate(instructions: &[Instruction], entities: &mut Vec<Pos>) -> usize {
    let mut part_one_visited: HashSet<Pos> = HashSet::new();
    part_one_visited.insert(Pos(0, 0));
    instructions.iter().for_each(|Instruction(dir, steps)| {
        move_entities(&mut part_one_visited, entities, dir, *steps)
    });
    part_one_visited.len()
}

fn move_entities(
    visited: &mut HashSet<Pos>,
    entities: &mut Vec<Pos>,
    direction: &Direction,
    steps: usize,
) {
    let len = entities.len();
    for _ in 0..steps {
        for i in 0..len {
            if i == 0 {
                let target = entities.get_mut(i).unwrap();
                *target += direction.into();
            } else {
                let target = *entities.get(i - 1).unwrap();
                let entity = entities.get_mut(i).unwrap();
                entity.follow(&target);
                // Only track the tail position in visited list
                if i == len - 1 {
                    visited.insert(*entity);
                }
            }
        }
    }
}
