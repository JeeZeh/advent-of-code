use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, AddAssign},
};

type Directions = Vec<Pos>;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Pos(i32, i32);

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, PartialEq)]
enum TileType {
    Vertical,
    Horizontal,
    Corner,
    Letter,
}

#[derive(Debug)]
struct Tile {
    char: char,
    directions: Option<Directions>,
    type_: TileType,
}

type Diagram = HashMap<Pos, Tile>;

fn main() {
    let file = fs::read_to_string("./src/input").unwrap();
    let diagram = parse_diagram(file.as_str());
    let start = diagram
        .iter()
        .find(|(pos, tile)| pos.1 == 0 && tile.char == '|')
        .unwrap();

    let (part_one, part_two) = traverse(&diagram, start.0);

    println!("Visited {} in {} steps", part_one, part_two)
}

fn traverse(diagram: &Diagram, start: &Pos) -> (String, i32) {
    let mut steps = 1;
    let mut letters = String::new();
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut current = start.clone();
    seen.insert(current.clone());

    let mut heading = Some(Pos(0, 1));

    loop {
        seen.insert(current.clone());
        let tile = diagram.get(&current).unwrap();
        // println!("Looking at {:?} in diagram ({})", current, tile.char);

        // We can go places (not a letter), change direction
        if tile.type_ == TileType::Letter {
            letters.push(tile.char);
        } else {
            heading = tile
                .directions
                .as_deref()
                .unwrap()
                .iter()
                .find(|d| {
                    let new_dir = *d.clone() + current.clone();
                    diagram.contains_key(&new_dir)
                        && heading.unwrap() + *d.clone() != Pos(0, 0)
                        && (!seen.contains(&new_dir) // If we didn't visit
                            || diagram.get(&new_dir).unwrap().type_ != tile.type_) // or if we did make sure it's gonna be a jump
                })
                .copied();
        }

        let next = diagram.get(&(current + heading.unwrap()));

        if next.is_none() {
            break;
        }

        let next = next.unwrap();

        // println!("Next is {:?}", next);
        // If the next tile is perpendicular, hop over it
        if next.type_ != TileType::Letter
            && !next
                .directions
                .as_deref()
                .unwrap()
                .contains(&heading.unwrap())
        {
            steps += 1;
            current += heading.unwrap();
        }
        steps += 1;
        current += heading.unwrap();
    }

    return (letters, steps);
}

fn parse_diagram(file: &str) -> Diagram {
    let mut diagram: HashMap<Pos, Tile> = HashMap::new();

    for (y, line) in file.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let type_: TileType = match char {
                '|' => TileType::Vertical,
                '-' => TileType::Horizontal,
                '+' => TileType::Corner,
                ' ' => continue,
                _ => TileType::Letter,
            };
            let directions: Option<Directions> = match char {
                '|' => Some(Vec::from([Pos(0, 1), Pos(0, -1)])),
                '-' => Some(Vec::from([Pos(-1, 0), Pos(1, 0)])),
                '+' => Some(Vec::from([Pos(0, 1), Pos(0, -1), Pos(-1, 0), Pos(1, 0)])),
                ' ' => continue,
                _ => None,
            };

            diagram.insert(
                Pos(x as i32, y as i32),
                Tile {
                    char,
                    directions,
                    type_,
                },
            );
        }
    }

    return diagram;
}
