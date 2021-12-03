enum Direction {
    Up,
    Down,
    Forward,
}

pub fn solve(input: Vec<String>) -> (i32, i32) {
    let instructions: Vec<(Direction, i32)> = input.iter().map(parse_input).collect();
    (part_one(&instructions), part_two(&instructions))
}

fn parse_input(line: &String) -> (Direction, i32) {
    let mut parts = line.split(" ");

    let direction = match parts.next().unwrap() {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!(),
    };
    let value = parts.next().unwrap().parse().unwrap();

    (direction, value)
}

fn part_one(instructions: &[(Direction, i32)]) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for (direction, value) in instructions {
        match direction {
            Direction::Down => y += value,
            Direction::Up => y -= value,
            Direction::Forward => x += value,
        }
    }

    x * y
}

fn part_two(instructions: &[(Direction, i32)]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for (direction, value) in instructions {
        match direction {
            Direction::Down => aim += value,
            Direction::Up => aim -= value,
            Direction::Forward => {
                x += value;
                y += aim * value;
            }
        }
    }

    x * y
}
