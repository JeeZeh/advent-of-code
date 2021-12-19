struct Beacon {
    x: i32,
    y: i32,
    z: i32
}

impl Beacon {
    fn from_string(string: &str) -> Self {
        let mut parts = string.split(',');
        Beacon {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }
}

struct Scanner {
    beacons: Vec<Beacon>
}

pub fn solve(lines: String) -> (i32, i32) {
    let scanners: Vec<Scanner> = lines.split("\n\n").map(parse_scanner).collect();
    (0,0)
}

fn parse_scanner(lines: &str) -> Scanner {
    let beacons = lines.lines().collect::<Vec<&str>>()[1..].iter().map(Beacon::from_string).collect_vec()

    Scanner {
        beacons
    }
}