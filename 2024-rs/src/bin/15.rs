use std::fmt::{Display, Write};

use advent_of_code::{Direction, Grid, Pos2D};
use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy)]
enum Tile {
    Space,
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy)]
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

impl WideTile {
    fn from(c: char) -> WideTile {
        match c {
            '.' | '@' => WideTile::Space,
            '#' => WideTile::Wall,
            '[' => WideTile::BoxLeft,
            ']' => WideTile::BoxRight,
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
    Vec<Direction>,
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
            match Tile::from(c) {
                Tile::Space => wide_row.extend(vec![WideTile::Space, WideTile::Space]),
                Tile::Box => wide_row.extend(vec![WideTile::BoxLeft, WideTile::BoxRight]),
                Tile::Wall => wide_row.extend(vec![WideTile::Wall, WideTile::Wall]),
            }
        }
        warehouse.push(row);
        wide_warehouse.push(wide_row);
    }
    let moves: Vec<Direction> = input_vec[1]
        .trim()
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
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

fn find_wide_box_group(
    warehouse: &impl Grid<Tile>,
    from: (usize, usize),
    dir: Direction,
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

fn walk_robot(moves: Vec<Direction>, mut robot: (usize, usize), warehouse: &mut Vec<Vec<Tile>>) {
    for m in moves {
        let new_robot: (usize, usize) = m.step_usize(robot);
        match warehouse.getxy_pos(new_robot) {
            Some(Tile::Space) => robot = new_robot,
            Some(Tile::Wall) | None => continue,
            Some(Tile::Box) => {
                if let Some(box_dest) = find_wide_box_group(&*warehouse, new_robot, m) {
                    *warehouse.getxy_pos_mut(new_robot).unwrap() = Tile::Space;
                    *warehouse.getxy_pos_mut(box_dest).unwrap() = Tile::Box;
                    robot = new_robot;
                }
            }
        }
    }
}

fn walk_robot_wide(
    moves: Vec<Direction>,
    mut robot: (usize, usize),
    wide_warehouse: &mut Vec<Vec<WideTile>>,
) {
    for m in moves {
        let new_robot: (usize, usize) = m.step_usize(robot);
        match wide_warehouse.getxy_pos(new_robot) {
            Some(WideTile::Space) => robot = new_robot,
            Some(WideTile::Wall) | None => continue,
            Some(WideTile::BoxLeft) | Some(WideTile::BoxRight) => {
                // Find wide box group
                // Try move group in direction (return step count)
                // Insert spaces at original locations
                // Insert boxes at offset locations
            }
        }
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (mut warehouse, mut wide_warehouse, mut robot, moves) = parse_input(input);

    walk_robot(moves, robot, &mut warehouse);
    warehouse.show_display();
    wide_warehouse.show_display();

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

    (Some(part_one as u64), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(10092), None));
    }
}
