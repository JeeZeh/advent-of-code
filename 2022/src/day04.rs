struct Pair(Zone, Zone);
struct Zone {
    start: u32,
    end: u32,
}

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let pairs: Vec<Pair> = lines.iter().map(|s| parse_pair(s)).collect();
    (
        pairs.iter().filter(|p| p.overlaps_fully()).count(),
        pairs.iter().filter(|p| p.overlaps_partially()).count(),
    )
}

impl Pair {
    fn overlaps_fully(&self) -> bool {
        let left_overlaps_right = self.0.start <= self.1.start && self.0.end >= self.1.end;
        let right_overlaps_left = self.1.start <= self.0.start && self.1.end >= self.0.end;
        left_overlaps_right || right_overlaps_left
    }
    fn overlaps_partially(&self) -> bool {
        self.0.start <= self.1.end && self.0.end >= self.1.start
    }
}

fn parse_pair(s: &str) -> Pair {
    let mut pair_parts = s.split(',');
    let left = parse_zone(pair_parts.next().unwrap());
    let right = parse_zone(pair_parts.next().unwrap());

    Pair(left, right)
}

fn parse_zone(s: &str) -> Zone {
    let mut parts = s.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();

    Zone { start, end }
}
