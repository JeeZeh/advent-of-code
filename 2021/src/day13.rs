use std::str::Lines;

use itertools::Itertools;
use ndarray::{s, Array2, Axis};

use crate::aocutil::Grid;

#[derive(PartialEq)]
enum FoldAxis {
    Y,
    X,
}

struct Fold {
    axis: FoldAxis,
    on: usize,
}

pub fn solve(lines: String) {
    let mut parts = lines.split("\n\n");
    let mut grid: Array2<u8> = generate_grid(parts.next().unwrap().lines());
    let folds = parts.next().unwrap().lines().map(parse_fold).collect_vec();

    grid = do_fold(&mut grid, &folds[0]);
    println!("{}", grid.iter().map(|v| 1.min(*v) as u32).sum::<u32>());

    for fold in &folds[1..] {
        grid = do_fold(&mut grid, fold);
    }

    println!(
        "{}",
        grid.rows()
            .into_iter()
            .map(|row| row.iter().map(|v| if *v >= 1 { "#" } else { "." }).join(""))
            .join("\n")
    );
}

fn do_fold(grid: &mut Array2<u8>, fold: &Fold) -> Array2<u8> {
    use FoldAxis::*;

    let retain_slice = if fold.axis == X {
        s![0..grid.shape()[0], 0..fold.on,]
    } else {
        s![0..fold.on, 0..grid.shape()[1]]
    };

    let to_fold_slice = if fold.axis == X {
        s![0..grid.shape()[0], fold.on + 1..grid.shape()[1]; -1]
    } else {
        s![fold.on + 1..grid.shape()[0]; -1, 0..grid.shape()[1]]
    };

    let mirrored = grid.slice(to_fold_slice);
    let mut new_grid = grid.slice(retain_slice).to_owned();

    for y in 0..mirrored.shape()[0] {
        for x in 0..mirrored.shape()[1] {
            new_grid[[y, x]] += mirrored[[y, x]];
        }
    }

    new_grid
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

fn generate_grid(lines: Lines) -> Array2<u8> {
    let coords: Vec<(usize, usize)> = lines.map(parse_coord).collect();
    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    let mut grid = Array2::<u8>::zeros((max_y + 1, max_x + 1));

    for (x, y) in coords {
        grid[[y, x]] = 1;
    }

    grid
}
