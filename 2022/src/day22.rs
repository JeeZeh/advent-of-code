use std::ops::{Add, AddAssign, Sub};

use itertools::Itertools;
use regex::Regex;

use crate::aocutil::Grid;

const HEADINGS: [Direction; 4] = [
    Direction::Right, // 0
    Direction::Down,  // 1
    Direction::Left,  // 2
    Direction::Up,    // 3
];

pub fn solve(input: String) -> (i32, i32) {
    let (grid_text, directions_text) = input.split_once("\n\n").unwrap();
    let grid = parse_grid(grid_text);
    let instructions = parse_instructions(directions_text);

    (
        walk_grid(&grid, &instructions, false),
        if grid.height() < 20 {
            -1
        } else {
            walk_grid(&grid, &instructions, true)
        },
    )
}

fn walk_grid(grid: &Vec<Vec<Tile>>, instructions: &[Instruction], as_cube: bool) -> i32 {
    let start_x = grid[0].iter().position(|t| t == &Tile::Empty).unwrap() as i32;

    let mut player = Player {
        position: Pos(start_x, 0), // Leftmost open tile of the top row of tiles
        heading_index: 0,          // Right
    };

    for instr in instructions {
        match instr {
            Instruction::Walk(steps) => player.walk_on_grid(&grid, *steps, as_cube),
            Instruction::Turn(dir) => player.rotate(&dir),
        }
    }

    1000 * (player.position.1 + 1) + 4 * (player.position.0 + 1) + player.heading_index as i32
}

#[derive(Debug)]
struct Player {
    position: Pos,
    heading_index: usize,
}

impl Player {
    fn get_heading(&self) -> Direction {
        HEADINGS[self.heading_index]
    }

    fn rotate(&mut self, dir: &Direction) {
        self.heading_index = match dir {
            Direction::Left => (self.heading_index as i8 - 1).rem_euclid(4) as usize,
            Direction::Right => (self.heading_index as i8 + 1).rem_euclid(4) as usize,
            _ => panic!("Unsupported rotation"),
        }
    }

    fn walk_on_grid(&mut self, grid: &Vec<Vec<Tile>>, steps: usize, as_cube: bool) {
        // println!("Walking {} steps {:?}", steps, self.get_heading());
        for _ in 0..steps {
            if let Some((new_position, new_heading)) = self.try_step(grid, as_cube) {
                self.position = new_position;
                self.heading_index = new_heading;
            } else {
                break;
            }
        }
    }

    fn try_step(&self, grid: &Vec<Vec<Tile>>, as_cube: bool) -> Option<(Pos, usize)> {
        let heading = self.get_heading();
        let Pos(x, y) = self.position + Pos::from(&heading);

        match grid.getyx(y as usize, x as usize) {
            Some(Tile::Empty) => Some((Pos(x, y), self.heading_index)),
            Some(Tile::Wall) => None,
            None | Some(Tile::Void) => {
                if as_cube {
                    self.try_wrap_cube(grid, &heading, Pos(x, y))
                } else {
                    self.try_wrap(grid)
                }
            }
        }
    }

