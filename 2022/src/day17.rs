use itertools::Itertools;

use crate::aocutil::Grid;

pub fn solve(input: String) -> (u64, u64) {
    let moves = input.chars().map(Move::from).collect_vec();
    let bricks = make_bricks();
    let mut chamber: Vec<Vec<bool>> = Vec::new();

    let mut turn: u64 = 0;
    let mut move_index = 0;

    // Track the cycle for part 2
    let mut cycle: Option<Vec<Vec<bool>>> = None;
    let mut cycle_repeats_at = 0;
    let mut part_one = 0;
    while part_one == 0 || cycle.is_none() {
        chamber.play_brick(
            &bricks[turn as usize % bricks.len()],
            &moves,
            &mut move_index,
        );
        turn += 1;
        // Try find the cycle during part 1
        if cycle.is_none() {
            cycle = chamber.repeats();
            cycle_repeats_at = turn;
        }
        if turn == 2022 {
            part_one = chamber.tower_height();
        }
    }

    // Reset back to the start to find the first instance of the cycle pattern
    // allowing us to skip to the end
    turn = 0;
    move_index = 0;
    chamber.clear();
    let pattern = cycle.expect("Expected to have found the cycle already!");
    let mut additional_inferred_height = 0;
    while turn < 1_000_000_000_000 {
        chamber.play_brick(
            &bricks[turn as usize % bricks.len()],
            &moves,
            &mut move_index,
        );
        turn += 1;
        if chamber.tower_height() >= pattern.len()
            && chamber[chamber.tower_height() - pattern.len()..chamber.tower_height()] == pattern
        {
            let cycle_interval = cycle_repeats_at - turn;
            let cycle_height = pattern.len();
            let turns_remaining: u64 = 1_000_000_000_000 - turn;
            let can_skip_cycles = turns_remaining.div_floor(cycle_interval);
            additional_inferred_height = can_skip_cycles * cycle_height as u64;
            turn += can_skip_cycles * cycle_interval;
        }
    }

    (
        part_one as u64,
        chamber.tower_height() as u64 + additional_inferred_height,
    )
}

fn make_bricks<'a>() -> [Vec<Vec<bool>>; 5] {
    let minus: Vec<Vec<bool>> = vec![vec![true, true, true, true]];
    let plus: Vec<Vec<bool>> = vec![
        vec![false, true, false],
        vec![true, true, true],
        vec![false, true, false],
    ];
    let corner: Vec<Vec<bool>> = vec![
        vec![false, false, true],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let bar: Vec<Vec<bool>> = vec![vec![true], vec![true], vec![true], vec![true]];
    let square: Vec<Vec<bool>> = vec![vec![true, true], vec![true, true]];

    [minus, plus, corner, bar, square]
}

trait Chamber {
    fn repeats(&self) -> Option<Vec<Vec<bool>>>;
    fn play_brick(&mut self, brick: &Vec<Vec<bool>>, moves: &[Move], move_index: &mut usize);
    fn tower_height(&self) -> usize;
    fn collides_at(&self, brick: &Vec<Vec<bool>>, pos: &(i32, i32)) -> Option<bool>;
}

impl Chamber for Vec<Vec<bool>> {
    fn repeats(&self) -> Option<Vec<Vec<bool>>> {
        for slice_size in (4..self.tower_height() / 2).rev() {
            let a = &self[self.tower_height() - slice_size..self.tower_height()];
            let b = &self[self.tower_height() - (slice_size * 2)..self.tower_height() - slice_size];
            if a == b {
                return Some(a.to_vec());
            }
        }
        None
    }

    fn play_brick(&mut self, brick: &Vec<Vec<bool>>, moves: &[Move], move_index: &mut usize) {
        // Find existing space between top of the chamber and highest brick.
        let highest_brick = self.tower_height();
        let already_free = self.height() - highest_brick;

        // Extend the current chamber by up to 3 spaces plus the height of the new brick, accounting for whatever
        // space already exists.
        self.extend(vec![
            vec![false; 7];
            (3 + brick.height())
                .checked_sub(already_free)
                .unwrap_or_default() // Don't underflow, we might not need to add more height
        ]);

        // Brick origin starts at the bottom left relative to itself, and 2 positions from the left
        // relative to the side of the chamber (bricks fall upwards, so left and right are reversed)
        let mut brick_origin: (i32, i32) = (4, (highest_brick + 3) as i32);
        loop {
            let _move = &moves[*move_index % moves.len()];
            *move_index += 1;
            // println!("{:?}, move {:?}", &brick_origin, &_move);
            let try_move_to = match _move {
                Move::Left => (brick_origin.0 + 1, brick_origin.1),
                Move::Right => (brick_origin.0 - 1, brick_origin.1),
            };
            // If we're out of bounds, don't save this move.
            if let Some(collides) = self.collides_at(brick, &try_move_to) {
                // If moving left or right is possible save this move.
                if !collides {
                    brick_origin = try_move_to;
                }
            }
            // Try move down, if None we've fallen out of bounds
            let try_fall = (brick_origin.0, brick_origin.1 - 1);
            let collides_below = self.collides_at(brick, &try_fall);
            // If we collide with something in the chamber, we're finished moving.
            if collides_below.is_none() || collides_below.unwrap() {
                break;
            } else {
                brick_origin = try_fall;
            }
        }

        // Save the brick's resting place to the chamber
        for (local_y, row) in brick.iter().rev().enumerate() {
            for (local_x, filled) in row.iter().enumerate().rev() {
                if *filled {
                    let (chamber_x, chamber_y) = (
                        brick_origin.0 as usize - local_x,
                        brick_origin.1 as usize + local_y,
                    );
                    *self.getyx_mut(chamber_y, chamber_x).unwrap() = true;
                }
            }
        }
    }

    fn tower_height(&self) -> usize {
        self.height()
            - self
                .iter()
                .rev()
                .position(|row| row.contains(&true))
                .map_or(0, |p| p)
    }

    /// Checks if the brick at a given origin (starting bottom left of the brick's boundary) collides
    /// with anything in the grid. If the brick exceeds the bounds of the grid, returns None.
    fn collides_at(&self, brick: &Vec<Vec<bool>>, (brick_x, brick_y): &(i32, i32)) -> Option<bool> {
        for (local_y, row) in brick.iter().rev().enumerate() {
            for (local_x, filled) in row.iter().enumerate().rev() {
                let (check_x, check_y) = (*brick_x - (local_x as i32), *brick_y + (local_y as i32));
                // Within bounds
                if check_x >= 0 && check_x < self.width() as i32 && check_y >= 0 {
                    // Collides
                    if *filled && self[check_y as usize][check_x as usize] {
                        return Some(true);
                    }
                } else {
                    // Out of bounds
                    return None;
                }
            }
        }
        // Doesn't collide
        Some(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Move {
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Unexpected movement!"),
        }
    }
}
