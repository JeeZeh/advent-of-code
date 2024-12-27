use std::{
    collections::{HashSet, VecDeque},
    fmt::{Display, Write},
};

use advent_of_code::{Direction, DirectionAxes, Grid};
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Space,
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WideTile {
    Space,
    BoxLeft,
    BoxRight,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => f.write_char(' '),
            Tile::Wall => f.write_char('#'),
            Tile::Box => f.write_char('O'),
        }
    }
}

impl Display for WideTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WideTile::Space => f.write_char(' '),
            WideTile::Wall => f.write_char('#'),
            WideTile::BoxLeft => f.write_char('['),
            WideTile::BoxRight => f.write_char(']'),
        }
    }
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' | '@' => Tile::Space,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

fn parse_input(
    input: &str,
) -> (
    Vec<Vec<Tile>>,
    Vec<Vec<WideTile>>,
    (usize, usize),
    Vec<DirectionAxes>,
) {
    let input_vec = input.split("\n\n").collect_vec();
    let mut warehouse = Vec::new();
    let mut wide_warehouse = Vec::new();
    let mut robot = None;
    for (y, line) in input_vec[0].lines().enumerate() {
        let mut row = Vec::new();
        let mut wide_row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                robot = Some((x, y));
            }
            let tile = Tile::from(c);
            match Tile::from(c) {
                Tile::Space => wide_row.extend(vec![WideTile::Space, WideTile::Space]),
                Tile::Box => wide_row.extend(vec![WideTile::BoxLeft, WideTile::BoxRight]),
                Tile::Wall => wide_row.extend(vec![WideTile::Wall, WideTile::Wall]),
            }
            row.push(tile);
        }
        warehouse.push(row);
        wide_warehouse.push(wide_row);
    }
    let moves: Vec<DirectionAxes> = input_vec[1]
        .trim()
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '^' => DirectionAxes::Up,
            'v' => DirectionAxes::Down,
            '<' => DirectionAxes::Left,
            '>' => DirectionAxes::Right,
            _ => panic!("Unknown move: {}", c),
        })
        .collect_vec();

    (
        warehouse,
        wide_warehouse,
        robot.expect("Robot not found!"),
        moves,
    )
}

fn find_box_destination(
    warehouse: &impl Grid<Tile>,
    from: (usize, usize),
    dir: DirectionAxes,
) -> Option<(usize, usize)> {
    let mut check = from;
    while let Some(t) = warehouse.getxy_pos(check) {
        match t {
            // Can't push box(es) any further.
            Tile::Wall => return None,
            // Another box, keep following to the end...
            Tile::Box => check = dir.step_usize(check),
            // Box can go here!
            Tile::Space => return Some(check),
        };
    }

    None
}

fn move_wide_group_horiz(
    warehouse: &impl Grid<WideTile>,
    from: (usize, usize),
    dir: DirectionAxes,
) -> Option<Vec<((usize, usize), WideTile)>> {
    let mut check = from;
    let mut to_move = Vec::new();
    while let Some(t) = warehouse.getxy_pos(check) {
        match t {
            // Can't push box(es) any further.
            WideTile::Wall => return None,
            // Another box, keep following to the end...
            WideTile::BoxLeft | WideTile::BoxRight => {
                to_move.push((check, *t));
                check = dir.step_usize(check);
            }
            // Box can go here!
            WideTile::Space => break,
        };
    }

    Some(to_move)
}

fn move_wide_group_vert(
    warehouse: &impl Grid<WideTile>,
    from: (usize, usize),
    dir: DirectionAxes,
) -> Option<Vec<((usize, usize), WideTile)>> {
    let mut to_move = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(from);
    while let Some(check) = queue.pop_front() {
        match warehouse.getxy_pos(check) {
            // Can't push box(es) any further.
            Some(WideTile::Wall) => return None,
            // Another box, keep following to the end...
            Some(WideTile::BoxLeft) => {
                let next_pos = dir.step_usize(check);
                to_move.insert((check, WideTile::BoxLeft));
                queue.push_back(next_pos);

                to_move.insert((DirectionAxes::Right.step_usize(check), WideTile::BoxRight));
                queue.push_back(DirectionAxes::Right.step_usize(next_pos));
            }
            // We could try to merge BoxLeft and BoxRight logic, but the conditional check for
            // direction to determine which adjacent positions and tiles need to be tracked
            // causes runtime to increase by 60x! Assumption is that this is due to poorer
            // branch prediction and optimization of in-lined values.
            Some(WideTile::BoxRight) => {
                let next_pos = dir.step_usize(check);
                to_move.insert((check, WideTile::BoxRight));
                queue.push_back(next_pos);

                to_move.insert((DirectionAxes::Left.step_usize(check), WideTile::BoxLeft));
                queue.push_back(DirectionAxes::Left.step_usize(next_pos));
            }
            // Group might be able to move, keep checking
            Some(WideTile::Space) => continue,
            None => panic!("Out of bounds"),
        }
    }
    Some(to_move.into_iter().collect_vec())
}

