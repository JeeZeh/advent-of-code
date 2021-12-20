use std::collections::HashSet;

use itertools::{zip, Itertools};

const MIN_BEACON_INTERSECTS: usize = 11;
const MIN_BEACON_OVERLAPS: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);

#[derive(Debug, Clone)]
struct Beacon {
    position: Point,
    relative_vectors: HashSet<Point>,
}

impl Beacon {
    fn is_same(&self, other: &Self) -> bool {
        let intersection = self
            .relative_vectors
            .intersection(&other.relative_vectors)
            .count();

        intersection >= MIN_BEACON_INTERSECTS
    }
}

impl Point {
    fn from_string(string: &str) -> Self {
        let mut parts = string.split(',');
        Point(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }

    fn roll(&self) -> Point {
        Point(self.0, self.2, -self.1)
    }

    fn turn(&self) -> Point {
        Point(-self.1, self.0, self.2)
    }

    fn special_rtr_roll(&self) -> Self {
        self.roll().turn().roll().roll()
    }

    fn relative_vector(&self, other: &Self) -> Point {
        Point(other.0 - self.0, other.1 - self.1, other.2 - self.2)
    }

    fn manhattan(&self, other: &Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) as usize
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
    orientation: usize,
    merged: Vec<Point>,
}

impl Scanner {
    // Great SO answer for generating orientations https://stackoverflow.com/a/16467849
    fn next_orientation(&self) -> Option<Scanner> {
        if self.orientation == 23 {
            return None;
        }

        let op: fn(&Point) -> Point;

        // Edge case to first perform RTR before the operation
        if self.orientation == 11 {
            op = Point::special_rtr_roll
        } else {
            op = match self.orientation % 4 {
                0 => Point::roll,
                _ => Point::turn,
            };
        }

        let mut new_beacons = Vec::new();

        for beacon in &self.beacons {
            let mut new_relative_vectors = HashSet::new();

            beacon.relative_vectors.iter().for_each(|p| {
                new_relative_vectors.insert(op(p));
            });

            new_beacons.push(Beacon {
                position: op(&beacon.position),
                relative_vectors: new_relative_vectors,
            });
        }

        Some(Scanner {
            beacons: new_beacons,
            orientation: (self.orientation + 1) % 24,
            merged: self.merged.clone(),
        })
    }

    fn try_match(&self, other: &Self) -> Option<(Vec<(Point, Point)>, Scanner)> {
        let mut other_reoriented = other.clone();
        loop {
            let mut matching_beacons = Vec::new();

            for a in &self.beacons {
                for b in &other_reoriented.beacons {
                    if a.is_same(b) {
                        matching_beacons.push((a.position, b.position));
                    }
                }
            }

            if matching_beacons.len() >= MIN_BEACON_OVERLAPS {
                return Some((matching_beacons, other_reoriented));
            }

            if let Some(reorient) = other_reoriented.next_orientation() {
                other_reoriented = reorient;
            } else {
                break;
            }
        }
        None
    }

    fn try_combine(&self, other: &Self) -> Option<Self> {
        if let Some((matching_beacons, other_orientation)) = self.try_match(other) {
            let mut copy = self.clone();
            let (sample_a, sample_b) = matching_beacons[0];
            let translation = sample_a.relative_vector(&sample_b);
            let matched_from_other: HashSet<Point> = matching_beacons.iter().map(|p| p.1).collect();

            for beacon in &other_orientation.beacons {
                if !matched_from_other.contains(&beacon.position) {
                    let mut migrated_beacon = Beacon {
                        position: translation.relative_vector(&beacon.position),
                        relative_vectors: HashSet::new(),
                    };

                    // Update relative vectors
                    for to_update in copy.beacons.iter_mut() {
                        let rel_b = migrated_beacon
                            .position
                            .relative_vector(&to_update.position);
                        let rel_a = to_update
                            .position
                            .relative_vector(&migrated_beacon.position);
                        to_update.relative_vectors.insert(rel_a);
                        migrated_beacon.relative_vectors.insert(rel_b);
                    }

                    copy.beacons.push(migrated_beacon);
                }

                copy.merged.push(translation);
            }
            return Some(copy);
        }

        None
    }

    fn largest_manhattan(&self) -> usize {
        let mut max = 0;
        for (i, a) in self.merged.iter().enumerate() {
            for b in self.merged[i + 1..].iter() {
                let man = a.manhattan(&b);
                if man > max {
                    max = man;
                }
            }
        }
        return max;
    }
}

pub fn solve(lines: String) -> (usize, usize) {
    let mut scanners: Vec<Scanner> = lines.split("\n\n").map(parse_scanner).collect();

    while scanners.len() > 1 {
        // println!("{} remaining...", scanners.len() - 1);
        scanners = find_match(&scanners);
    }

    (scanners[0].beacons.len(), scanners[0].largest_manhattan())
}

fn find_match(scanners: &Vec<Scanner>) -> Vec<Scanner> {
    let mut remaining = Vec::new();
    let mut composite_scanner = scanners[0].clone();
    remaining.push(scanners[0].clone());

    for other in scanners[1..].iter() {
        if let Some(combined) = composite_scanner.try_combine(&other) {
            // println!("Successfully combined with {}", i + 1);
            composite_scanner = combined;
        } else {
            remaining.push(other.clone());
        }
    }

    remaining[0] = composite_scanner;
    remaining
}

fn parse_scanner(lines: &str) -> Scanner {
    let points = lines.lines().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|l| Point::from_string(*l))
        .collect_vec();

    let mut points_relative_vectors: Vec<HashSet<Point>> = vec![HashSet::new(); points.len()];

    for (i, a) in points.iter().enumerate() {
        for (j, b) in points[i + 1..].iter().enumerate() {
            points_relative_vectors[i].insert(a.relative_vector(b));
            points_relative_vectors[i + 1 + j].insert(b.relative_vector(a));
        }
    }

    let mut beacons: Vec<Beacon> = Vec::new();
    for (position, relative_vectors) in zip(points, points_relative_vectors) {
        beacons.push(Beacon {
            position,
            relative_vectors,
        })
    }

    Scanner {
        beacons,
        orientation: 0,
        merged: Vec::new(),
    }
}
