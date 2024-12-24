use std::{ops::Range, usize};

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

fn find_first_output(
    gates: &[Gate],
    wire: usize,
    lookup: &[&str],
    output_wires: &[usize],
) -> Option<usize> {
    let parents = gates
        .iter()
        .filter(|g| g.a == wire || g.b == wire)
        .collect_vec();

    if let Some(valid_output) = parents.iter().find(|g| output_wires.contains(&g.out)) {
        return Some(valid_output.out - 1);
    }

    parents
        .iter()
        .filter_map(|p| find_first_output(gates, p.out, lookup, output_wires))
        .next()
}

// fn run(wires: &mut [bool], gates: &[Gate]){
//     gates.iter().sorted_by_key(|g| g.out).rev().
// }

pub fn solve(input: &str) -> (Option<String>, Option<String>) {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let lookup = lines_no_empty(top)
        .map(|l| l.split_once(": ").unwrap().0)
        .chain(lines_no_empty(bottom).flat_map(|l| l.split(" ")))
        .filter(|wire| wire.len() == 3 && &wire.to_lowercase() == wire)
        .unique()
        .sorted()
        .collect_vec();

    let x_wires = lookup
        .iter()
        .enumerate()
        .filter(|(_, w)| w.starts_with("x"))
        .map(|(i, _)| i)
        .collect_vec();
    let y_wires = lookup
        .iter()
        .enumerate()
        .filter(|(_, w)| w.starts_with("y"))
        .map(|(i, _)| i)
        .collect_vec();
    let z_wires = lookup
        .iter()
        .enumerate()
        .filter(|(_, w)| w.starts_with("z"))
        .map(|(i, _)| i)
        .collect_vec();

    let mut gates = lines_no_empty(bottom)
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

    // Reset
    wires = [None; 312];
    let invalid_end_wires = gates
        .iter()
        .filter(|gate| gate.op != Op::XOR && z_wires[..z_wires.len() - 1].contains(&gate.out))
        .cloned()
        .collect_vec();

    let invalid_mid_wires = gates
        .iter()
        .filter(|gate| {
            gate.op == Op::XOR
                && !lookup[gate.a].starts_with("x")
                && !lookup[gate.a].starts_with("y")
                && !lookup[gate.b].starts_with("x")
                && !lookup[gate.b].starts_with("y")
        })
        .cloned()
        .collect_vec();

    let mut switches = Vec::new();
    invalid_mid_wires.iter().for_each(|mid| {
        let to_switch = invalid_end_wires.iter().find(|w| {
            w.out == find_first_output(&gates, mid.out, &lookup, &z_wires).unwrap_or(usize::MAX)
        });
        if let Some(switch) = to_switch {
            switches.push((mid.out, switch.out));
        }
    });

    println!("{:?}", switches);
    for gate in gates.iter_mut() {
        if let Some((_, r)) = switches.iter().find(|(a, _)| *a == gate.out) {
            gate.out = *r;
        }
        if let Some((l, _)) = switches.iter().find(|(_, b)| *b == gate.out) {
            gate.out = *l;
        }
    }
    while z_wires.iter().any(|&z| wires[z].is_none()) {
        for gate in &gates {
            if wires[gate.out].is_none() {
                gate.evaluate(&mut wires);
            }
        }
    }

    let x_input = x_wires
        .iter()
        .enumerate()
        .map(|(idx, &w)| (wires[w].unwrap_or(false) as u64) << idx)
        .sum::<u64>();
    let y_input = y_wires
        .iter()
        .enumerate()
        .map(|(idx, &w)| (wires[w].unwrap_or(false) as u64) << idx)
        .sum::<u64>();
    let z_value_swapped = z_wires
        .iter()
        .enumerate()
        .map(|(idx, &w)| (wires[w].unwrap() as u64) << idx)
        .sum::<u64>();

    let diff = (x_input + y_input) ^ z_value_swapped;
    let zero_bits = format!(
        "{:02}",
        format!("0{:b} ", diff)
            .chars()
            .rev()
            .take_while(|&c| c == '0')
            .count()
    );

    println!("{}", zero_bits);

    let invalid_carry_wires = gates
        .iter()
        .filter(|g| lookup[g.a].ends_with(&zero_bits) && lookup[g.b].ends_with(&zero_bits))
        .map(|&g| g.out)
        .collect_vec();

    let swaps = invalid_end_wires
        .iter()
        .map(|g| g.out)
        .chain(invalid_mid_wires.iter().map(|g| g.out))
        .chain(invalid_carry_wires)
        .map(|w| lookup[w])
        .sorted()
        .join(",");
    (Some(format!("{}", z_value_swapped)), Some(swaps))
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
