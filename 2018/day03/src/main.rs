use std::{fs, time::Instant};

struct Pos {
    x: u16,
    y: u16,
}
struct Dimensions {
    x: u16,
    y: u16,
}
struct Claim {
    id: String,
    pos: Pos,
    dimensions: Dimensions,
}

fn main() {
    let now = Instant::now();
    let file = fs::read_to_string("./src/input").unwrap();
    let claims: Vec<Claim> = file.lines().map(parse_claim).collect();

    let mut plot = vec![vec![0u32; 1000]; 1000];

    for claim in claims.iter() {
        for x in claim.pos.x..claim.pos.x + claim.dimensions.x {
            for y in claim.pos.y..claim.pos.y + claim.dimensions.y {
                plot[usize::from(y)][usize::from(x)] += 1;
            }
        }
    }
    let at_least_two = count_squares_with_at_least_two_claims(&plot);
    println!("Part 1: {}", at_least_two);

    let not_overlapping: &Claim = claims.iter().find(|c| !claim_overlaps(&plot, c)).unwrap();
    println!("Part 2: {}", not_overlapping.id);

    println!("{}ms", now.elapsed().as_millis());
}

fn count_squares_with_at_least_two_claims(plot: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if plot[y][x] >= 2 {
                count += 1;
            }
        }
    }

    count
}

fn claim_overlaps(plot: &Vec<Vec<u32>>, claim: &Claim) -> bool {
    for x in claim.pos.x..claim.pos.x + claim.dimensions.x {
        for y in claim.pos.y..claim.pos.y + claim.dimensions.y {
            if plot[usize::from(y)][usize::from(x)] > 1 {
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

    let positions: Vec<u16> = positions[..positions.len() - 1]
        .split(",")
        .map(|p| p.parse().unwrap())
        .collect();

    let dimensions: Vec<u16> = dimensions.split("x").map(|d| d.parse().unwrap()).collect();

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
