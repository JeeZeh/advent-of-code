use std::usize;

use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    OR,
    XOR,
    AND,
}

#[derive(Debug, Clone, Copy)]
struct Gate {
    a: usize,
    b: usize,
    out: usize,
    op: Op,
}

impl Gate {
    fn evaluate(&self, wires: &mut [Option<bool>]) {
        if let Some(av) = wires[self.a] {
            if let Some(bv) = wires[self.b] {
                wires[self.out] = match self.op {
                    Op::OR => Some(av | bv),
                    Op::XOR => Some(av ^ bv),
                    Op::AND => Some(av & bv),
                }
            }
        }
    }
}

pub fn solve(input: &str) -> (Option<String>, Option<String>) {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let lookup = lines_no_empty(top)
        .map(|l| l.split_once(": ").unwrap().0)
        .chain(lines_no_empty(bottom).flat_map(|l| l.split(" ")))
        .filter(|wire| wire.len() == 3 && &wire.to_lowercase() == wire)
        .unique()
        .sorted()
        .collect_vec();

    let z_wires = lookup
        .iter()
        .enumerate()
        .filter(|(_, w)| w.starts_with("z"))
        .map(|(i, _)| i)
        .collect_vec();

    let gates = lines_no_empty(bottom)
        .map(|l| l.split(" ").collect_vec())
        .map(|parts| Gate {
            a: lookup.iter().position(|&p| p == parts[0]).unwrap(),
            b: lookup.iter().position(|&p| p == parts[2]).unwrap(),
            out: lookup.iter().position(|&p| p == parts[4]).unwrap(),
            op: match parts[1] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Unknown op: {}", parts[1]),
            },
        })
        .collect_vec();

    // TODO: make work for any input size;
    let mut wires = [None; 312];
    top.lines()
        .map(|l| l.split_once(": ").unwrap())
        .for_each(|(r, v)| wires[lookup.iter().position(|&p| p == r).unwrap()] = Some(v == "1"));

    if cfg!(debug_assertions) {
        println!("{:?}", lookup);
        println!("{:?}", wires);
        println!("{:?}", &gates);
    }

    // Part 1
    while z_wires.iter().any(|&z| wires[z].is_none()) {
        for gate in &gates {
            if wires[gate.out].is_none() {
                gate.evaluate(&mut wires);
            }
        }
    }

    let z_value_initial = z_wires
        .iter()
        .enumerate()
        .map(|(idx, &w)| (wires[w].unwrap() as u64) << idx)
        .sum::<u64>();

    // Part 2: https://www.reddit.com/r/adventofcode/comments/1hl698z/2024_day_24_solutions/m3ljgo9/
    let mut connections: Vec<Vec<(Op, usize)>> = vec![vec![]; 400];
    gates.iter().for_each(|g| {
        connections[g.a].push((g.op, g.out));
        connections[g.b].push((g.op, g.out));
    });

    let mut wrong_outputs = vec![];
    for &Gate { a, b, out, op } in gates.iter() {
        // basically we ensure the adder looks like this:
        // https://en.wikipedia.org/wiki/Adder_(electronics)#/media/File:Fulladder.gif
        let chained_ops = &connections[out];
        let chained_ops_contain = |op| chained_ops.iter().any(|a| a.0 == op);

        let lhs = lookup[a];
        let rhs = lookup[b];
        let has_chained_xor = chained_ops_contain(Op::XOR);
        let has_chained_and = chained_ops_contain(Op::AND);
        let has_chained_or = chained_ops_contain(Op::OR);
        let takes_first_input = lhs.ends_with("00") && rhs.ends_with("00");
        let takes_input_bit = (lhs.starts_with('x') && rhs.starts_with('y'))
            || (rhs.starts_with('x') && lhs.starts_with('y'));
        let outputs_bit = lookup[out].starts_with('z');
        let outputs_last_bit = lookup[out] == "z45";

        let valid = match op {
            Op::XOR => {
                // XOR only outputs a bit if it doesn't take an input bit
                (!takes_input_bit && outputs_bit)
                // XOR only takes an input bit if a XOR follows it
                    || (takes_input_bit && has_chained_xor)
                    // unless the input bits are the first bits (no carryover bit exists)
                    || takes_first_input && outputs_bit
            }
            Op::OR => {
                // OR either outputs into z45 or an AND and XOR (carryover bit)
                outputs_last_bit || (has_chained_and && has_chained_xor)
            }
            Op::AND => {
                // ANDs only lead into ORs
                has_chained_or
                // unless the input bits are the first bits (no carryover bit exists)
                || takes_first_input
            }
            _ => {
                unreachable!()
            }
        };
        if !valid {
            wrong_outputs.push(out);
        }
    }

    (
        Some(format!("{}", z_value_initial)),
        Some(wrong_outputs.iter().sorted().map(|&w| lookup[w]).join(",")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(String::from("2024")), None));
    }
}
