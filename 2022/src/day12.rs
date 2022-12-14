use std::collections::VecDeque;

use crate::aocutil::Grid;

pub fn solve(mut grid: Vec<Vec<char>>) -> (usize, usize) {
    let start = grid
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, c)| **c == 'S')
        .map(|(i, _)| (i % grid[0].len(), i.div_floor(grid[0].len())))
        .unwrap();
    let end = grid
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, c)| **c == 'E')
        .map(|(i, _)| (i % grid[0].len(), i.div_floor(grid[0].len())))
        .unwrap();

    grid[start.1][start.0] = 'a';
    grid[end.1][end.0] = 'z';

    let shortest_from_s = bfs(
        &grid,
        start,
        end,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
    )
    .unwrap();

    let mut shortest_from_any_lowest = usize::MAX;
    let mut shared_visited = vec![vec![false; grid[0].len()]; grid.len()];
    for start_attempt in grid
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, c)| **c == 'a')
        .map(|(i, _)| (i % grid[0].len(), i.div_floor(grid[0].len())))
    {
        if let Some(dist) = bfs(&grid, start_attempt, end, &mut shared_visited) {
            shortest_from_any_lowest = shortest_from_any_lowest.min(dist);
            // Reset visited if we found at least one path from this starting position
            // Might be possible to speed this up by flood-filling all the 'locked' areas first
            shared_visited = vec![vec![false; grid[0].len()]; grid.len()]
        }
    }

    (shortest_from_s, shortest_from_any_lowest)
}

fn bfs(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    goal: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
) -> Option<usize> {
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    if visited[start.1][start.0] {
        return None;
    } else {
        visited[start.1][start.0] = true;
    }

    while !queue.is_empty() {
        let (current, distance) = queue.pop_front().unwrap();
        if current == goal {
            return Some(distance);
        }
        let current_char = grid[current.1][current.0];
        for (x, y) in neighbours(current.0, current.1) {
            if let Some(new) = grid.getyx(y, x) {
                if !visited[y][x] && (*new as u8) <= (current_char as u8 + 1) {
                    visited[y][x] = true;
                    queue.push_back(((x, y), distance + 1));
                }
            }
        }
    }

    None
}

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (x + 1, y),
        (x, y + 1),
        ((x as i32 - 1) as usize, y),
        (x, (y as i32 - 1) as usize),
    ]
    .into_iter()
}
