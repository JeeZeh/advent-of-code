use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

use itertools::Itertools;

use crate::aocutil::Grid;

const DIRECTIONS: [Direction; 4] = [
    Direction::Right, // 0
    Direction::Down,  // 1
    Direction::Left,  // 2
    Direction::Up,    // 3
];

pub fn solve(input: String) -> (usize, usize) {
    let (valley, blizzard) = parse_valley(input);

    let start = Pos(1, 0);
    let goal = Pos((valley.width() - 2) as i32, (valley.height() - 1) as i32);

    let start_to_end = bfs(&valley, &blizzard, start, goal, 0).unwrap();

    let end_to_snack = bfs(&valley, &blizzard, goal, start, start_to_end).unwrap();
    let snack_to_end = bfs(&valley, &blizzard, start, goal, end_to_snack).unwrap();

    (start_to_end, snack_to_end)
}

fn bfs(
    valley: &Vec<Vec<Tile>>,
    blizzard: &Vec<Vec<Option<Direction>>>,
    start: Pos,
    goal: Pos,
    tick: usize,
) -> Option<usize> {
    let mut queue: VecDeque<(usize, Pos)> = VecDeque::new();
    queue.push_back((tick, start));

    let mut seen: HashSet<(usize, Pos)> = HashSet::new();
    seen.insert((tick, start));

    while let Some((tick, pos)) = queue.pop_front() {
        if pos == goal {
            return Some(tick);
        }

        for delta in DIRECTIONS {
            let new_pos = pos + delta.into();
            if let Some(tile) = valley.getyx(new_pos.1 as usize, new_pos.0 as usize) {
                if *tile == Tile::Empty
                    && !is_blizzard_blocking(new_pos, blizzard, tick + 1)
                    && !seen.contains(&(tick + 1, new_pos))
                {
                    // can_move = true;
                    seen.insert((tick + 1, new_pos));
                    queue.push_back((tick + 1, new_pos));
                }
            }
        }

        if !is_blizzard_blocking(pos, blizzard, tick + 1) && !seen.contains(&(tick + 1, pos)) {
            seen.insert((tick + 1, pos));
            queue.push_back((tick + 1, pos));
        }
    }

    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Player,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<Pos> for Direction {
    fn into(self) -> Pos {
        match self {
            Direction::Right => Pos(1, 0),
            Direction::Down => Pos(0, 1),
            Direction::Left => Pos(-1, 0),
            Direction::Up => Pos(0, -1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

/// This method works by predicting if a blizzard will intersect the proposed position by
/// starting from the proposed position and stepping n steps (n = tick) in the positive and negative
/// x and y axes. This means we don't have to track where the blizzards are moving in time, since the
/// blizzards move predictably, only wrapping at the edges.
fn is_blizzard_blocking(pos: Pos, blizzard: &Vec<Vec<Option<Direction>>>, tick: usize) -> bool {
    // dbg!(pos);
    if pos.1 < 1 || pos.1 > blizzard.height() as i32 {
        return false;
    }
    // Shift the position to the blizzard map
    let (dx, dy) = (pos.0 - 1, pos.1 - 1);

    return [
        // Check up for blizzards pointing down
        ((dx, dy - tick as i32), Direction::Down),
        // Check down for blizzards pointing up
        ((dx, dy + tick as i32), Direction::Up),
        // Check left for blizzards pointing right
        ((dx - tick as i32, dy), Direction::Right),
        // Check right for blizzards pointing left
        ((dx + tick as i32, dy), Direction::Left),
    ]
    .iter()
    .map(|((x, y), dir)| {
        (
            blizzard[(*y as usize).rem_euclid(blizzard.height())]
                [(*x as usize).rem_euclid(blizzard.width())],
            dir,
        )
    })
    .any(|(b_direction, expected)| b_direction.is_some() && b_direction.unwrap() == *expected);

    // false
}

fn parse_valley(input: String) -> (Vec<Vec<Tile>>, Vec<Vec<Option<Direction>>>) {
    let lines = input.lines().collect_vec();

    let mut valley: Vec<Vec<Tile>> = vec![vec![Tile::Empty; lines[0].len()]; lines.len()];
    let mut blizzard: Vec<Vec<Option<Direction>>> =
        vec![vec![None; lines[0].len() - 2]; lines.len() - 2];

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '.' => valley[y][x] = Tile::Empty,
                '#' => valley[y][x] = Tile::Wall,
                '^' => blizzard[y - 1][x - 1] = Some(Direction::Up),
                'v' => blizzard[y - 1][x - 1] = Some(Direction::Down),
                '<' => blizzard[y - 1][x - 1] = Some(Direction::Left),
                '>' => blizzard[y - 1][x - 1] = Some(Direction::Right),
                _ => panic!("Unexpected tile input"),
            };
        }
    }

    (valley, blizzard)
}
