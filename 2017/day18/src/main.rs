use std::{cell::RefCell, collections::HashMap, fs, usize};

#[derive(Debug)]
struct Args(String, Option<String>);

struct Instruction(String, Args);

struct CPU {
    registers: RefCell<HashMap<String, i64>>,
    buffer: RefCell<Vec<i64>>,
    rom: RefCell<Vec<Instruction>>,
    ptr: RefCell<i64>,
    exit_on_rcv: bool,
}

impl CPU {
    fn boot(rom: Vec<Instruction>, exit_on_rcv: bool) -> CPU {
        CPU {
            registers: RefCell::from(HashMap::new()),
            buffer: RefCell::from(Vec::new()),
            rom: RefCell::from(rom),
            ptr: RefCell::from(0),
            exit_on_rcv,
        }
    }

    fn get_value(&self, arg: &String) -> i64 {
        // Converts the string to a number or gets the registry value it points to
        match arg.parse() {
            Ok(s) => s,
            Err(_) => *self.registers.borrow().get(arg).unwrap_or(&0),
        }
    }

    fn run(&self) {
        let rom = self.rom.borrow();
        while *self.ptr.borrow() >= 0 && *self.ptr.borrow() < rom.len() as i64 {
            let instruction = rom.get(*self.ptr.borrow() as usize).unwrap();

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

    fn snd(&self, args: &Args) {
        let to_play = self.get_value(&args.0);
        self.buffer.borrow_mut().push(to_play);

        *self.ptr.borrow_mut() += 1;
    }

    fn set(&self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        self.registers.borrow_mut().insert(args.0.clone(), val);

        *self.ptr.borrow_mut() += 1;
    }

    fn add(&self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        *self
            .registers
            .borrow_mut()
            .entry(args.0.clone())
            .or_insert(0) += val;

        *self.ptr.borrow_mut() += 1;
    }

    fn mul(&self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        // println!("{} * {}", val, self.get_value(args.0.clone()));
        *self
            .registers
            .borrow_mut()
            .entry(args.0.clone())
            .or_insert(0) *= val;

        *self.ptr.borrow_mut() += 1;
    }

    fn mod_(&self, args: &Args) {
        let val = self.get_value(args.1.as_ref().unwrap());
        *self
            .registers
            .borrow_mut()
            .entry(args.0.clone())
            .or_insert(0) %= val;

        *self.ptr.borrow_mut() += 1;
    }

    fn rcv(&self, args: &Args) {
        if self.get_value(&args.0) != 0 {
            println!("Recovered: {}", self.buffer.borrow().last().unwrap())
        }

        if self.exit_on_rcv {
            *self.ptr.borrow_mut() = -1;
        } else {
            *self.ptr.borrow_mut() += 1;
        }
    }

    fn jgz(&self, args: &Args) {
        if self.get_value(&args.0) > 0 {
            *self.ptr.borrow_mut() += self.get_value(args.1.as_ref().unwrap());
        } else {
            *self.ptr.borrow_mut() += 1;
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
