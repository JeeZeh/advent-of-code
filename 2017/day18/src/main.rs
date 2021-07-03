use std::{
    collections::{HashMap, VecDeque},
    fs,
    time::Instant,
};

#[derive(Debug, Clone)]
struct Args(String, Option<String>);

#[derive(Clone)]
struct Instruction(String, Args);

struct CPU {
    registers: HashMap<String, i64>,
    buffer: VecDeque<i64>,
    rom: Vec<Instruction>,
    ptr: i64,
    ipc_mode: bool,
}

impl CPU {
    fn boot(rom: Vec<Instruction>, ipc_mode: bool) -> CPU {
        CPU {
            registers: HashMap::new(),
            buffer: VecDeque::new(),
            rom: rom,
            ptr: 0,
            ipc_mode,
        }
    }

    fn send(&mut self, value: i64) {
        self.buffer.push_back(value);
    }

    fn get_value(&self, arg: &String) -> i64 {
        // Converts the string to a number or gets the registry value it points to
        match arg.parse() {
            Ok(s) => s,
            Err(_) => *self.registers.get(arg).unwrap_or(&0),
        }
    }

    // Returns true if last instruction was not blocking (i.e. not waiting on rcv)
    fn next(&mut self) -> (bool, Option<i64>) {
        let old_ptr = self.ptr;
        let mut send = None;
        if self.ptr >= 0 && self.ptr < self.rom.len() as i64 {
            let instruction = self.rom[self.ptr as usize].clone();

            let op = instruction.0.as_str();
            let args = &instruction.1;

            // println!(
            //     "Running {} with {:?} at pointer {}",
            //     op,
            //     args,
            //     self.ptr
            // );

            send = match op {
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

        (old_ptr != self.ptr, send)
    }

    fn snd(&mut self, args: &Args) -> Option<i64> {
        let to_send = self.get_value(&args.0);
        self.ptr += 1;

        // Part 1 ('sound mode')
        if !self.ipc_mode {
            self.buffer.push_back(to_send);
            return None;
        } else {
            // sends the value of X to the other program
            return Some(to_send);
        }
    }

    fn set(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        self.registers.insert(args.0.clone(), val);

        self.ptr += 1;
        return None;
    }

    fn add(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        *self.registers.entry(args.0.clone()).or_insert(0) += val;

        self.ptr += 1;
        return None;
    }

    fn mul(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        *self.registers.entry(args.0.clone()).or_insert(0) *= val;

        self.ptr += 1;
        return None;
    }

    fn mod_(&mut self, args: &Args) -> Option<i64> {
        let val = self.get_value(&args.1.as_ref().unwrap());
        *self.registers.entry(args.0.clone()).or_insert(0) %= val;

        self.ptr += 1;
        return None;
    }

    fn rcv(&mut self, args: &Args) -> Option<i64> {
        // Part 1 ('sound mode')
        if !self.ipc_mode {
            if self.get_value(&args.0) != 0 {
                println!("Recovered: {}", self.buffer.pop_back().unwrap())
            }

            self.ptr = -1;
            return None;
        }

        // Programs do not continue to the next instruction until they have received a value
        if self.buffer.len() > 0 {
            self.registers
                .insert(args.0.clone(), self.buffer.pop_front().unwrap());
            self.ptr += 1;
        }

        return None;
    }

    fn jgz(&mut self, args: &Args) -> Option<i64> {
        if self.get_value(&args.0) > 0 {
            self.ptr += self.get_value(args.1.as_ref().unwrap());
        } else {
            self.ptr += 1;
        }

        return None;
    }
}

struct VM {
    cpu: CPU,
    running: bool,
    send_queue: VecDeque<i64>,
    send_count: usize,
}

impl VM {
    fn new(mut cpu: CPU, id: i64) -> VM {
        cpu.registers.insert(String::from("p"), id);

        VM {
            cpu,
            running: false,
            send_queue: VecDeque::new(),
            send_count: 0,
        }
    }

    fn next(&mut self, send: Option<i64>) {
        if send.is_some() {
            self.cpu.send(send.unwrap());
        }

        let (running, sent) = self.cpu.next();
        self.running = running;

        match sent {
            Some(v) => {
                self.send_queue.push_back(v);
                self.send_count += 1;
            }
            _ => (),
        }
    }
}

struct Hypervisor {
    vm_1: VM,
    vm_2: VM,
}

impl Hypervisor {
    fn new(cpu_1: CPU, cpu_2: CPU) -> Hypervisor {
        Hypervisor {
            vm_1: VM::new(cpu_1, 0),
            vm_2: VM::new(cpu_2, 1),
        }
    }

    fn start(&mut self) {
        self.vm_1.running = true;
        self.vm_2.running = true;

        while self.vm_1.running || self.vm_2.running {
            self.vm_1.next(self.vm_2.send_queue.pop_front());
            self.vm_2.next(self.vm_1.send_queue.pop_front());
        }

        println!("VM2 Send Count: {}", self.vm_2.send_count)
    }
}

fn main() {
    let now = Instant::now();

    part_one();
    part_two();

    println!("{}ms", now.elapsed().as_millis())
}

fn part_one() {
    let file = fs::read_to_string("./src/input").unwrap();
    let rom = file.lines().map(parse_instructions).collect();

    let mut cpu = CPU::boot(rom, false);

    while cpu.next().0 {}
}

fn part_two() {
    let file = fs::read_to_string("./src/input").unwrap();

    let rom_1 = file.lines().map(parse_instructions).collect();
    let rom_2 = file.lines().map(parse_instructions).collect();

    let cpu_1 = CPU::boot(rom_1, true);
    let cpu_2 = CPU::boot(rom_2, true);

    let mut hypervisor = Hypervisor::new(cpu_1, cpu_2);

    hypervisor.start();
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
