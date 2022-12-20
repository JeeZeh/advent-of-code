use itertools::Itertools;

pub fn solve(input: Vec<i64>) -> (i64, i64) {
    let (mut vec, mut zero) = build_map(&input);

    // Part 1: run once
    for i in 0..input.len() {
        mix_number(&mut vec, i);
    }
    let part_one = get_coordinate_sum(&vec, zero);

    // Part 2: apply decryption key and run 10 times
    let input_with_key = input.iter().map(|i| i * 811589153).collect_vec();
    (vec, zero) = build_map(&input_with_key);
    for _ in 0..10 {
        for i in 0..input_with_key.len() {
            mix_number(&mut vec, i);
        }
    }
    let part_two = get_coordinate_sum(&vec, zero);

    (part_one, part_two)
}

fn get_coordinate_sum(vec: &[Number], zero_pos: usize) -> i64 {
    [1000, 2000, 3000]
        .iter()
        .map(|n| vec[get_relative_number(&vec, zero_pos, *n)].this)
        .sum()
}

fn mix_number(vec: &mut Vec<Number>, identifier: usize) {
    // Hold starting
    let mut number = vec[identifier];

    if number.this == 0 {
        return;
    }

    // Stitch up start
    let start_prev = number.prev;
    let start_next = number.next;
    vec.get_mut(start_prev).unwrap().next = start_next;
    vec.get_mut(start_next).unwrap().prev = start_prev;

    // Find ending
    let current = get_relative_number(&vec, identifier, number.this);

    let (end_prev, end_next) = match number.this.is_positive() {
        true => (current, vec[current].next),
        false => (vec[current].prev, current),
    };

    // Stitch up end
    number.prev = end_prev;
    number.next = end_next;
    vec.get_mut(end_prev).unwrap().next = identifier;
    vec.get_mut(end_next).unwrap().prev = identifier;
    vec[identifier] = number;
}

fn get_relative_number(vec: &[Number], start: usize, rel: i64) -> usize {
    let size = vec.len() as i64;
    let wrapped = rel % (size - 1);

    let positive = wrapped.is_positive();
    let mut current = start;
    for _ in 0..wrapped.abs() {
        current = if positive {
            vec[current].next
        } else {
            vec[current].prev
        };
    }
    current
}

fn build_map(input: &[i64]) -> (Vec<Number>, usize) {
    let mut vec: Vec<Number> = Vec::with_capacity(input.len());
    let mut zero = 0;

    for (i, number) in input.iter().enumerate() {
        let prev_idx = (i as i64 - 1).rem_euclid(input.len() as i64) as usize;
        let next_idx = (i as i64 + 1).rem_euclid(input.len() as i64) as usize;

        if *number == 0 {
            zero = i;
        }

        vec.push(Number {
            prev: prev_idx,
            this: *number,
            next: next_idx,
        });
    }

    (vec, zero)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Number {
    prev: usize,
    this: i64,
    next: usize,
}
