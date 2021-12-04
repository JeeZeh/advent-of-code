use std::collections::{HashMap, HashSet};

struct Board {
    grid: Vec<Vec<bool>>,
    locations: HashMap<u32, (usize, usize)>,
}

impl Board {
    fn parse_board(board_str: &str) -> Self {
        let lines: Vec<Vec<u32>> = board_str
            .split("\r\n")
            .map(|line| {
                line.split_whitespace()
                    .map(|c| c.parse().unwrap())
                    .collect()
            })
            .collect();

        let grid = vec![vec![false; lines[0].len()]; lines.len()];
        let mut locations: HashMap<u32, (usize, usize)> = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, number) in line.iter().enumerate() {
                locations.insert(*number, (x, y));
            }
        }

        Board { grid, locations }
    }

    fn mark_number(&mut self, number: u32) -> bool {
        let loc = self.locations.get(&number);
        if loc.is_some() {
            let (x, y) = loc.unwrap();
            self.grid[*y][*x] = true;
            return self.grid[*y].iter().all(|v| *v)
                || self.grid.iter().map(|row| row[*x]).all(|v| v);
        }

        return false;
    }

    fn score(&self, last_call: u32) -> u32 {
        let sum_unmarked: u32 = self
            .locations
            .iter()
            .filter(|(_, (x, y))| !self.grid[*y][*x])
            .map(|(n, _)| n)
            .sum();

        sum_unmarked * last_call
    }
}

pub fn solve(input: String) -> (u32, u32) {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let numbers_order: Vec<u32> = parts[0].split(",").map(|c| c.parse().unwrap()).collect();
    let mut boards: Vec<Board> = parts[1..].iter().map(|s| Board::parse_board(*s)).collect();

    play_bingo(&mut boards, &numbers_order)
}

fn play_bingo(boards: &mut Vec<Board>, numbers: &Vec<u32>) -> (u32, u32) {
    let mut finished: HashSet<usize> = HashSet::new();
    let mut winning_boards: Vec<u32> = Vec::new();
    let total_boards = boards.len();

    for number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if finished.contains(&i) {
                continue;
            }
            if board.mark_number(*number) {
                finished.insert(i);
                winning_boards.push(board.score(*number));
            }
        }
        if winning_boards.len() == total_boards {
            break;
        }
    }

    (
        *winning_boards.first().unwrap(),
        *winning_boards.last().unwrap(),
    )
}
