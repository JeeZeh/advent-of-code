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
    output: Vec<i64>,
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
                    return registers[(v - 4) as usize];
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
            2 => Inst::Bst(Op::Combo(prog[ptr + 1])),
            3 => Inst::Jnz(Op::Literal(prog[ptr + 1])),
            4 => Inst::Bxc,
            5 => Inst::Out(Op::Combo(prog[ptr + 1])),
            6 => Inst::Bdv(Op::Combo(prog[ptr + 1])),
            7 => Inst::Cdv(Op::Combo(prog[ptr + 1])),
            _ => panic!("Unrecognized opcode: {}", opcode),
        })
    }

    fn execute(&self, registers: &mut [i64; 3]) -> Option<i64> {
        match self {
            Inst::Adv(op) => {
                registers[REG_A] =
                    registers[REG_A].div_floor(2_i64.pow(op.resolve(&registers) as u32));
            }
            Inst::Bxl(op) => {
                registers[REG_B] = registers[REG_B].bitxor(op.resolve(&registers));
            }
            Inst::Bst(op) => {
                registers[REG_B] = op.resolve(&registers).rem_euclid(8);
            }
            Inst::Bxc => {
                registers[REG_B] = registers[REG_B].bitxor(registers[REG_C]);
            }
            Inst::Out(op) => {
                return Some(op.resolve(&registers).rem_euclid(8));
            }
            Inst::Bdv(op) => {
                registers[REG_B] =
                    registers[REG_A].div_floor(2_i64.pow(op.resolve(&registers) as u32));
            }
            Inst::Cdv(op) => {
                registers[REG_C] =
                    registers[REG_A].div_floor(2_i64.pow(op.resolve(&registers) as u32));
            }
            // Don't do anything with jumps just yet.
            Inst::Jnz(_) => (),
        };
        None
    }
}

impl Cpu {
    fn init(reg_a: i64, reg_b: i64, reg_c: i64) -> Self {
        Cpu {
            registers: [reg_a, reg_b, reg_c],
            inst_ptr: 0,
            output: Vec::new(),
        }
    }

    fn drain_output(&mut self) -> String {
        self.output.drain(0..self.output.len()).join(",")
    }

    fn run_till_completion(&mut self, program: &[i64]) {
        while self.tick(program).is_some() {
            continue;
        }
    }

    fn tick(&mut self, program: &[i64]) -> Option<usize> {
        if let Some(inst) = Inst::read(&program, self.inst_ptr) {
            // Execute instruction, store output if any.
            if let Some(output) = inst.execute(&mut self.registers) {
                self.output.push(output);
            }

            // Advance pointer.
            match inst {
                Inst::Jnz(op) => {
                    if self.registers[REG_A] == 0 {
                        self.inst_ptr += 2;
                    } else {
                        self.inst_ptr = op.resolve(&self.registers) as usize;
                    }
                }
                _ => self.inst_ptr += 2,
            }

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

    cpu.run_till_completion(&program);

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

    /// If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn test_one() {
        let mut cpu = Cpu::init(0, 0, 9);

        cpu.run_till_completion(&vec![2, 6]);
        assert_eq!(cpu.registers[REG_B], 1);
    }

    /// If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn test_two() {
        let mut cpu = Cpu::init(10, 0, 0);
        cpu.run_till_completion(&vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(cpu.drain_output(), "0,1,2");
    }

    /// If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn test_three() {
        let mut cpu = Cpu::init(2024, 0, 0);
        cpu.run_till_completion(&vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(cpu.drain_output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(cpu.registers[REG_A], 0);
    }

    /// If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn test_four() {
        let mut cpu = Cpu::init(0, 29, 9);
        cpu.run_till_completion(&vec![1, 7]);
        assert_eq!(cpu.registers[REG_B], 26);
    }

    /// If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn test_five() {
        let mut cpu = Cpu::init(0, 2024, 43690);
        cpu.run_till_completion(&vec![4, 0]);
        assert_eq!(cpu.registers[REG_B], 44354);
    }
}
