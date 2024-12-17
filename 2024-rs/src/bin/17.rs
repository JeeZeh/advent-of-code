#![feature(int_roundings)]
use std::{collections::VecDeque, ops::BitXor};

use itertools::Itertools;

advent_of_code::solution!(17);

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

struct Cpu {
    registers: [i64; 3],
    inst_ptr: usize,
    output: VecDeque<i64>,
}

enum Op {
    Literal(i64),
    Combo(i64),
}

impl Op {
    fn resolve(&self, registers: &[i64; 3]) -> i64 {
        match self {
            Op::Literal(v) => *v,
            Op::Combo(v) => {
                if *v >= 0 && *v <= 3 {
                    return *v;
                } else if *v < 7 {
                    return registers[(v - 3) as usize];
                } else {
                    panic!("Unexpected operand: {}", v);
                }
            }
        }
    }
}

enum Inst {
    Adv(Op),
    Bxl(Op),
    Bst(Op),
    Jnz(Op),
    Bxc,
    Out(Op),
    Bdv(Op),
    Cdv(Op),
}

impl Inst {
    /// Reads the instruction and associated operand (if needed).
    fn read(prog: &[i64], ptr: usize) -> Option<Inst> {
        if ptr >= prog.len() {
            return None;
        }
        let opcode = prog[ptr];
        Some(match opcode {
            0 => Inst::Adv(Op::Combo(prog[ptr + 1])),
            1 => Inst::Bxl(Op::Literal(prog[ptr + 1])),
            2 => Inst::Bst(Op::Literal(prog[ptr + 1])),
            3 => Inst::Jnz(Op::Literal(prog[ptr + 1])),
            4 => Inst::Bxc,
            5 => Inst::Out(Op::Combo(prog[ptr + 1])),
            6 => Inst::Bdv(Op::Combo(prog[ptr + 1])),
            7 => Inst::Cdv(Op::Combo(prog[ptr + 1])),
            _ => panic!("Unrecognized opcode: {}", opcode),
        })
    }
}

impl Cpu {
    fn init(reg_a: i64, reg_b: i64, reg_c: i64) -> Self {
        Cpu {
            registers: [0, 0, 0],
            inst_ptr: 0,
            output: VecDeque::new(),
        }
    }

    fn drain_output(&mut self) -> String {
        self.output.drain(0..self.output.len()).join(",")
    }

    fn tick(&mut self, program: &[i64]) -> Option<usize> {
        if let Some(inst) = Inst::read(&program, self.inst_ptr) {
            let new_pointer: Option<usize> = match inst {
                Inst::Adv(op) => {
                    self.registers[REG_A] =
                        self.registers[REG_A].div_floor(op.resolve(&self.registers).pow(2));
                    None
                }
                Inst::Bxl(op) => {
                    self.registers[REG_B] =
                        self.registers[REG_B].bitxor(op.resolve(&self.registers));
                    None
                }
                Inst::Bst(op) => {
                    self.registers[REG_B] = op.resolve(&self.registers).rem_euclid(8);
                    None
                }
                Inst::Jnz(op) => {
                    if self.registers[REG_A] != 0 {
                        return Some(op.resolve(&self.registers) as usize);
                    }
                    None
                }
                Inst::Bxc => {
                    self.registers[REG_B] = self.registers[REG_B].bitxor(self.registers[REG_C]);
                    None
                }
                Inst::Out(op) => {
                    self.output
                        .push_front(op.resolve(&self.registers).rem_euclid(8));
                    None
                }
                Inst::Bdv(op) => {
                    self.registers[REG_B] =
                        self.registers[REG_A].div_floor(op.resolve(&self.registers).pow(2));
                    None
                }
                Inst::Cdv(op) => {
                    self.registers[REG_C] =
                        self.registers[REG_A].div_floor(op.resolve(&self.registers).pow(2));
                    None
                }
            };

            // Set new pointer location or increment by 2.
            self.inst_ptr = new_pointer.unwrap_or(self.inst_ptr + 2);
            return Some(self.inst_ptr);
        }
        None
    }
}

pub fn solve(input: &str) -> (Option<String>, Option<String>) {
    let (reg, prog) = input.split_once("\n\n").unwrap();
    let reg_values = reg
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<i64>().unwrap())
        .collect_vec();
    let mut cpu = Cpu::init(reg_values[0], reg_values[1], reg_values[2]);
    let program = prog
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|i| i.parse::<i64>().unwrap())
        .collect_vec();

    while let Some(ptr) = cpu.tick(&program) {
        continue;
    }

    (Some(cpu.drain_output()), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(String::from("4,6,3,5,6,3,5,2,1,0")), None));
    }
}
