use itertools::Itertools;

use crate::aocutil::Grid;

enum Instruction {
    Add(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s {
            x if x.starts_with("addx") => {
                Instruction::Add(x.split_once(" ").unwrap().1.parse().unwrap())
            }
            "noop" => Instruction::Noop,
            _ => panic!("Unexpected instruction"),
        }
    }
}

impl Instruction {
    // Applies an instruction, potentially modifying the passed register,
    // and returning the number of cycles consumed by the instruction.
    fn apply(&self, reg: &mut i32) -> Vec<i32> {
        match self {
            Instruction::Add(v) => {
                let cycle_values = vec![*reg, *reg];
                *reg += v;
                return cycle_values;
            }
            Instruction::Noop => vec![*reg],
        }
    }
}

pub fn solve(input: String) -> (i32, String) {
    let mut reg_x: i32 = 1;
    let instructions = input.lines().map(Instruction::from).collect_vec();

    let cycles = instructions
        .iter()
        .map(|inst| inst.apply(&mut reg_x))
        .flatten()
        .collect_vec();

    let start = 20;
    let step = 40;

    (
        cycles
            .iter()
            .enumerate()
            .skip(start - 1)
            .step_by(step)
            .map(|(i, c)| (i + 1) as i32 * c)
            .sum::<i32>(),
        format!("\n{}", draw_cycles(&cycles)),
    )
}

fn draw_cycles(cycles: &[i32]) -> String {
    let width = 40;
    let mut rows: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    for (c, pos) in cycles.iter().enumerate() {
        if pos - 1 <= (c % width) as i32 && (c % width) as i32 <= pos + 1 {
            *rows
                .getyx_mut((c).div_floor(width) as usize, c % width)
                .unwrap() = '#';
        }
    }

    rows.iter().map(|chars| chars.iter().join("")).join("\n")
}
