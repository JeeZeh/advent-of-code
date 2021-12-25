use itertools::Itertools;

// This solution is hardcoded from manually analysing
pub fn solve(lines: Vec<String>) -> (i64, i64) {
    let instructions = parse_instructions(lines);
    let (smallest, largest) = solver();
    let mut monad = CPU::new(generate_inputs(largest).unwrap(), instructions);
    assert!(monad.run());

    monad.reset();
    monad.set_inputs(generate_inputs(smallest).unwrap());
    assert!(monad.run());

    (largest, smallest)
}

fn get_w5(w1: i32, w2: i32, w3: i32, w4: i32) -> Option<i32> {
    let res = ((((((((w1 + 14) * 26) + (w2 + 6)) * 26) + (w3 + 6)) * 26) + (w4 + 13)) % 26) - 12;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn get_w7(w1: i32, w2: i32, w3: i32, w6: i32) -> Option<i32> {
    let res = ((((((((w1 + 14) * 26) + (w2 + 6)) * 26) + (w3 + 6)) * 26) + (w6 + 8)) % 26) - 15;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn get_w10(w1: i32, w2: i32, w3: i32, w8: i32, w9: i32) -> Option<i32> {
    let res = ((((((((((w1 + 14) * 26) + (w2 + 6)) * 26) + (w3 + 6)) * 26) + (w8 + 10)) * 26)
        + (w9 + 8))
        % 26)
        - 13;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn get_w11(w1: i32, w2: i32, w3: i32, w8: i32) -> Option<i32> {
    let res = ((((((((w1 + 14) * 26) + (w2 + 6)) * 26) + (w3 + 6)) * 26) + (w8 + 10)) % 26) - 13;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}
fn get_w12(w1: i32, w2: i32, w3: i32) -> Option<i32> {
    let res = ((((((w1 + 14) * 26) + (w2 + 6)) * 26) + (w3 + 6)) % 26) - 14;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}
fn get_w13(w1: i32, w2: i32) -> Option<i32> {
    let res = ((((w1 + 14) * 26) + (w2 + 6)) % 26) - 2;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}
fn get_w14(w1: i32) -> Option<i32> {
    let res = ((w1 + 14) % 26) - 9;
    if ((1..10).rev()).contains(&res) {
        Some(res)
    } else {
        None
    }
}

fn solver() -> (i64, i64) {
    let mut model_numbers: Vec<i64> = Vec::new();

    for w1 in (1..10).rev() {
        for w2 in (1..10).rev() {
            for w3 in (1..10).rev() {
                for w4 in (1..10).rev() {
                    if let Some(w5) = get_w5(w1, w2, w3, w4) {
                        for w6 in (1..10).rev() {
                            if let Some(w7) = get_w7(w1, w2, w3, w6) {
                                for w8 in (1..10).rev() {
                                    for w9 in (1..10).rev() {
                                        if let Some(w10) = get_w10(w1, w2, w3, w8, w9) {
                                            if let Some(w11) = get_w11(w1, w2, w3, w8) {
                                                if let Some(w12) = get_w12(w1, w2, w3) {
                                                    if let Some(w13) = get_w13(w1, w2) {
                                                        if let Some(w14) = get_w14(w1) {
                                                            model_numbers.push(
                                                                format!(
                                                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                    w1,
                                                                    w2,
                                                                    w3,
                                                                    w4,
                                                                    w5,
                                                                    w6,
                                                                    w7,
                                                                    w8,
                                                                    w9,
                                                                    w10,
                                                                    w11,
                                                                    w12,
                                                                    w13,
                                                                    w14
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

    // dbg!(&model_numbers);
    (
        *model_numbers.last().unwrap(),
        *model_numbers.first().unwrap(),
    )
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

    fn run(&mut self) -> bool {
        while self.pc < self.instructions.len() {
            let (opcode, (a, b)) = self.instructions.get(self.pc).unwrap();
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
