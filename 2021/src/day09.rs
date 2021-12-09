use std::collections::HashSet;

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let max_y = grid.len();
    let max_x = grid[0].len();
    let bounds = (max_x as i32 - 1, max_y as i32 - 1);

    let mut low_points: Vec<(usize, usize)> = Vec::new();

    let mut basins: Vec<i32> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..max_y {
        for x in 0..max_x {
            if is_local_minimum(x, y, &grid, bounds) {
                low_points.push((x, y));
                basins.push(floodfill(x, y, &grid, &mut seen, bounds));
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));

    let mut prod = 1;
    basins[..3].iter().for_each(|d| prod *= d);

    (
        low_points
            .iter()
            .map(|(x, y)| (grid[*y][*x] as usize + 1) as usize)
            .sum(),
        prod as usize,
    )
}

fn floodfill(
    x: usize,
    y: usize,
    grid: &[Vec<u8>],
    seen: &mut HashSet<(usize, usize)>,
    bounds: (i32, i32),
) -> i32 {
    if grid[y][x] == 9 || seen.contains(&(x, y)) {
        return 0;
    }

    seen.insert((x, y));
    let mut size = 1;

    for (check_x, check_y) in check_around(x, y) {
        if check_x < 0 || check_x > bounds.0 || check_y < 0 || check_y > bounds.1 {
            continue;
        }
        size += floodfill(check_x as usize, check_y as usize, grid, seen, bounds);
    }

    size
}

fn is_local_minimum(x: usize, y: usize, grid: &[Vec<u8>], bounds: (i32, i32)) -> bool {
    let centre = grid[y][x];
    for (check_x, check_y) in check_around(x, y) {
        if check_x < 0 || check_x > bounds.0 || check_y < 0 || check_y > bounds.1 {
            continue;
        }
        let to_compare = grid[check_y as usize][check_x as usize];
        if to_compare <= centre {
            return false;
        }
    }
    return true;
}

fn check_around(x: usize, y: usize) -> [(i32, i32); 4] {
    [
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 + 1),
        (x as i32 - 1, y as i32),
        (x as i32, y as i32 - 1),
    ]
}
