use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(24);

#[derive(Debug)]
enum Op {
    OR,
    XOR,
    AND,
}

#[derive(Debug)]
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

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let all_wires = lines_no_empty(top)
        .map(|l| l.split_once(": ").unwrap().0)
        .chain(lines_no_empty(bottom).flat_map(|l| l.split(" ")))
        .filter(|wire| wire.len() == 3 && &wire.to_lowercase() == wire)
        .unique()
        .sorted()
        .collect_vec();

    let z_wires = all_wires
        .iter()
        .enumerate()
        .filter(|(_, w)| w.starts_with("z"))
        .map(|(i, _)| i)
        .collect_vec();

    let gates = lines_no_empty(bottom)
        .map(|l| l.split(" ").collect_vec())
        .map(|parts| Gate {
            a: all_wires.iter().position(|&p| p == parts[0]).unwrap(),
            b: all_wires.iter().position(|&p| p == parts[2]).unwrap(),
            out: all_wires.iter().position(|&p| p == parts[4]).unwrap(),
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
        .for_each(|(r, v)| wires[all_wires.iter().position(|&p| p == r).unwrap()] = Some(v == "1"));

    if cfg!(debug_assertions) {
        println!("{:?}", all_wires);
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

    let sum = z_wires
        .iter()
        .enumerate()
        .map(|(idx, &w)| (wires[w].unwrap() as u64) << idx)
        .sum::<u64>();

    (Some(sum), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(2024), None));
    }
}
