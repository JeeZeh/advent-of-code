use std::str::FromStr;

use itertools::Itertools;

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn solve(input: String) -> (String, String) {
    let parts = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = parts.1.lines().map(Instruction::from).collect_vec();
    let stacks: Vec<Vec<char>> = parse_stacks(parts.0);

    let mut part_one_stacks = crane_go_brrr(&stacks, &instructions, true);
    let mut part_two_stacks = crane_go_brrr(&stacks, &instructions, false);

    (
        part_one_stacks
            .iter_mut()
            .map(|s| s.last().unwrap())
            .join(""),
        part_two_stacks
            .iter_mut()
            .map(|s| s.last().unwrap())
            .join(""),
    )
}

fn crane_go_brrr(
    stacks: &Vec<Vec<char>>,
    instructions: &Vec<Instruction>,
    lifo: bool,
) -> Vec<Vec<char>> {
    let mut new_stacks = stacks.clone();
    for inst in instructions {
        let bottom_of_stack = new_stacks[inst.from].len() - inst.amount;
        let to_move = new_stacks[inst.from].drain(bottom_of_stack..);
        // Need to consume the iterator to drop the reference to new_stacks before it's used again mutably
        let mut ordered_move = if lifo {
            to_move.rev().collect_vec()
        } else {
            to_move.collect_vec()
        };
        new_stacks[inst.to].append(&mut ordered_move);
    }
    new_stacks
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let stack_width = 4; // Space between each stack's contents

    let mut stack_iter = input.lines().rev();
    let stack_count: u32 = stack_iter
        .next()
        .unwrap()
        .chars()
        .rev()
        .nth(1)
        .unwrap()
        .to_digit(10)
        .expect("Could not parse last digit in crate base as digit");

    let mut stacks = vec![Vec::new(); stack_count as usize];
    for row in stack_iter {
        for (stack, idx) in (1..(stack_count * stack_width) + 1)
            .step_by(stack_width as usize)
            .enumerate()
        {
            let crate_char = row
                .chars()
                .nth(idx as usize)
                .expect("Error extracting crate char");
            if crate_char != ' ' {
                stacks[stack].push(crate_char);
            }
        }
    }

    stacks
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut parts = s.split(" ");
        let amount = parts.nth(1).unwrap().parse().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1; // 1-index
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        Instruction { amount, from, to }
    }
}
