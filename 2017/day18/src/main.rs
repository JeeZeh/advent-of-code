use std::{cell::RefCell, collections::HashMap, fs, ops::Deref, usize};

#[derive(Debug)]
struct Args(String, Option<String>);

struct Instruction(String, Args);

struct CPU {
    registers: HashMap<String, i64>,
    buffer: Vec<i64>,
    rom: Vec<Instruction>,
    ptr: i64,
    exit_on_rcv: bool,
}

impl CPU {
    fn boot(rom: Vec<Instruction>, exit_on_rcv: bool) -> CPU {
        CPU {
            registers: HashMap::new(),
            buffer: Vec::new(),
            rom: rom,
            ptr: 0,
            exit_on_rcv,
        }
    }

    fn get_value(&self, arg: &String) -> i64 {
        // Converts the string to a number or gets the registry value it points to
        match arg.parse() {
            Ok(s) => s,
            Err(_) => *self.registers.get(arg).unwrap_or(&0),
        }
    }

    fn run(&mut self) {
        while self.ptr >= 0 && self.ptr < self.rom.len() as i64 {
            let instruction = self.rom.get(self.ptr as usize).unwrap();
            let instruction = instruction.deref().clone();

            let op = instruction.0.as_str();
            let args = &instruction.1;

            // println!(
            //     "Running {} with {:?} at pointer {}",
            //     op,
            //     args,
            //     self.ptr.borrow()
            // );

            match op {
                "snd" => self.snd(args),
                "set" => self.set(args),
                "add" => self.add(args),
                "mul" => self.mul(args),
                "mod" => self.mod_(args),
                "rcv" => self.rcv(args),
                "jgz" => self.jgz(args),
                _ => panic!("Unknown operation"),
            };
        }
    }

    fn snd(&mut self, args: &Args) {
        let to_play = self.get_value(&args.0);
        self.buffer.push(to_play);

        self.ptr += 1;
    }

    fn set(&mut self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        self.registers.insert(args.0.clone(), val);

        self.ptr += 1;
    }

    fn add(&mut self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        *self.registers.entry(args.0.clone()).or_insert(0) += val;

        self.ptr += 1;
    }

    fn mul(&mut self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        // println!("{} * {}", val, self.get_value(args.0.clone()));
        *self.registers.entry(args.0.clone()).or_insert(0) *= val;

        self.ptr += 1;
    }

    fn mod_(&mut self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        *self.registers.entry(args.0.clone()).or_insert(0) %= val;

        self.ptr += 1;
    }

    fn rcv(&mut self, args: &Args) {
        if self.get_value(&args.0) != 0 {
            println!("Recovered: {}", self.buffer.last().unwrap())
        }

        if self.exit_on_rcv {
            self.ptr = -1;
        } else {
            self.ptr += 1;
        }
    }

    fn jgz(&mut self, args: &Args) {
        if self.get_value(&args.0) > 0 {
            self.ptr += self.get_value(args.1.as_ref().unwrap());
        } else {
            self.ptr += 1;
        }
    }
}

fn main() {
    part_one();
}

fn part_one() {
    let rom: Vec<Instruction> = fs::read_to_string("./src/input")
        .unwrap()
        .lines()
        .map(parse_instructions)
        .collect();

    let cpu = CPU::boot(rom, true);

    cpu.run();
}

fn parse_instructions(line: &str) -> Instruction {
    let mut parts = line.split(" ");
    let ins = parts.next().unwrap();
    let arg_1 = String::from(parts.next().unwrap());
    let arg_2: Option<String> = match parts.next() {
        Some(s) => Some(String::from(s)),
        None => None,
    };

    Instruction(String::from(ins), Args(arg_1, arg_2))
}
