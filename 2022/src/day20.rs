use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

pub fn solve(input: Vec<i64>) -> (i64, i64) {
    let (mut map, mut zero) = build_map(&input);

    for (i, n) in input.iter().enumerate() {
        mix_number(&mut map, (i, *n));
        // dbg!(&map);
    }

    let part_one = [1000, 2000, 3000]
        .iter()
        .map(|n| get_relative_number(&map, zero, *n).1)
        .sum();

    let input_with_key = input.iter().map(|i| i * 811589153).collect_vec();
    (map, zero) = build_map(&input_with_key);

    for _ in 0..10 {
        for (i, n) in input_with_key.iter().enumerate() {
            mix_number(&mut map, (i, *n));
        }
    }

    let part_two = [1000, 2000, 3000]
        .iter()
        .map(|n| get_relative_number(&map, zero, *n).1)
        .sum();

    (part_one, part_two)
}

fn mix_number(map: &mut HashMap<(usize, i64), Number>, identifier: (usize, i64)) {
    if identifier.1 == 0 {
        return;
    }

    // Hold starting
    let mut number = map.get(&identifier).unwrap().clone();

    // Stitch up start
    let start_prev = number.prev;
    let start_next = number.next;
    map.get_mut(&start_prev).unwrap().next = start_next;
    map.get_mut(&start_next).unwrap().prev = start_prev;

    // Find ending
    let current = get_relative_number(&map, number.this, number.this.1);

    let (end_prev, end_next) = match identifier.1.is_positive() {
        true => (current, map.get(&current).unwrap().next),
        false => (map.get(&current).unwrap().prev, current),
    };

    // Stitch up end
    number.prev = end_prev;
    number.next = end_next;
    map.get_mut(&end_prev).unwrap().next = number.this;
    map.get_mut(&end_next).unwrap().prev = number.this;
    map.insert(identifier, number);
}

fn get_relative_number(
    map: &HashMap<(usize, i64), Number>,
    start: (usize, i64),
    rel: i64,
) -> (usize, i64) {
    let size = map.len() as i64;
    let wrapped = rel % (size - 1);

    let mut current = start;
    for _ in 0..wrapped.abs() {
        if wrapped.is_negative() {
            current = map.get(&current).unwrap().prev;
        } else {
            current = map.get(&current).unwrap().next;
        }
    }
    current
}

fn build_map(input: &[i64]) -> (HashMap<(usize, i64), Number>, (usize, i64)) {
    let mut map: HashMap<(usize, i64), Number> = HashMap::new();
    let mut zero = (0, 0);

    for (i, number) in input.iter().enumerate() {
        let prev_idx = (i as i64 - 1).rem_euclid(input.len() as i64) as usize;
        let next_idx = (i as i64 + 1).rem_euclid(input.len() as i64) as usize;
        let previous = input[prev_idx];
        let next = input[next_idx];

        if *number == 0 {
            zero = (i, *number);
        }

        map.insert(
            (i, *number),
            Number {
                prev: (prev_idx, previous),
                this: (i, *number),
                next: (next_idx, next),
            },
        );
    }

    (map, zero)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Number {
    prev: (usize, i64),
    this: (usize, i64),
    next: (usize, i64),
}
