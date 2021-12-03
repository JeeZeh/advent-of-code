use std::fs;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Inst(Direction, i32);

fn main() {
    let input: Vec<Inst> = fs::read_to_string("./src/input")
        .unwrap()
        .lines()
        .map(parse_input)
        .collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse_input(line: &str) -> Inst {
    let mut parts = line.split(" ");

    let direction = match parts.next().unwrap() {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!(),
    };
    let value = parts.next().unwrap().parse().unwrap();

    Inst(direction, value)
}

fn part_one(instructions: &[Inst]) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for inst in instructions {
        match inst.0 {
            Direction::Down => y += inst.1,
            Direction::Up => y -= inst.1,
            Direction::Forward => x += inst.1,
        }
    }

    x * y
}

fn part_two(instructions: &[Inst]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for inst in instructions {
        match inst.0 {
            Direction::Down => aim += inst.1,
            Direction::Up => aim -= inst.1,
            Direction::Forward => {
                x += inst.1;
                y += aim * inst.1;
            }
        }
    }

    x * y
}
