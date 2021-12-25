use itertools::Itertools;

pub fn solve(lines: Vec<String>) -> (u64, u64) {
    let instructions = parse_instructions(lines);
    // let mut monad = CPU::new(ve c![], instructions);
    // let mut monad = CPU::new(generate_inputs(solver().unwrap()).unwrap(), instructions);
    // monad.run();

    solver();
    // for (i, num) in (333333333333..99999999999999).enumerate() {
    //     if let Some(inputs) = generate_inputs(333333333333) {
    //         monad.reset();
    //         monad.set_inputs(inputs);
    //         let trial = monad.run();
    //         if trial == 0 {
    //             println!("{} {}", num, trial);
    //         }
    //     }
    // }

    (0, 0)
}

fn get_w5(W1: i32, W2: i32, W3: i32, W4: i32) -> Option<i32> {
    let res = ((((((((W1 + 14) * 26) + (W2 + 6)) * 26) + (W3 + 6)) * 26) + (W4 + 13)) % 26) - 12;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn get_w7(W1: i32, W2: i32, W3: i32, W6: i32) -> Option<i32> {
    let res = ((((((((W1 + 14) * 26) + (W2 + 6)) * 26) + (W3 + 6)) * 26) + (W6 + 8)) % 26) - 15;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn get_w10(W1: i32, W2: i32, W3: i32, W8: i32, W9: i32) -> Option<i32> {
    let res = ((((((((((W1 + 14) * 26) + (W2 + 6)) * 26) + (W3 + 6)) * 26) + (W8 + 10)) * 26)
        + (W9 + 8))
        % 26)
        - 13;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn get_w11(W1: i32, W2: i32, W3: i32, W8: i32) -> Option<i32> {
    let res = ((((((((W1 + 14) * 26) + (W2 + 6)) * 26) + (W3 + 6)) * 26) + (W8 + 10)) % 26) - 13;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}
fn get_w12(W1: i32, W2: i32, W3: i32) -> Option<i32> {
    let res = ((((((W1 + 14) * 26) + (W2 + 6)) * 26) + (W3 + 6)) % 26) - 14;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}
fn get_w13(W1: i32, W2: i32) -> Option<i32> {
    let res = ((((W1 + 14) * 26) + (W2 + 6)) % 26) - 2;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}
fn get_w14(W1: i32) -> Option<i32> {
    let res = ((W1 + 14) % 26) - 9;
    if (1..10).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn solver() -> Option<i64> {
    // ((((((W1 + 14) * 26) + (W2 + 6)) * 26) + (W3 + 6)) * 26) + (W4 + 13) % 26

    for W1 in (1..10).rev() {
        for W2 in (1..10).rev() {
            for W3 in (1..10).rev() {
                for W4 in (1..10).rev() {
                    if let Some(W5) = get_w5(W1, W2, W3, W4) {
                        for W6 in (1..10).rev() {
                            if let Some(W7) = get_w7(W1, W2, W3, W5) {
                                for W8 in (1..10).rev() {
                                    for W9 in (1..10).rev() {
                                        if let Some(W10) = get_w10(W1, W2, W3, W8, W9) {
                                            if let Some(W11) = get_w11(W1, W2, W3, W8) {
                                                if let Some(W12) = get_w12(W1, W2, W3) {
                                                    if let Some(W13) = get_w13(W1, W2) {
                                                        if let Some(W14) = get_w14(W1) {
                                                            return Some(
                                                                format!(
                                                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                    W1,
                                                                    W2,
                                                                    W3,
                                                                    W4,
                                                                    W5,
                                                                    W6,
                                                                    W7,
                                                                    W8,
                                                                    W9,
                                                                    W10,
                                                                    W11,
                                                                    W12,
                                                                    W13,
                                                                    W14
                                                                )
                                                                .parse()
                                                                .unwrap(),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

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

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(usize),
    Value(i64),
}

type Opcode = fn(&mut CPU, Operand, Option<Operand>);
struct CPU {
    pc: usize,
    ic: usize,
    reg: [i64; 4],
    inputs: Vec<i64>,
    instructions: Vec<(Opcode, (Operand, Option<Operand>))>,
}

impl CPU {
    fn new(inputs: Vec<i64>, instructions: Vec<(Opcode, (Operand, Option<Operand>))>) -> CPU {
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

    fn run(&mut self) -> i64 {
        while self.pc < self.instructions.len() {
            let (opcode, (a, b)) = self.instructions.get(self.pc).unwrap();

            if b.is_none() {
                println!("{}", self.inputs[self.ic]);
                self.print_registers();
            }
            opcode(self, *a, b.clone());
            self.pc += 1;
        }
        // println!("{}", self.inputs[self.ic]);
        self.print_registers();

        self.reg[3]
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
            "W: {}\nX: {}\nY: {}\nZ: {}\n",
            self.reg[0], self.reg[1], self.reg[2], self.reg[3]
        )
    }
}

fn parse_instructions(lines: Vec<String>) -> Vec<(Opcode, (Operand, Option<Operand>))> {
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

        instructions.push((opcode, operands));
    }

    instructions
}
