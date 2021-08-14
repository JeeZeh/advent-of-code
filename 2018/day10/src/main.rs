use std::{
    collections::HashMap,
    fs,
    io::{stdin, stdout, Read, Write},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Star {
    position: Point,
    velocity: Point,
}

impl Star {
    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

fn main() {
    let mut stars: Vec<Star> = fs::read_to_string("./src/input")
        .unwrap()
        .lines()
        .map(parse_stars)
        .collect();

    run_stars(&mut stars)
}

fn run_stars(stars: &mut Vec<Star>) {
    let mut best: (i32, HashMap<Point, bool>, i32) = (i32::MAX, HashMap::new(), 0);
    let mut time = 1;

    loop {
        for star in stars.iter_mut() {
            star.step();
        }

        let (size, sky) = get_letter_confidence_v2(&stars);

        if size < best.0 {
            best = (size, sky, time);
        }

        if size > (best.0 * 2) {
            break;
        }
        time += 1;
    }

    print_sky(&best.1);
    println!("Elves waited {}s", &best.2)
}

fn print_sky(sky: &HashMap<Point, bool>) {
    let x_max = sky.keys().map(|p| p.x).max().unwrap() + 5;
    let x_min = sky.keys().map(|p| p.x).min().unwrap() - 5;
    let y_max = sky.keys().map(|p| p.y).max().unwrap() + 5;
    let y_min = sky.keys().map(|p| p.y).min().unwrap() - 5;

    for y in y_min..y_max {
        let mut line = String::new();
        for x in x_min..x_max {
            if sky.contains_key(&Point { x, y }) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn get_letter_confidence_v2(stars: &[Star]) -> (i32, HashMap<Point, bool>) {
    let y_max = stars.iter().map(|p| p.position.y).max().unwrap();
    let y_min = stars.iter().map(|p| p.position.y).min().unwrap();

    let mut sky: HashMap<Point, bool> = HashMap::new();

    for star in stars {
        sky.insert(star.position, true);
    }

    ((y_max - y_min), sky)
}

#[deprecated]
fn get_letter_confidence(stars: &[Star]) -> (i32, HashMap<Point, bool>) {
    let mut sky: HashMap<Point, bool> = HashMap::new();

    for star in stars {
        sky.insert(star.position, true);
    }

    let mut borders = 0;

    for star in &sky {
        for x in -1..=1 {
            for y in -1..=1 {
                let test = Point {
                    x: star.0.x + x,
                    y: star.0.y + y,
                };

                if test != *star.0 && sky.contains_key(&test) {
                    borders += 1;
                }
            }
        }
    }

    return (borders, sky);
}

fn parse_stars(line: &str) -> Star {
    let u_pos = &line[10..];
    let mut pos_parts = u_pos[0..u_pos.find(">").unwrap()].split(",");

    let u_vel = line.split("velocity=<").nth(1).unwrap();
    let mut vel_parts = u_vel[0..u_vel.find(">").unwrap()].split(",");

    return Star {
        position: Point {
            x: pos_parts.next().unwrap().replace(" ", "").parse().unwrap(),
            y: pos_parts.next().unwrap().replace(" ", "").parse().unwrap(),
        },
        velocity: Point {
            x: vel_parts.next().unwrap().replace(" ", "").parse().unwrap(),
            y: vel_parts.next().unwrap().replace(" ", "").parse().unwrap(),
        },
    };
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
