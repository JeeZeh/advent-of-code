use itertools::Itertools;

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

pub fn solve(input: String) -> (i32, usize) {
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
            .skip(start)
            .step_by(step)
            .map(|(i, c)| dbg!(i as i32) * dbg!(c))
            .sum::<i32>(),
        0,
    )
}
