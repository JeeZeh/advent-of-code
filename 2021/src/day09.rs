pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let grid: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let max_y: usize = grid.len();
    let max_x: usize = grid[0].len();
    let mut seen = vec![vec![false; max_x]; max_y];

    let mut low_points: Vec<(usize, usize)> = Vec::new();

    let mut basins: Vec<i32> = Vec::new();
    for y in 0..max_y {
        for x in 0..max_x {
            if is_local_minimum(x, y, &grid) {
                low_points.push((x, y));
                basins.push(floodfill(x, y, &grid, &mut seen));
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));

    (
        low_points
            .iter()
            .map(|(x, y)| (grid[*y][*x] as usize + 1) as usize)
            .sum(),
        (basins[0] * basins[1] * basins[2]) as usize,
    )
}

fn floodfill(x: usize, y: usize, grid: &[Vec<u8>], seen: &mut [Vec<bool>]) -> i32 {
    if grid.get(y).and_then(|f| f.get(x)).unwrap_or_else(|| &9) == &9 || seen[y][x] {
        return 0;
    }

    seen[y][x] = true;
    let mut size = 1;

    for (check_x, check_y) in check_around(x, y) {
        size += floodfill(check_x as usize, check_y as usize, grid, seen);
    }

    size
}

fn is_local_minimum(x: usize, y: usize, grid: &[Vec<u8>]) -> bool {
    for (check_x, check_y) in check_around(x, y) {
        let to_compare = grid
            .get(check_y as usize)
            .and_then(|f| f.get(check_x as usize))
            .unwrap_or_else(|| &9);

        if to_compare <= &grid[y][x] {
            return false;
        }
    }
    return true;
}

fn check_around(x: usize, y: usize) -> impl Iterator<Item = (i32, i32)> {
    [
        (x as i32 + 1, y as i32),
        (x as i32, y as i32 + 1),
        (x as i32 - 1, y as i32),
        (x as i32, y as i32 - 1),
    ]
    .into_iter()
}
