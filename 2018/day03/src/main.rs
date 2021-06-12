use std::{collections::HashMap, fs};

struct Pos {
    x: i32,
    y: i32,
}
struct Dimensions {
    x: i32,
    y: i32,
}
struct Claim {
    id: String,
    pos: Pos,
    dimensions: Dimensions,
}

fn main() {
    println!("Hello, world!");
    let file = fs::read_to_string("./src/input").unwrap();
    let claims: Vec<Claim> = file.lines().map(parse_claim).collect();

    let mut plot: HashMap<String, i32> = HashMap::new();
    for claim in claims.iter() {
        for x in claim.pos.x..claim.pos.x + claim.dimensions.x {
            for y in claim.pos.y..claim.pos.y + claim.dimensions.y {
                let pos = format!("{},{}", x, y);
                let mut num_claims = 1;
                if plot.contains_key(&pos) {
                    num_claims += plot.get(&pos).unwrap();
                }
                plot.insert(pos, num_claims);
            }
        }
    }

    let more_than_two_claims = plot.values().filter(|c| c >= &&2).count();
    println!("Part 1: {}", more_than_two_claims);

    let not_overlapping: &Claim = claims.iter().find(|c| !claim_overlaps(&plot, c)).unwrap();
    println!("Part 2: {}", not_overlapping.id)
}

fn claim_overlaps(plot: &HashMap<String, i32>, claim: &Claim) -> bool {
    for x in claim.pos.x..claim.pos.x + claim.dimensions.x {
        for y in claim.pos.y..claim.pos.y + claim.dimensions.y {
            let pos = format!("{},{}", x, y);
            if plot.get(&pos).unwrap() > &1 {
                return true;
            };
        }
    }
    return false;
}

fn parse_claim(line: &str) -> Claim {
    let mut split = line.split(" ");
    let id = split.next().unwrap();
    let positions = split.nth(1).unwrap(); // 141,223:
    let dimensions = split.next().unwrap(); // 19x12

    let positions: Vec<i32> = positions[..positions.len() - 1]
        .split(",")
        .map(|p| p.parse().unwrap())
        .collect();

    let dimensions: Vec<i32> = dimensions.split("x").map(|d| d.parse().unwrap()).collect();

    Claim {
        id: String::from(id),
        pos: Pos {
            x: positions[0],
            y: positions[1],
        },
        dimensions: Dimensions {
            x: dimensions[0],
            y: dimensions[1],
        },
    }
}
