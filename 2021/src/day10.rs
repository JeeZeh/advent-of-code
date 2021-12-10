use std::collections::VecDeque;

use itertools::{Either, Itertools};

struct Bracket {
    char: char,
    pair: char,
    error_score: u32,
    close_score: u64,
    is_open: bool,
}

pub fn solve(lines: Vec<String>) -> (u32, u64) {
    let (errors, mut completions): (Vec<u32>, Vec<u64>) =
        lines.iter().map(get_scores).partition_map(|(e, c)| {
            if e != 0 {
                Either::Left(e)
            } else {
                Either::Right(c)
            }
        });

    let middle = (completions.len()) / 2;

    (
        errors.iter().sum(),
        *completions.select_nth_unstable(middle).1,
    )
}

fn as_brackets(line: &str) -> impl Iterator<Item = Bracket> + '_ {
    line.chars().map(|char| {
        let (error_score, close_score, is_open, pair) = match char {
            '(' => (3, 1, true, ')'),
            ')' => (3, 1, false, '('),
            '[' => (57, 2, true, ']'),
            ']' => (57, 2, false, '['),
            '{' => (1197, 3, true, '}'),
            '}' => (1197, 3, false, '{'),
            '<' => (25137, 4, true, '>'),
            '>' => (25137, 4, false, '<'),
            _ => panic!("Unexpected char"),
        };
        Bracket {
            char,
            error_score,
            close_score,
            is_open,
            pair,
        }
    })
}

fn get_scores(line: &String) -> (u32, u64) {
    let mut stack = VecDeque::new();
    let mut completion = 0;
    for bracket in as_brackets(line) {
        if bracket.is_open {
            stack.push_back(bracket);
        } else {
            if let Some(open) = stack.pop_back() {
                if open.pair != bracket.char {
                    return (bracket.error_score, 0);
                }
            }
        }
    }

    while let Some(open) = stack.pop_back() {
        completion = (completion * 5) + open.close_score;
    }

    (0, completion)
}
