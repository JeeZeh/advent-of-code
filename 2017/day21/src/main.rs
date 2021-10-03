use core::panic;
use std::collections::HashMap;

fn main() {
    let rules = read_rules(false);
    let mut current_tile = Tile::from_pattern(".#./..#/###");

    let part_one = 5;
    for _ in 0..part_one {
        current_tile = current_tile.next(&rules);
    }
    println!(
        "After {} iterations there are {} pixels on",
        part_one,
        current_tile.pixels_on()
    );

    // Continue the iterations from Part One
    let part_two = 18;
    for _ in part_one..part_two {
        current_tile = current_tile.next(&rules);
    }
    println!(
        "After {} iterations there are {} pixels on",
        part_two,
        current_tile.pixels_on()
    )
}

// ==== Helpers ====

fn read_rules(test: bool) -> HashMap<String, Tile> {
    let input = std::fs::read_to_string(if test { "./src/test" } else { "./src/input" });
    let mut rules: HashMap<String, Tile> = HashMap::new();
    for line in input.unwrap().lines() {
        let (rule_input, rule_output) = parse_rule(line);
        rules.insert(rule_input.as_pattern(), rule_output);
    }

    return rules;
}

fn parse_rule(line: &str) -> (Tile, Tile) {
    let mut parts = line.split(" => ");

    let rule = parts.next().unwrap();
    let output = parts.next().unwrap();

    (Tile::from_pattern(rule), Tile::from_pattern(output))
}

// ==== Structs ====

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    data: Vec<Vec<bool>>,
    size: usize,
}

impl Tile {
    fn from_pattern(patt: &str) -> Tile {
        let parts: Vec<&str> = patt.split("/").collect();
        let size = parts.len();
        let mut rows: Vec<Vec<bool>> = Vec::with_capacity(size);

        for part in parts {
            let mut row = Vec::with_capacity(size);
            for c in part.chars() {
                match c {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    _ => panic!("Found char that wasn't '.' or '#' {}", c),
                };
            }
            rows.push(row);
        }

        return Tile { data: rows, size };
    }

    fn as_pattern(&self) -> String {
        let mut out = String::with_capacity((self.size * self.size) + self.size - 1);

        for (i, row) in self.data.as_slice().iter().enumerate() {
            for b in row {
                let ch = match b {
                    true => '#',
                    false => '.',
                };
                out.push(ch);
            }
            if i < self.size - 1 {
                out.push('/');
            }
        }

        return out;
    }

    fn as_ascii(&self) -> String {
        self.as_pattern().replace("/", "\n")
    }

    fn pixels_on(&self) -> usize {
        return self
            .as_pattern()
            .chars()
            .map(|c| if c == '#' { 1 } else { 0 })
            .sum();
    }

    fn rot90(&self) -> Tile {
        let mut new_data: Vec<Vec<bool>> = Vec::with_capacity(self.size);

        for x in 0..self.size {
            let mut row: Vec<bool> = Vec::with_capacity(self.size);
            for y in 0..self.size {
                row.push(self.data[(self.size - 1) - y][x]);
            }
            new_data.push(row);
        }

        return Tile {
            size: self.size,
            data: new_data,
        };
    }

    fn flip_y(&self) -> Tile {
        let mut data = self.clone().data;
        data.reverse();

        return Tile {
            size: self.size,
            data,
        };
    }

    fn permutations(&self) -> Vec<Tile> {
        let mut perms = Vec::with_capacity(8);
        perms.push(self.clone());

        let mut current_tile = self.clone();

        for _ in 0..2 {
            for _ in 0..3 {
                current_tile = current_tile.rot90();
                perms.push(current_tile.clone());
            }
            current_tile = self.flip_y();
            perms.push(current_tile.clone())
        }

        return perms;
    }

    fn enhance(&self, rules: &HashMap<String, Tile>) -> Tile {
        for perm in self.permutations() {
            let test = rules.get(&perm.as_pattern());
            if test.is_some() {
                return test.unwrap().clone();
            }
        }

        panic!("Never found a matching rule for {}", self.as_ascii());
    }

