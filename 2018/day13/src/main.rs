use std::{collections::HashMap, fs, ops::Index};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cart {
    pos: Pos,
    dir: u8,
    next_turn: u8,
}

impl Cart {
    fn step(&mut self, track: &HashMap<Pos, char>) {
        // Move forward
        match self.dir {
            0 => self.pos.y -= 1,
            1 => self.pos.x += 1,
            2 => self.pos.y += 1,
            3 => self.pos.x -= 1,
            _ => panic!("Unknown direction!"),
        }

        if !track.contains_key(&self.pos) {
            panic!("Cart@{:?} moved off track!", self.pos);
        }

        // Rotate
        match track.get(&self.pos).unwrap() {
            '/' => {
                let rot = if self.dir % 2 == 0 { 1 } else { -1 };
                self.dir = (self.dir as i32 + rot).rem_euclid(4) as u8;
            }
            '\\' => {
                let rot = if self.dir % 2 == 0 { -1 } else { 1 };
                self.dir = (self.dir as i32 + rot).rem_euclid(4) as u8;
            }
            '+' => {
                // Straight if next_turn is 1
                if self.next_turn == 1 {
                    return;
                }

                // Left if next_turn is 0, right if 2
                let rot = if self.next_turn == 0 { -1 } else { 1 };
                self.dir = (self.dir as i32 + rot).rem_euclid(4) as u8;
            }
            _ => (),
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/test").unwrap();
    let (track, mut carts) = parse_track(&input);

    println!("{:?}", carts);

    for _ in 0..10 {
        carts.iter_mut().for_each(|c| c.step(&track));
        println!("{:?}", carts);
    }
}

fn parse_track(input: &str) -> (HashMap<Pos, char>, Vec<Cart>) {
    let mut track = HashMap::new();
    let mut carts = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == ' ' {
                continue;
            }
            let mut write_char = ' ';

            if ['\\', '/', '|', '-', '+'].contains(&char) {
                write_char = char;
            } else {
                let dir = ['^', '>', 'v', '<']
                    .iter()
                    .position(|c| c == &char)
                    .unwrap() as u8;
                let cart = Cart {
                    pos: Pos { x, y },
                    dir,
                    next_turn: 0,
                };
                carts.push(cart);

                if char == '<' || char == '>' {
                    write_char = '-';
                } else if char == '^' || char == 'v' {
                    write_char = '|';
                }
            }

            track.insert(Pos { x, y }, write_char);
        }
    }

    (track, carts)
}
