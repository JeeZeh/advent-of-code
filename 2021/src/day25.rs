use std::ops::Deref;

use ahash::{AHashMap, AHashSet};
use itertools::Itertools;

use crate::aocutil::Grid;

const DOWN: u8 = 118;
const RIGHT: u8 = 62;
const EMPTY: u8 = 46;

pub fn solve(input: Vec<Vec<u8>>) -> (usize, String) {
    let mut map = Map::from_grid(&input);

    let mut updates = usize::MAX;
    let mut steps = 0;
    while updates > 0 {
        (updates, map) = map.step();
        // dbg!(updates);
        steps += 1;
    }

    (steps, String::from("Merry Christmas!"))
}

#[derive(Clone)]
struct Map {
    rights: AHashSet<(usize, usize)>,
    downs: AHashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Map {
    fn step(&self) -> (usize, Map) {
        let mut updated = 0;

        let mut occupied: AHashSet<_> = self.rights.union(&self.downs).collect();
        let mut new_map = self.clone();

        for (x, y) in &self.rights {
            let new = ((*x + 1) % self.width, *y);
            if !occupied.contains(&new) {
                updated += 1;
                new_map.rights.remove(&(*x, *y));
                new_map.rights.insert(new);
            }
        }

        occupied = new_map.rights.union(&self.downs).collect();

        for (x, y) in &self.downs {
            let new = (*x, (*y + 1) % self.height);
            if !occupied.contains(&new) {
                updated += 1;
                new_map.downs.remove(&(*x, *y));
                new_map.downs.insert(new);
            }
        }

        (updated, new_map)
    }

    fn print(&self) {
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.rights.contains(&(x, y)) {
                    grid[y][x] = '>';
                } else if self.downs.contains(&(x, y)) {
                    grid[y][x] = 'v';
                }
            }
        }

        grid.show_display();
    }

    fn from_grid(grid: &Vec<Vec<u8>>) -> Map {
        let mut rights: AHashSet<(usize, usize)> = AHashSet::new();
        let mut downs = AHashSet::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match *cell {
                    RIGHT => rights.insert((x, y)),
                    DOWN => downs.insert((x, y)),
                    _ => false,
                };
            }
        }

        Map {
            rights,
            downs,
            width: grid.width(),
            height: grid.height(),
        }
    }
}

// fn step(grid: &mut Vec<Vec<u8>>) {
//     let mut buffer_a = grid[0].clone();
//     for row in 1..grid.height() - 1 {
//         let mut buffer_b = grid[row + 1].clone();

//         for col in 0..grid.width() - 1 {
//             let this = grid[row][col];
//             if this == EMPTY {
//                 continue;
//             }

//             let right = grid[row][col + 1];
//             let below = grid[row][col + 1];

//             if this == RIGHT && right == EMPTY {
//                 buffer_a[col] = EMPTY;
//                 buffer_a[col + 1] = this;
//             } else if this == DOWN && below == EMPTY {
//                 buffer_a[col] = EMPTY;
//                 buffer_b[col] = this;
//             }
//         }
//         grid[row] = buffer_a;
//         buffer_a = buffer_b;
//     }
// }
