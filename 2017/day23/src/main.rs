use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::Instant,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Set,
    Sub,
    Mul,
    Jnz,
}

#[derive(Debug, Clone)]
struct Args(String, Option<String>);

#[derive(Clone)]
struct Instruction(Op, Args);

struct CPU {
    registers: [i64; 8],
    rom: Vec<Instruction>,
    ptr: i64,
}

impl CPU {
    fn boot(rom: Vec<Instruction>, ipc_mode: bool) -> CPU {
        CPU {
            registers: [0; 8],
            rom: rom,
            ptr: 0,
        }
    }

    fn get_register_index(register: &str) -> usize {
        match register {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => panic!("Unknown register: {}", register),
        }
    }

    fn get_value(&self, arg: &String) -> i64 {
        // Converts the string to a number or gets the registry value it points to
        match arg.parse() {
            Ok(s) => s,
            Err(_) => self.registers[CPU::get_register_index(arg)] as i64,
        }
    }

    // Returns true if last instruction was not blocking (i.e. not waiting on rcv)
    fn next(&mut self) -> Option<Op> {
        if self.ptr >= 0 && self.ptr < self.rom.len() as i64 {
            let instruction = self.rom[self.ptr as usize].clone();

            let op = instruction.0;
            let args = &instruction.1;

            let jmp = match op {
                Op::Set => self.set(args),
                Op::Sub => self.sub(args),
                Op::Mul => self.mul(args),
                Op::Jnz => self.jnz(args),
                _ => panic!("Unknown operation"),
            };

            self.ptr += jmp.unwrap_or(1);
            return Some(op);
        }
        None
    }

    fn set(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        self.registers[CPU::get_register_index(&args.0)] = val;

        None
    }

    fn sub(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        self.registers[CPU::get_register_index(&args.0)] -= val;

        None
    }

    fn mul(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        self.registers[CPU::get_register_index(&args.0)] *= val;

        None
    }

    fn jnz(&mut self, args: &Args) -> Option<i64> {
        if self.get_value(&args.0) != 0 {
            return Some(self.get_value(args.1.as_ref().unwrap()));
        }

        None
    }
}

fn main() {
    let now = Instant::now();

    part_one();
    part_two_jimmy();

    println!("{}ms", now.elapsed().as_millis())
}

fn part_one() {
    let file = fs::read_to_string("./src/input/real").unwrap();
    let rom = file.lines().map(parse_instructions).collect();

    let mut cpu = CPU::boot(rom, false);

    let mut mul_count = 0;
    while let Some(op) = cpu.next() {
        if op == Op::Mul {
            mul_count += 1;
        }
    }

    println!("Mul count: {}", mul_count);
}

fn part_two() {
    let file = fs::read_to_string("./src/input/real").unwrap();
    let rom = file.lines().map(parse_instructions).collect();

    let mut cpu = CPU::boot(rom, false);
    cpu.registers[0] = 1; // Set register 'a' to 1 to match the input

    while cpu.next().is_some() {
        println!("Pointer: {}", cpu.ptr);
        println!("Registers: {:?}", cpu.registers);
        println!("-------------------");
    }

    println!("Reg H: {}", cpu.registers[CPU::get_register_index("h")]);
}

/// Almost got this right by working through the opcodes,
/// but never realized that this was a prime counting problem.
///
/// ```
/// b = 109900
/// c = 126900
/// while b != c:
///     f = 1
///     e = 2
///     g = 2
///     d = 2
///     while d != b:
///         while e != b:
///             g = (d * e) - b
///             if d * e == b:
///                 f = 0
///
///             e += 1;
///             g = e - b;
///
///         d += 1
///         g = d - b
///
///     if f == 0:
///         h += 1
///
///     g = b - c
///     b += 17
/// ```
fn part_two_jimmy() {
    let c: i32 = 126900;
    let mut h = 0;
    for b in (109900..=c).step_by(17) {
        let mut f = 1;
        for d in 2..b.isqrt() + 1 {
            if b % d == 0 {
                f = 0; // Not prime
                break;
            }
        }
        if f == 0 {
            h += 1;
        }
    }
    println!("h: {}", h);
}

fn parse_instructions(line: &str) -> Instruction {
    let mut parts = line.split(" ");
    let ins = parts.next().unwrap();
    let arg_1 = String::from(parts.next().unwrap());
    let arg_2: Option<String> = match parts.next() {
        Some(s) => Some(String::from(s)),
        None => None,
    };

    let op = match ins {
        "set" => Op::Set,
        "sub" => Op::Sub,
        "mul" => Op::Mul,
        "jnz" => Op::Jnz,
        _ => panic!("Unknown operation: {}", ins),
    };

    Instruction(op, Args(arg_1, arg_2))
}
