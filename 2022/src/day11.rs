use itertools::Itertools;

pub fn solve(input: String) -> (i32, i32) {
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect_vec();

    for round in 0..20 {
        for monkey in monkeys.iter_mut() {

        }
    }

    (0, 0)
}

impl Monkey {
    fn inspect(&mut self) -> Vec<(usize, i32)> {
        let mut thrown = Vec::new();
        for item in &self.items {
            let worry_level = match self.operation {
                Operation::Add => {
                    item + self
                        .operand
                        .expect("Expected Operation::Mult when Operand is None")
                }
                Operation::Mult => self.operand.unwrap_or(*item) * item,
            }
            .div_floor(3);
            let destination = if worry_level % self.test_modulo == 0 {
                self.destinations.0
            } else {
                self.destinations.1
            };

            thrown.push((destination, worry_level));
        }

        thrown
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

#[derive(Debug)]
struct Monkey {
    monkey_num: usize,
    items: Vec<i32>,
    operation: Operation,
    operand: Option<i32>, // None means perform operation on 'old'
    test_modulo: i32,
    destinations: (usize, usize),
    has_inspected: Vec<i32>,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let monkey_num = lines
            .next()
            .unwrap()
            .replace(":", "")
            .split_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect_vec();

        let op_line = &lines.next().unwrap().split("new = old ").last().unwrap();

        let operation = match op_line.chars().next().unwrap() {
            '*' => Operation::Mult,
            '+' => Operation::Add,
            _ => panic!("Unexpected operation"),
        };
        let operand: Option<i32> = op_line.split_once(' ').unwrap().1.parse::<i32>().ok();

        let test_modulo = lines
            .next()
            .unwrap()
            .split("by ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let if_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            monkey_num,
            items,
            operation,
            operand,
            test_modulo,
            destinations: (if_true, if_false),
            has_inspected: Vec::new(),
        }
    }
}
