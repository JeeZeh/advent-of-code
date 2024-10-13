use itertools::Itertools;

pub fn solve(input: String) -> (usize, usize) {
    let monkeys = input.split("\n\n").map(Monkey::from).collect_vec();

    (
        monkey_around(monkeys.clone(), 20, false),
        monkey_around(monkeys.clone(), 10_000, true),
    )
}

/// Simulates the monkeys throwing items for a provided number of `rounds` and returns
/// the level of monkey business observed at the end.
/// If `extra_worried`, calculates and applies a safe wrap-around value to all worry
/// levels while the monkeys are monkeying around.
fn monkey_around(mut monkeys: Vec<Monkey>, rounds: usize, extra_worried: bool) -> usize {
    let mut safe_wrap_around: Option<u64> = None;

    if extra_worried {
        safe_wrap_around = Some(
            monkeys
                .iter()
                .map(|m| m.test_modulo)
                .reduce(|acc, m| acc * m)
                .unwrap(),
        );
    }
    // Maximum number of items possible for one monkey to hold.
    let max_possible_items = monkeys.iter().map(|m| m.items.len()).sum();

    // To avoid double mutable borrow, keep track of what each monkey should 'catch'
    // when its turn starts.
    let mut thrown_items: Vec<Vec<u64>> =
        vec![Vec::with_capacity(max_possible_items); monkeys.len()];
    for _ in 0..rounds {
        for monkey in monkeys.iter_mut() {
            // Allow the current monkey to 'catch' anything thrown to it by other monkeys
            let items_to_catch = &mut thrown_items[monkey.monkey_num];
            if !items_to_catch.is_empty() {
                monkey.items.append(items_to_catch);
                items_to_catch.clear();
            }

            // Let current monkey inspect items and remember any thrown items to be caught
            // by other monkeys.
            monkey
                .inspect(safe_wrap_around)
                .iter()
                .for_each(|(m, w)| thrown_items[*m].push(*w));
        }
    }

    monkeys
        .iter()
        .map(|m| m.has_inspected)
        .sorted()
        .rev()
        .take(2)
        .reduce(|acc, v| acc * v)
        .unwrap()
}

impl Monkey {
    /// Makes the monkey inspect all items, returning a vec of items to be thrown, where the first
    /// element represents the monkey they item is thrown to, and the second representing the item
    /// with the latest-calculated worry level.
    ///
    /// If `wrap_around` is provided, this will not perform the division by 3 that reduces worry
    /// levels at a constant rate, but instead will wrap the worry level around the provided value.
    fn inspect(&mut self, wrap_around: Option<u64>) -> Vec<(usize, u64)> {
        let mut thrown = Vec::new();
        for item in self.items.drain(..) {
            let mut worry_level = match self.operation {
                Operation::Add => {
                    item + self
                        .operand
                        .expect("Expected Operation::Mult when Operand is None")
                }
                Operation::Mult => self.operand.unwrap_or(item) * item,
            };

            if let Some(wrap) = wrap_around {
                worry_level %= wrap;
            } else {
                worry_level = worry_level.div_floor(3);
            }

            let destination = if worry_level % self.test_modulo == 0 {
                self.destinations.0
            } else {
                self.destinations.1
            };

            self.has_inspected += 1;
            thrown.push((destination, worry_level));
        }

        thrown
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Mult,
}

#[derive(Debug, Clone)]
struct Monkey {
    monkey_num: usize,
    items: Vec<u64>,
    operation: Operation,
    operand: Option<u64>, // None means perform operation on 'old'
    test_modulo: u64,
    destinations: (usize, usize),
    has_inspected: usize,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let monkey_num = lines
            .next()
            .unwrap()
            .replace(":", "")
            .split_whitespace()
            .last()
            .unwrap()
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
        let operand: Option<u64> = op_line.split_once(' ').unwrap().1.parse::<u64>().ok();

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
            has_inspected: 0,
        }
    }
}