fn try_move_wide(
    wide_warehouse: &mut Vec<Vec<WideTile>>,
    from: (usize, usize),
    dir: DirectionAxes,
) -> bool {
    let to_move = match dir {
        DirectionAxes::Up | DirectionAxes::Down => move_wide_group_vert(wide_warehouse, from, dir),
        DirectionAxes::Left | DirectionAxes::Right => move_wide_group_horiz(wide_warehouse, from, dir),
    };

    if let Some(group) = to_move {
        if dir == DirectionAxes::Left || dir == DirectionAxes::Right {
            *wide_warehouse.getxy_pos_mut(from).unwrap() = WideTile::Space;
        }
        if dir == DirectionAxes::Up || dir == DirectionAxes::Down {
            // Empty everything to be moved since it's not a linear group
            group.iter().for_each(|(old_pos, _)| {
                *wide_warehouse.getxy_pos_mut(*old_pos).unwrap() = WideTile::Space
            });
        }
        group.iter().for_each(|(old_pos, tile)| {
            // Offset and place old tiles in new positions
            *wide_warehouse
                .getxy_pos_mut(dir.step_usize(*old_pos))
                .unwrap() = *tile
        });
        return true;
    }
    false
}

fn walk_robot(moves: &[DirectionAxes], mut robot: (usize, usize), warehouse: &mut Vec<Vec<Tile>>) {
    for m in moves {
        let new_robot: (usize, usize) = m.step_usize(robot);
        match warehouse.getxy_pos(new_robot) {
            Some(Tile::Space) => robot = new_robot,
            Some(Tile::Wall) | None => continue,
            Some(Tile::Box) => {
                if let Some(box_dest) = find_box_destination(&*warehouse, new_robot, *m) {
                    *warehouse.getxy_pos_mut(new_robot).unwrap() = Tile::Space;
                    *warehouse.getxy_pos_mut(box_dest).unwrap() = Tile::Box;
                    robot = new_robot;
                }
            }
        }
    }
}

fn walk_robot_wide(
    moves: &[DirectionAxes],
    mut robot: (usize, usize),
    wide_warehouse: &mut Vec<Vec<WideTile>>,
) {
    for m in moves {
        let new_robot: (usize, usize) = m.step_usize(robot);
        match wide_warehouse.getxy_pos(new_robot) {
            Some(WideTile::Space) => robot = new_robot,
            Some(WideTile::Wall) | None => continue,
            Some(WideTile::BoxLeft) | Some(WideTile::BoxRight) => {
                if try_move_wide(wide_warehouse, new_robot, *m) {
                    // Only move the robot in the direction if we were able to move the
                    // the boxes.
                    robot = new_robot;
                }
            }
        }
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (mut warehouse, mut wide_warehouse, robot, moves) = parse_input(input);

    walk_robot(&moves, robot, &mut warehouse);
    warehouse.show_display();

    let part_one: usize = warehouse
        .scan()
        .filter_map(|(pos, tile)| {
            if matches!(tile, Tile::Box) {
                Some((pos.1 * 100) + pos.0)
            } else {
                None
            }
        })
        .sum();

    walk_robot_wide(&moves, (robot.0 * 2, robot.1), &mut wide_warehouse);
    wide_warehouse.show_display();
    let part_two: usize = wide_warehouse
        .scan()
        .filter_map(|(pos, tile)| {
            if matches!(tile, WideTile::BoxLeft) {
                Some((pos.1 * 100) + pos.0)
            } else {
                None
            }
        })
        .sum();

    (Some(part_one as u64), Some(part_two as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(10092), Some(9021)));
    }
}