    fn cut(&self) -> Vec<Vec<Tile>> {
        let divisor = match self.size % 2 {
            0 => 2,
            _ => 3,
        };
        let parts = self.size / divisor;

        let mut super_rows = Vec::with_capacity(parts);

        for start_y in (0..self.size).step_by(divisor) {
            let mut super_row = Vec::with_capacity(parts);
            for start_x in (0..self.size).step_by(divisor) {
                let mut sub_rows = Vec::with_capacity(divisor);
                for y in start_y..start_y + divisor {
                    let mut sub_row = Vec::with_capacity(divisor);
                    for x in start_x..start_x + divisor {
                        sub_row.push(self.data[y][x]);
                    }
                    sub_rows.push(sub_row);
                }
                super_row.push(Tile {
                    size: divisor,
                    data: sub_rows,
                });
            }
            super_rows.push(super_row);
        }
        return super_rows;
    }

    fn stitch(tiles: Vec<Vec<Tile>>) -> Tile {
        let in_size = tiles.len();
        let sector_size = tiles[0][0].size;
        let out_size = in_size * sector_size;

        let mut data = Vec::with_capacity(out_size);
        for y in 0..out_size {
            let mut row = Vec::with_capacity(out_size);
            for x in 0..out_size {
                let v =
                    tiles[y / sector_size][x / sector_size].data[y % sector_size][x % sector_size];
                row.push(v);
            }
            data.push(row);
        }

        return Tile {
            size: out_size,
            data,
        };
    }

    fn next(&self, rules: &HashMap<String, Tile>) -> Tile {
        let mut parts = self.cut();

        for y in 0..parts.len() {
            for x in 0..parts[y].len() {
                parts[y][x] = parts[y][x].enhance(rules);
            }
        }

        return Tile::stitch(parts);
    }
}

// ==== Tests ====

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_tile_cut() {
        let tile = Tile::from_pattern("#..#/..../..../#..#");
        let parts = tile.cut();

        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].len(), 2);

        assert_eq!(parts[0][0].as_pattern(), "#./..");
        assert_eq!(parts[0][1].as_pattern(), ".#/..");
        assert_eq!(parts[1][0].as_pattern(), "../#.");
        assert_eq!(parts[1][1].as_pattern(), "../.#");
    }

    #[test]
    fn test_tile_stitch() {
        let original_tile = Tile::from_pattern("#..#/..../..../#..#");
        let cut = original_tile.cut();
        assert_eq!(Tile::stitch(cut).as_pattern(), original_tile.as_pattern());
    }

    #[test]
    fn test_tile_flip_y() {
        let tile = Tile::from_pattern(".#./..#/###").flip_y();
        assert_eq!(tile.as_pattern(), "###/..#/.#.");
    }

    #[test]
    fn test_tile_rot90() {
        let tile = Tile::from_pattern(".#./..#/###").rot90();
        assert_eq!(tile.as_pattern(), "#../#.#/##.");
    }

    #[test]
    fn test_tile_as_ascii() {
        let tile = Tile::from_pattern(".#./..#/###");
        assert_eq!(
            tile.as_ascii(),
            indoc! {"
                .#.
                ..#
                ###"
            }
        );
    }

    #[test]
    fn test_tile_permutations() {
        let tile = Tile::from_pattern(".#./..#/###");
        let permutations: Vec<Tile> = tile.permutations();
        assert!(permutations.contains(&tile));

        // #..
        // #.#
        // ##.
        let flip_0_rot_1 = "#../#.#/##.";
        assert!(permutations.contains(&Tile::from_pattern(flip_0_rot_1)));

        // ###
        // #..
        // .#.
        let flip_0_rot_2 = "###/#../.#.";
        assert!(permutations.contains(&Tile::from_pattern(flip_0_rot_2)));

        // .##
        // #.#
        // ..#
        let flip_0_rot_3 = ".##/#.#/..#";
        assert!(permutations.contains(&Tile::from_pattern(flip_0_rot_3)));

        // ###
        // ..#
        // .#.
        let flip_1_rot_0 = "###/..#/.#.";
        assert!(permutations.contains(&Tile::from_pattern(flip_1_rot_0)));

        // ..#
        // #.#
        // .##
        let flip_1_rot_1 = "..#/#.#/.##";
        assert!(permutations.contains(&Tile::from_pattern(flip_1_rot_1)));

        // .#.
        // #..
        // ###
        let flip_1_rot_2 = ".#./#../###";
        assert!(permutations.contains(&Tile::from_pattern(flip_1_rot_2)));

        // ##.
        // #.#
        // #..
        let flip_1_rot_3 = "##./#.#/#..";
        assert!(permutations.contains(&Tile::from_pattern(flip_1_rot_3)));
    }
}
