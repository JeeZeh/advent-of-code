use std::{borrow::Borrow, collections::HashMap, fs};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Cart {
    id: i32,
    pos: Pos,
    dir: u8,
    next_turn: u8,
}

impl Cart {
    fn step(&mut self) {
        // println!("{:?}", self);
        match self.dir {
            0 => self.pos.y -= 1,
            1 => self.pos.x += 1,
            2 => self.pos.y += 1,
            3 => self.pos.x -= 1,
            _ => panic!("Unknown direction!"),
        };
    }

    fn rotate(&mut self, standing_on: char) {
        match standing_on {
            '/' => {
                let rot = if self.dir % 2 == 0 { 1 } else { -1 };
                self.dir = (self.dir as i32 + rot).rem_euclid(4) as u8;
            }
            '\\' => {
                let rot = if self.dir % 2 == 0 { -1 } else { 1 };
                self.dir = (self.dir as i32 + rot).rem_euclid(4) as u8;
            }
            '+' => {
                if self.next_turn != 1 {
                    // Left if next_turn is 0, right if 2
                    let rot = if self.next_turn == 0 { -1 } else { 1 };
                    self.dir = (self.dir as i32 + rot).rem_euclid(4) as u8;
                }

                // Go straight if next_turn is 1
                self.next_turn = (self.next_turn + 1) % 3;
            }
            _ => (),
        };
    }
}

#[derive(Debug)]
struct Tile {
    cart: Option<Cart>,
    type_: char,
}

struct World {
    track: Vec<Vec<Tile>>,
}

impl World {
    fn tick(&mut self) -> usize {
        let mut moved = HashMap::new();

        for y in 0..self.track.len() {
            for x in 0..self.track[y].len() {
                let tile = self.track[y][x].borrow();

                if tile.cart.is_none() {
                    continue;
                }

                // A copy of the cart instance at the tile position
                let mut cart = tile.cart.unwrap();

                if moved.contains_key(&cart.id) {
                    continue;
                }

                moved.insert(cart.id, cart.pos);

                // Move forward
                cart.step();

                let new_tile = self.track[cart.pos.y][cart.pos.x].borrow();

                // New position is not a valid tile
                if new_tile.type_ == ' ' {
                    panic!("Cart@{:?} moved off track!", cart);
                }

                // There is already a cart at the position we tried to move to
                // BANG!
                if new_tile.cart.is_some() {
                    println!(
                        "Found collision at {}",
                        format!("[{}, {}]", cart.pos.x, cart.pos.y)
                    );

                    // Remove the cart we just crashed into
                    self.track[cart.pos.y][cart.pos.x].cart = None;
                } else {
                    // Perform rotation if necessary
                    cart.rotate(new_tile.type_);

                    // Insert at the new position
                    self.track[cart.pos.y][cart.pos.x].cart = Some(cart);
                }

                // Remove from the old position
                self.track[y][x].cart = None;
            }
        }

        // Only one cart moved (it's the last one)
        if moved.len() == 1 {
            let final_pos = moved.values().next().unwrap();
            println!(
                "Only one cart remaining at position {},{}",
                final_pos.x, final_pos.y
            );
        };

        moved.len()
    }
}

fn main() {
    let input = fs::read_to_string("./src/input").unwrap();
    let mut world = parse_track(&input);

    loop {
        if world.tick() == 1 {
            break;
        };
    }
}

fn parse_track(input: &str) -> World {
    let mut track = Vec::with_capacity(input.lines().count());
    let mut cart_id = 0;

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            let mut cart: Option<Cart> = None;

            let mut type_ = c;

            // We found a cart
            if ['^', '>', 'v', '<'].contains(&c) {
                let dir = ['^', '>', 'v', '<'].iter().position(|d| d == &c).unwrap() as u8;

                cart = Some(Cart {
                    id: cart_id,
                    pos: Pos { x, y },
                    dir,
                    next_turn: 0,
                });

                cart_id += 1;

                // Infer the tile the cart is sitting on
                if c == '<' || c == '>' {
                    type_ = '-';
                } else {
                    type_ = '|';
                }
            }

            row.push(Tile { type_, cart });
        }
        track.push(row);
    }

    return World { track };
}
