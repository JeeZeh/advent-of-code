use std::{collections::HashMap, fmt, thread::LocalKey};

use advent_of_code::Grid;
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Copy, Clone)]
enum Tile {
    Radio(char),
    Blank,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char_ = match self {
            Tile::Radio(freq) => *freq,
            Tile::Blank => '.',
        };
        write!(f, "{:?}", char_)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: &Pos) -> Pos {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Blank,
            freq => Tile::Radio(freq),
        }
    }
}

fn within_bounds<G: Grid<Tile>>(grid: &G, pos: &Pos) -> Option<Pos> {
    let Pos(x, y) = *pos;

    if x >= 0 && (x as usize) < grid.width() && y >= 0 && (y as usize) < grid.height() {
        return Some(Pos(x, y));
    }
    None
}

fn create_antinodes<G: Grid<Tile>>(grid: &G, a: &Pos, b: &Pos, resonance: bool) -> Vec<Pos> {
    let mut antinodes = Vec::new();
    if resonance {
        antinodes.push(*a);
    }
    let offset = &b.sub(a);

    let mut test = b.add(offset);
    while let Some(antinode) = within_bounds(grid, &test) {
        antinodes.push(antinode);
        test = test.add(offset);
        if !resonance {
            break;
        }
    }
    antinodes
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut grid = Vec::new();
    let mut radios: HashMap<char, Vec<Pos>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile = Tile::from_char(c);
            if let Tile::Radio(freq) = tile {
                radios
                    .entry(freq)
                    .or_default()
                    .push(Pos(x as i32, y as i32));
            }
            row.push(tile);
        }
        grid.push(row);
    }

    let mut anti_nodes = Vec::new();
    let mut resonant_nodes = Vec::new();
    for locations in radios.values() {
        // TODO: Make Pairs a generic Iterator<(&T, &T)>
        for (i, a) in locations.iter().enumerate() {
            for b in &locations[i + 1..] {
                anti_nodes.extend(create_antinodes(&grid, a, b, false));
                anti_nodes.extend(create_antinodes(&grid, b, a, false));
                resonant_nodes.extend(create_antinodes(&grid, a, b, true));
                resonant_nodes.extend(create_antinodes(&grid, b, a, true));
            }
        }
    }

    let part_one = anti_nodes.iter().unique().count();
    let part_two = resonant_nodes.iter().unique().count();

    (Some(part_one as u64), Some(part_two as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(14), Some(34)));
    }
}
