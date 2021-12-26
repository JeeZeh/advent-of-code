use std::collections::VecDeque;

use itertools::Itertools;

// This solution is hardcoded from manually analysing
pub fn solve(lines: Vec<String>) -> (String, String) {
    let instructions = parse_instructions(lines);
    let smallest = solver(11111111111111, &instructions);
    let largest = solver(99999999999999, &instructions);

    let mut monad = CPU::new(largest.clone(), instructions);
    assert!(monad.run());

    monad.reset();
    monad.set_inputs(smallest.clone());
    assert!(monad.run());

    (
        String::from_iter(largest.iter().map(|v| format!("{}", v))),
        String::from_iter(smallest.iter().map(|v| format!("{}", v))),
    )
}

static BLOCK_LEN: usize = 18;
static DIV_IDX: usize = 4;
static ADD_X_IDX: usize = 5;
static ADD_Z_IDX: usize = 15;

// Great solution https://www.reddit.com/r/adventofcode/comments/rnejv5/2021_day_24_solutions/hpuu3e0/?context=3
// Though still not sure I *fully* understand it
fn solver(to_correct: i64, instructions: &Vec<Instruction>) -> Vec<i64> {
    let mut inputs = generate_inputs(to_correct).unwrap();
    let mut stack: VecDeque<(usize, i64)> = VecDeque::new();

    for curr_number_idx in 0..14 {
        let block_idx = curr_number_idx * BLOCK_LEN;
        let div = instructions[block_idx + DIV_IDX].get_literal().unwrap();
        let add_x = instructions[block_idx + ADD_X_IDX].get_literal().unwrap();
        let add_z = instructions[block_idx + ADD_Z_IDX].get_literal().unwrap();

        if div == 1 {
            stack.push_back((curr_number_idx, add_z))
        } else {
            let (prev_num_idx, add_z) = stack.pop_back().unwrap();

            // Make the X = W check pass
            inputs[curr_number_idx] = inputs[prev_num_idx] + add_x + add_z;

            if inputs[curr_number_idx] > 9 {
                // Offset the previous block to correct current block to be
                // within the range 1..=9
                inputs[prev_num_idx] -= inputs[curr_number_idx] - 9;
                inputs[curr_number_idx] = 9 // This will be corrected in the next step
            }
            if inputs[curr_number_idx] < 1 {
                // Offset the previous block to correct current block to be
                // within the range 1..=9
                inputs[prev_num_idx] += 1 - inputs[curr_number_idx];
                inputs[curr_number_idx] = 1 // This will be corrected in the next step
            }
        }
    }

    inputs
}

// The rest of this is my implentation. See commit history for terrible
// hard-coded solution

fn generate_inputs(mut number: i64) -> Option<Vec<i64>> {
    let mut inputs: Vec<i64> = Vec::new();

    while inputs.len() < 14 {
        let next = number % 10;
        if next == 0 {
            return None;
        }
        inputs.push((number % 10) as i64);
        number /= 10;
    }

    inputs.reverse();
    Some(inputs)
}

struct Instruction(Opcode, (Operand, Option<Operand>));

impl Instruction {
    fn get_literal(&self) -> Option<i64> {
        let Instruction(_, (_, maybe_value)) = self;
        match maybe_value {
            Some(v) => v.get_value(),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(usize),
    Value(i64),
}

impl Operand {
    fn get_value(&self) -> Option<i64> {
        match self {
            &Operand::Register(_) => None,
            &Operand::Value(v) => Some(v),
        }
    }
}

type Opcode = fn(&mut CPU, Operand, Option<Operand>);
struct CPU {
    pc: usize,
    ic: usize,
    reg: [i64; 4],
    inputs: Vec<i64>,
    instructions: Vec<Instruction>,
}

impl CPU {
    fn new(inputs: Vec<i64>, instructions: Vec<Instruction>) -> CPU {
        CPU {
            pc: 0,
            ic: 0,
            reg: [0; 4],
            inputs,
            instructions,
        }
    }

    fn get_value(&self, a: Option<Operand>) -> i64 {
        match a {
            Some(Operand::Register(b)) => self.reg[b],
            Some(Operand::Value(v)) => v,
            _ => panic!("Cannot parse operand '{:?}'", &a),
        }
    }

    fn get_register(&mut self, a: Operand) -> &mut i64 {
        let register = match a {
            Operand::Register(r) => r,
            _ => panic!("Can only write to a register"),
        };

        self.reg.get_mut(register).unwrap()
    }

    fn input(&mut self, a: Operand, _: Option<Operand>) {
        let input = self.inputs[self.ic];
        *self.get_register(a) = input;
        self.ic += 1;
    }

    fn add(&mut self, a: Operand, b: Option<Operand>) {
        let value = self.get_value(b);
        *self.get_register(a) += value;
        // dbg!(self.reg[1], value);
    }

    fn multiply(&mut self, a: Operand, b: Option<Operand>) {
        let value = self.get_value(b);
        *self.get_register(a) *= value;
    }

    fn divide(&mut self, a: Operand, b: Option<Operand>) {
        let value = self.get_value(b);
        // dbg!(value);
        *self.get_register(a) /= value;
    }

    fn modulo(&mut self, a: Operand, b: Option<Operand>) {
        let value = self.get_value(b);
        *self.get_register(a) %= value;
        // dbg!(self.reg[1]);
    }

    fn equal(&mut self, a: Operand, b: Option<Operand>) {
        let value = self.get_value(b);
        let reg = self.get_register(a);
        *reg = (*reg == value) as i64;
        // dbg!(self.reg[1]);
    }

    fn run(&mut self) -> bool {
        while self.pc < self.instructions.len() {
            let Instruction(opcode, (a, b)) = self.instructions.get(self.pc).unwrap();
            opcode(self, *a, b.clone());
            self.pc += 1;
        }
        // println!("{}", self.inputs[self.ic]);
        // self.print_registers();

        self.reg[3] == 0
    }

    fn reset(&mut self) {
        self.reg = [0; 4];
        self.pc = 0;
        self.ic = 0;
    }

    fn set_inputs(&mut self, inputs: Vec<i64>) {
        self.inputs = inputs;
    }

    fn print_registers(&self) {
        println!(
            "w: {}\nX: {}\nY: {}\nZ: {}\n",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3]
        )
    }
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        let (instr, inputs) = line.split_once(' ').unwrap();

        let parts: Vec<&str> = inputs.split(" ").collect_vec();

        let opcode = match instr {
            "inp" => CPU::input,
            "add" => CPU::add,
            "mul" => CPU::multiply,
            "div" => CPU::divide,
            "mod" => CPU::modulo,
            "eql" => CPU::equal,
            _ => panic!("Unexpected instruction: {}", instr),
        };

        let operands;
        if instr == "inp" {
            operands = (
                Operand::Register((parts[0].bytes().next().unwrap() as u8 - 119) as usize),
                None,
            );
        } else {
            let b_operand = parts[1].parse::<i64>();
            operands = (
                Operand::Register((parts[0].bytes().next().unwrap() as u8 - 119) as usize),
                Some(if b_operand.is_ok() {
                    Operand::Value(b_operand.unwrap())
                } else {
                    Operand::Register((parts[1].bytes().next().unwrap() as u8 - 119) as usize)
                }),
            )
        }

        instructions.push(Instruction(opcode, operands));
    }

    instructions
}
