use itertools::Itertools;

use crate::aocutil::Grid;

pub fn solve(lines: String) -> (usize, usize) {
    let (lookup, mut image) = parse_input(lines);

    for iter in 0..2 {
        image = convolve(&lookup, &image, iter);
    }

    let part_one = image.iter().flatten().filter(|b| **b).count();

    for iter in 2..50 {
        image = convolve(&lookup, &image, iter);
    }

    let part_two = image.iter().flatten().filter(|b| **b).count();

    // image.show_map(|b| if *b { '#' } else { '.' });

    (part_one, part_two)
}

fn convolve(lookup: &Vec<bool>, image: &Vec<Vec<bool>>, iter: usize) -> Vec<Vec<bool>> {
    let (width, height) = (image.width(), image.height());

    // Grow the grid by 1 pixel in each direction before convolving
    let mut new = vec![vec![iter > 0 && iter % 2 != 0; image[0].len() + 2]; image.len() + 2];

    for y in 0..height + 2 {
        for x in 0..width + 2 {
            new[y][x] = lookup[get_idx((x as i32 - 1, y as i32 - 1), &image, iter % 2 == 1)];
        }
    }

    new
}

fn get_idx(center: (i32, i32), image: &Vec<Vec<bool>>, out_of_bounds: bool) -> usize {
    // Out of bounds means how we should interpret the infinite space beyond the grid
    // This is a little hardcoded, since it just so happens that all the pixels outside
    // the main image alternate between on and off each step
    let (x, y) = center;
    let (top_row, middle_row, bottom_row) = ((y - 1) as usize, y as usize, (y + 1) as usize);
    let (left_col, middle_col, right_col) = ((x - 1) as usize, x as usize, (x + 1) as usize);
    let top = [
        image
            .getyx(top_row, left_col)
            .unwrap_or_else(|| &out_of_bounds),
        image
            .getyx(top_row, middle_col)
            .unwrap_or_else(|| &out_of_bounds),
        image
            .getyx(top_row, right_col)
            .unwrap_or_else(|| &out_of_bounds),
    ];
    let mid = [
        image
            .getyx(middle_row, left_col)
            .unwrap_or_else(|| &out_of_bounds),
        image
            .getyx(middle_row, middle_col)
            .unwrap_or_else(|| &out_of_bounds),
        image
            .getyx(middle_row, right_col)
            .unwrap_or_else(|| &out_of_bounds),
    ];
    let bot = [
        image
            .getyx(bottom_row, left_col)
            .unwrap_or_else(|| &out_of_bounds),
        image
            .getyx(bottom_row, middle_col)
            .unwrap_or_else(|| &out_of_bounds),
        image
            .getyx(bottom_row, right_col)
            .unwrap_or_else(|| &out_of_bounds),
    ];
    [top, mid, bot]
        .concat()
        .iter()
        .fold(0, |acc, &b| (acc << 1) | *b as usize)
}

fn parse_input(lines: String) -> (Vec<bool>, Vec<Vec<bool>>) {
    let (a, b) = lines.split_once("\n\n").unwrap();
    let lookup = a
        .chars()
        .map(|c| if c == '#' { true } else { false })
        .collect_vec();

    let image_lines = b.lines().collect_vec();

    let mut image = vec![vec![false; image_lines[0].len()]; image_lines.len()];

    for (y, row) in image_lines.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                image[y][x] = true;
            }
        }
    }

    (lookup, image)
}
