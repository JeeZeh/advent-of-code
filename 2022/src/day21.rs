use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(input: String) -> (i64, i64) {
    let monkey_machine = build_monkey_machine(input);

    let root = monkey_machine.get("root").unwrap();
    let part_one = root.shout(&monkey_machine);
    let part_two = find_correct_humn_value(&monkey_machine);

    (part_one, part_two)
}

/// Unwind and reverse the operations from root to humn to determine what the humn should shout
fn find_correct_humn_value(monkey_machine: &HashMap<String, Monkey>) -> i64 {
    let path_to_humn = locate_humn(vec!["root".to_string()], &monkey_machine).unwrap();
    let mut monkeys_to_resolve = path_to_humn
        .iter()
        .map(|m| monkey_machine.get(m).unwrap())
        .peekable();

    let mut needs_value = 1;

    loop {
        let current = monkeys_to_resolve.next().unwrap();
        // Stop trying to unwind operations when we reach the humn
        if current.name == "humn" {
            break;
        }

        // Resolve the left or right operand, depending on where the human is known to be
        let (lhs, rhs) = current.operands.clone().unwrap();
        let next = monkeys_to_resolve.peek().unwrap();
        let human_left = &next.name == &lhs;
        let other_value = monkey_machine
            .get(if human_left { &rhs } else { &lhs })
            .unwrap()
            .shout(monkey_machine);

        // Set our 'next' required value that should be output by the previous operation in the stack
        if current.name == "root" {
            // We need to equal whatever is on the other side of the root operation
            needs_value = other_value;
        } else {
            // Otherwise, carefully reverse operands and operation, accounting for division and subtraction
            // special cases.
            needs_value = match (current.operation, human_left) {
                (Operation::Sub | Operation::Div, false) => current.operation.apply(other_value, needs_value),
                (_, _) => current.operation.reverse().apply(needs_value, other_value),
            };
        }
    }
    needs_value
}

/// Follow the path to the human from some starting point, tracking the path taken along the way.
/// Basically a DFS down the tree.
fn locate_humn(path: Vec<String>, monkey_machine: &HashMap<String, Monkey>) -> Option<Vec<String>> {
    let starting = path.last().unwrap();
    if starting == "humn" {
        return Some(path);
    }

    let current = monkey_machine.get(starting).unwrap();

    // Literal monkeys are dead ends
    if current.operation == Operation::Lit {
        return None;
    }

    let (check_left, check_right) = current.operands.clone().unwrap();
    let mut left_path = path.clone();
    left_path.push(check_left);
    if let Some(found) = locate_humn(left_path, monkey_machine) {
        return Some(found);
    }

    let mut right_path = path.clone();
    right_path.push(check_right);
    if let Some(found) = locate_humn(right_path, monkey_machine) {
        return Some(found);
    }

    None
}

fn build_monkey_machine(input: String) -> HashMap<String, Monkey> {
    let mut machine = HashMap::new();
    input.lines().map(Monkey::from).for_each(|m| {
        machine.insert(m.name.clone(), m);
    });

    machine
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    Lit,
}

impl Operation {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Sub => lhs - rhs,
            Operation::Mult => lhs * rhs,
            Operation::Div => lhs / rhs,
            Operation::Eq => {
                if lhs == rhs {
                    1
                } else {
                    -1
                }
            }
            Operation::Lit => panic!("Cannot apply literal operation"),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Mult => Operation::Div,
            Operation::Div => Operation::Mult,
            _ => panic!("Cannot reverse"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    name: String,
    operation: Operation,
    operands: Option<(String, String)>,
    literal: Option<i64>,
}
impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mult,
            "/" => Operation::Div,
            _ => panic!("Unsupported operation"),
        }
    }
}

impl Monkey {
    fn shout(&self, machine: &HashMap<String, Monkey>) -> i64 {
        match self.operation {
            Operation::Lit => self.literal.unwrap(),
            op => {
                let (resolve_left, resolve_right) = self.operands.clone().unwrap();

                let lhs = machine.get(&resolve_left).unwrap().shout(machine);
                let rhs = machine.get(&resolve_right).unwrap().shout(machine);

                op.apply(lhs, rhs)
            }
        }
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let (name, function) = value.split_once(": ").unwrap();
        let function_parts = function.split_whitespace().collect_vec();
        if function_parts.len() == 1 {
            return Monkey {
                name: name.to_string(),
                operation: Operation::Lit,
                operands: None,
                literal: Some(function_parts[0].parse().unwrap()),
            };
        } else {
            return Monkey {
                name: name.to_string(),
                operation: Operation::from(function_parts[1]),
                operands: Some((function_parts[0].to_string(), function_parts[2].to_string())),
                literal: None,
            };
        }
    }
}
