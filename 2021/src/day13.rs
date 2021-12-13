use std::str::Lines;

use itertools::Itertools;
use ndarray::{s, Array2, Axis};

use crate::aocutil::Grid;

#[derive(PartialEq, Debug)]
enum FoldAxis {
    Y,
    X,
}
#[derive(Debug)]
struct Fold {
    axis: FoldAxis,
    on: usize,
}

pub fn solve(lines: String) -> (u32, String) {
    let mut parts = lines.split("\n\n");
    let mut grid: Array2<u8> = generate_grid(parts.next().unwrap().lines());
    let folds = parts.next().unwrap().lines().map(parse_fold).collect_vec();

    grid = do_fold(&mut grid, &folds[0]);
    let part_one = grid.iter().map(|v| 1.min(*v) as u32).sum::<u32>();

    for fold in &folds[1..] {
        grid = do_fold(&mut grid, fold);
    }

    (
        part_one,
        format!(
            "\n{}",
            grid.rows()
                .into_iter()
                .map(|row| row.iter().map(|v| if *v >= 1 { "#" } else { "." }).join(""))
                .join("\n")
        ),
    )
}

/**
 * Folds by creating 2 slices, the retained slice (left or top side), and the fold slice of the grid.
 * To perform the fold, the fold slice is flipped and then added to the retain.
 * The addition of the two slices creates a new, smaller array ready for another fold.
 */
fn do_fold(grid: &mut Array2<u8>, fold: &Fold) -> Array2<u8> {
    let fold_slice;
    let retain_slice;

    if fold.axis == FoldAxis::X {
        retain_slice = s![.., 0..fold.on];
        fold_slice = s![.., fold.on + 1..; -1];
    } else {
        retain_slice = s![0..fold.on, ..];
        fold_slice = s![fold.on + 1..; -1, ..];
    }

    grid.slice(retain_slice).to_owned() + grid.slice(fold_slice)
}

fn parse_fold(line: &str) -> Fold {
    let (axis, index) = line
        .split(' ')
        .last()
        .unwrap()
        .split('=')
        .collect_tuple()
        .unwrap();

    Fold {
        axis: if axis == "x" {
            FoldAxis::X
        } else {
            FoldAxis::Y
        },
        on: index.parse().unwrap(),
    }
}

fn parse_coord(line: &str) -> (usize, usize) {
    let mut parts = line.split(',');

    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

/**
 * Grid is a 2D array of 0s, stored as u8.
 * You might think bools would be faster, but this is actually
 * slower when performing the fold (a | b) vs. (a + b). Seems there's
 * some magic going on with addition.
 */
fn generate_grid(lines: Lines) -> Array2<u8> {
    let coords: Vec<(usize, usize)> = lines.map(parse_coord).collect();
    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    // Make sure the array is odd-sized
    let mut grid = Array2::<u8>::zeros((
        if max_y % 2 == 1 { max_y + 2 } else { max_y + 1 },
        if max_x % 2 == 1 { max_x + 2 } else { max_x + 1 },
    ));

    for (x, y) in coords {
        grid[[y, x]] = 1;
    }

    grid
}
