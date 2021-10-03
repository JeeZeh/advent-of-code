use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/test");
    let mut rules: HashMap<String, Tile> = HashMap::new();
    for line in input.unwrap().lines() {
        let (rule_input, rule_output) = parse_rule(line);
        rules.insert(rule_input.as_pattern(), rule_output);
        println!("{}", rule_input.as_pattern());
    }
}

#[derive(Debug)]
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
}

fn parse_rule(line: &str) -> (Tile, Tile) {
    let mut parts = line.split(" => ");

    let rule = parts.next().unwrap();
    let output = parts.next().unwrap();

    (Tile::from_pattern(rule), Tile::from_pattern(output))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_rotations() {
        let tile = Tile::from_pattern(".#./..#/###");
        // .#.
        // ..#
        // ###

        // #..
        // #.#
        // ##.

        // ###
        // #..
        // .#.

        // .##
        // #.#
        // ..#

        // ###
        // ..#
        // .#.

        // ..#
        // #.#
        // .##

        // .#.
        // #..
        // ###

        // ##.
        // #.#
        // #..
        
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
}

