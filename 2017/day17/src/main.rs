use std::time::Instant;

fn main() {
    let instant = Instant::now();

    let steps = 394;

    println!("Part 1: {}", simulate(2017, steps, false));
    println!("Part 1: {}", simulate(50_000_000, steps, true));

    println!("{}ms", instant.elapsed().as_millis())
}

fn simulate(runs: usize, steps: usize, part_two: bool) -> usize {
    let mut buffer: Vec<usize> = Vec::new();
    buffer.push(0);

    let mut curr = 0;

    for run in 1..=runs {
        curr = ((curr + steps) % run) + 1;
        if !part_two || curr == 1 {
            buffer.insert(curr, run);
        }
    }

    if part_two {
        buffer[1]
    } else {
        buffer[curr]
    }
}
