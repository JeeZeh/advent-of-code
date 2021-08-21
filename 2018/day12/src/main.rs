use std::fs;

#[derive(Debug)]
struct Conv {
    pattern: Vec<bool>,
    result: bool,
}

impl Conv {
    fn pattern_from_sequence(sequence: &str) -> Vec<bool> {
        sequence
            .chars()
            .map(|c| match c {
                '#' => true,
                _ => false,
            })
            .collect()
    }
}

fn main() {
    let (initial_state, convolutions) =
        parse_input(fs::read_to_string("./src/input").unwrap().as_str());

    let size = initial_state.len() * 5;

    let offset = initial_state.len();

    let mut state = Vec::with_capacity(size);

    // Copy initial into padded array
    for i in 0..size {
        if i < offset || i >= offset + initial_state.len() {
            state.push(false);
        } else {
            state.push(initial_state[i - offset]);
        }
    }

    print_state(&state);

    let mut last = state_sum(&state, offset);
    let mut change = state_sum(&state, offset);
    let mut bailed = 0;

    for i in 0..1000 {
        if i == 20 {
            println!("Part One: {}", state_sum(&state, offset));
        }
        state = convolve(&state, &convolutions);

        // If the pattern is sliding to the right
        // Basically, if the last sum is equal to the current state slid one to the left
        if last == state_sum(&state, offset + 1) {
            change = dbg!(state_sum(&state, offset)) - last;
            bailed = i + 1;
            break;
        }

        change = dbg!(state_sum(&state, offset) - last);
        last = state_sum(&state, offset);
    }

    println!(
        "Stabilised at iteration {} with RoC at {}, in {} generations sum will be {}",
        bailed,
        change,
        50_000_000_000 - bailed,
        state_sum(&state, offset) + (50_000_000_000 - bailed) * change
    );
}

fn convolve(state: &[bool], kernels: &[Conv]) -> Vec<bool> {
    let mut new_state = vec![false; state.len()];

    for start in 0..state.len() - 5 {
        for kernel in kernels {
            if state[start..start + 5] == kernel.pattern {
                new_state[start + 2] = kernel.result;
                break;
            }
        }
    }

    new_state
}

fn print_state(state: &[bool]) {
    let outstr: String = state.iter().map(|b| if *b { '#' } else { '.' }).collect();

    println!("{}", outstr);
}

fn state_sum(state: &[bool], offset: usize) -> i64 {
    state
        .iter()
        .enumerate()
        .map(|(i, p)| if *p { (i as i64) - (offset as i64) } else { 0 })
        .sum()
}

fn parse_input(input: &str) -> (Vec<bool>, Vec<Conv>) {
    let mut lines = input.lines();

    let initial_state =
        Conv::pattern_from_sequence(lines.next().unwrap().split(": ").nth(1).unwrap());

    lines.next();

    let convs = lines
        .map(|l| {
            let mut parts = l.split(" => ");
            let pattern = Conv::pattern_from_sequence(parts.next().unwrap());
            let result = Conv::pattern_from_sequence(parts.next().unwrap())[0];

            Conv { pattern, result }
        })
        .collect();

    return (initial_state, convs);
}
