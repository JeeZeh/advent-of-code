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
    buffer: VecDeque<i64>,
    rom: Vec<Instruction>,
    ptr: i64,
    ipc_mode: bool,
}

impl CPU {
    fn boot(rom: Vec<Instruction>, ipc_mode: bool) -> CPU {
        CPU {
            registers: [0; 8],
            buffer: VecDeque::new(),
            rom: rom,
            ptr: 0,
            ipc_mode,
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

            // println!(
            //     "Running {:?} with {:?} at pointer {}",
            //     op,
            //     args,
            //     self.ptr
            // );

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

struct VM {
    cpu: CPU,
    running: bool,
    send_queue: VecDeque<i64>,
    send_count: usize,
}

// impl VM {
//     fn new() -> VM {
//         VM {
//             cpu,
//             running: false,
//             send_queue: VecDeque::new(),
//             send_count: 0,
//         }
//     }

//     fn next(&mut self, send: Option<i64>) {
//         if send.is_some() {
//             self.cpu.send(send.unwrap());
//         }

//         let (running, sent) = self.cpu.next();
//         self.running = running;

//         match sent {
//             Some(v) => {
//                 self.send_queue.push_back(v);
//                 self.send_count += 1;
//             }
//             _ => (),
//         }
//     }
// }

// struct Hypervisor {
//     vm_1: VM,
//     vm_2: VM,
// }

// impl Hypervisor {
//     fn new(cpu_1: CPU, cpu_2: CPU) -> Hypervisor {
//         Hypervisor {
//             vm_1: VM::new(cpu_1, 0),
//             vm_2: VM::new(cpu_2, 1),
//         }
//     }

//     fn start(&mut self) {
//         self.vm_1.running = true;
//         self.vm_2.running = true;

//         while self.vm_1.running || self.vm_2.running {
//             self.vm_1.next(self.vm_2.send_queue.pop_front());
//             self.vm_2.next(self.vm_1.send_queue.pop_front());
//         }

//         println!("VM2 Send Count: {}", self.vm_2.send_count)
//     }
// }

fn main() {
    let now = Instant::now();

    part_one();
    // part_two();

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

// fn part_two() {
//     let file = fs::read_to_string("./src/input").unwrap();

//     let rom_1 = file.lines().map(parse_instructions).collect();
//     let rom_2 = file.lines().map(parse_instructions).collect();

//     let cpu_1 = CPU::boot(rom_1, true);
//     let cpu_2 = CPU::boot(rom_2, true);

//     let mut hypervisor = Hypervisor::new(cpu_1, cpu_2);

//     hypervisor.start();
// }

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