    /// Horrible hard-coded translations for my input.
    /// Eric, you are cruel for making the example shape not match the real input.
    ///
    /// This function is only called when trying to walk off the 'edge' and into the void, the to_x and to_y position,
    /// since walking to connected sections (in the input) does not cause a transformation in position or heading.
    ///
    /// 1 -> 6 describe the faces, a -> e describe columns/rows in each face.
    /// I used a 10:1 scale of the input to help my brain.
    fn try_wrap_cube(
        &self,
        grid: &Vec<Vec<Tile>>,
        heading: &Direction,
        Pos(to_x, to_y): Pos,
    ) -> Option<(Pos, usize)> {
        let current_face = get_cube_face(self.position);

        let (new_pos, new_heading) = match (current_face, heading) {
            (1, Direction::Up) => (Pos(0, 100 + to_x), Direction::Right), // top 1a -> left 6a
            (1, Direction::Left) => (Pos(0, 149 - to_y), Direction::Right), // left 1a -> left 4e
            (2, Direction::Up) => (Pos(to_x - 100, 199), Direction::Up),  // top 2a -> bottom 6a
            (2, Direction::Right) => (Pos(99, 149 - to_y), Direction::Left), // right 2a -> right 5e
            (2, Direction::Down) => (Pos(99, to_x - 50), Direction::Left), // bottom 2a -> right 3a
            (3, Direction::Right) => (Pos(to_y + 50, 49), Direction::Up), // right 3a -> bottom 2a
            (3, Direction::Left) => (Pos(to_y - 50, 100), Direction::Down), // left 3a -> top 4a
            (4, Direction::Up) => (Pos(50, to_x + 50), Direction::Right), // top 4a -> left 3a
            (4, Direction::Left) => (Pos(50, 149 - to_y), Direction::Right), // left 4a -> left 1e
            (5, Direction::Right) => (Pos(149, 149 - to_y), Direction::Left), // right 5a -> right 2e
            (5, Direction::Down) => (Pos(49, to_x + 100), Direction::Left), // bottom 5a -> right 6a
            (6, Direction::Down) => (Pos(to_x + 100, 0), Direction::Down),  // bottom 6a -> top 2a
            (6, Direction::Right) => (Pos(to_y - 100, 149), Direction::Up), // right 6a -> bottom 5a
            (6, Direction::Left) => (Pos(to_y - 100, 0), Direction::Down),  // left 6a -> top 1a
            (face, dir) => panic!(
                "Unsupported cube-wrapping operation from face {:?} in direction {:?}",
                face, dir
            ),
        };

        if grid[new_pos.1 as usize][new_pos.0 as usize] == Tile::Empty {
            // println!(
            //     "At {:?} (face {:?}) heading {:?}, will appear at {:?} (face {:?}) heading {:?}",
            //     self.position,
            //     current_face,
            //     heading,
            //     new_pos,
            //     get_cube_face(new_pos),
            //     new_heading
            // );
            return Some((new_pos, new_heading.into()));
        }

        None
    }

    fn try_wrap(&self, grid: &Vec<Vec<Tile>>) -> Option<(Pos, usize)> {
        let direction = Pos::from(&self.get_heading().reverse());
        let mut current = self.position;
        let mut last;
        loop {
            last = current;
            current += direction;
            let loc = grid.getyx(current.1 as usize, current.0 as usize);
            if loc.is_none() || *loc.unwrap() == Tile::Void {
                break;
            }
        }
        if grid[last.1 as usize][last.0 as usize] == Tile::Empty {
            Some((last, self.heading_index))
        } else {
            None
        }
    }
}

fn get_cube_face(Pos(x, y): Pos) -> usize {
    if y < 50 {
        if x < 100 {
            return 1;
        }
        return 2;
    }

    if y >= 50 && y < 100 {
        return 3;
    }

    if y >= 100 && y < 150 {
        if x < 50 {
            return 4;
        }
        return 5;
    }

    6
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Void,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Walk(usize),
    Turn(Direction),
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

impl From<&Direction> for Pos {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::Up => Pos(0, -1),
            Direction::Down => Pos(0, 1),
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unexpected direction"),
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

fn parse_instructions(direction_text: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let re = Regex::new(r"(\d+|[LR]{1})").unwrap();

    for capture in re.captures_iter(direction_text) {
        instructions.push(match capture.get(0).unwrap().as_str() {
            d if d == "R" || d == "L" => Instruction::Turn(Direction::from(d)),
            s => Instruction::Walk(s.parse().unwrap()),
        });
    }

    instructions
}

fn parse_grid(grid_text: &str) -> Vec<Vec<Tile>> {
    let unparsed = grid_text
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let width = unparsed.iter().map(|r| r.len()).max().unwrap();
    let height = unparsed.len();

    let mut grid = vec![vec![Tile::Void; width]; height];

    for (y, row) in unparsed.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match *c {
                '.' => grid[y][x] = Tile::Empty,
                '#' => grid[y][x] = Tile::Wall,
                _ => (),
            };
        }
    }

    grid
}
