use crate::aocutil::Grid;

pub fn solve(lines: Vec<String>) -> (u32, u32) {
    let mut grid = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += step(&mut grid);
    }

    let mut steps_to_0 = 100;

    while step(&mut grid) != 100 {
        steps_to_0 += 1;
    }

    (flash_count, steps_to_0 + 1)
}

pub fn step(grid: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashed = [[false; 10]; 10];
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if !flashed[y][x] {
                charge(x, y, grid, &mut flashed);
            }
        }
    }

    flashed.iter().flatten().filter(|b| **b).count() as u32
}

pub fn charge(x: usize, y: usize, grid: &mut Vec<Vec<u32>>, flashed: &mut [[bool; 10]; 10]) {
    if let Some(entry) = grid.getyx_mut(y, x) {
        if !flashed[y][x] {
            *entry += 1;
        } else {
            *entry = 0;
        }

        if entry == &10 {
            *entry = 0;
            flashed[y][x] = true;
            for (dx, dy) in neighbours(x, y) {
                charge(dx as usize, dy as usize, grid, flashed);
            }
        }
    }
}

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (i32, i32)> {
    [
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 + 1),
        (x as i32 - 1, y as i32),
        (x as i32, y as i32 - 1),
        (x as i32 + 1, y as i32 + 1),
        (x as i32 + 1, y as i32 - 1),
        (x as i32 - 1, y as i32 + 1),
        (x as i32 - 1, y as i32 - 1),
    ]
    .into_iter()
}
