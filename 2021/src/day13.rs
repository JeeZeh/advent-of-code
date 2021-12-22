use ahash::AHashSet;
use itertools::Itertools;

#[derive(PartialEq)]
enum FoldAxis {
    Y,
    X,
}

pub fn solve(lines: String) -> (usize, String) {
    let mut parts = lines.split("\n\n");
    let mut grid: AHashSet<(u16, u16)> = parts.next().unwrap().lines().map(parse_coord).collect();
    let folds = parts.next().unwrap().lines().map(parse_fold).collect_vec();

    grid = fold_points(&grid, &folds[0..1]);
    let part_one = grid.iter().count();

    grid = fold_points(&grid, &folds[1..]);

    (part_one, display_grid(&grid))
}

fn display_grid(grid: &AHashSet<(u16, u16)>) -> String {
    let max_x = grid.iter().map(|(x, _)| *x).max().unwrap() as usize;
    let max_y = grid.iter().map(|(_, y)| *y).max().unwrap() as usize;

    format!(
        "\n{}",
        (0..=max_y)
            .map(|y| (0..=max_x)
                .map(|x| if grid.contains(&(x as u16, y as u16)) {
                    '#'
                } else {
                    ' '
                })
                .join(""))
            .join("\n")
    )
}

/**
 * Credit to @mgoszcz2 for implementing this... though the idea to process
 * all folds in one go was mine :)
 */
fn fold_points(grid: &AHashSet<(u16, u16)>, folds: &[(FoldAxis, u16)]) -> AHashSet<(u16, u16)> {
    use FoldAxis::*;
    let mut next = AHashSet::new();
    for (mut x, mut y) in grid {
        for (axis, on) in folds {
            x = if x < *on || *axis == Y { x } else { on * 2 - x };
            y = if y < *on || *axis == X { y } else { on * 2 - y };
        }
        next.insert((x, y));
    }
    next
}

fn parse_fold(line: &str) -> (FoldAxis, u16) {
    let (axis, index) = line
        .split(' ')
        .last()
        .unwrap()
        .split('=')
        .collect_tuple()
        .unwrap();

    (
        if axis == "x" {
            FoldAxis::X
        } else {
            FoldAxis::Y
        },
        index.parse().unwrap(),
    )
}

fn parse_coord(line: &str) -> (u16, u16) {
    let mut parts = line.split(',');

    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}
