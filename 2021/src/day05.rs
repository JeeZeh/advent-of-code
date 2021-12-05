use std::{collections::HashMap, num::ParseIntError, str::FromStr, string::ParseError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        let x = parts[0].parse()?;
        let y = parts[1].parse()?;
        Ok(Point { x, y })
    }
}

impl Point {
    fn euclid_distance(&self, other: &Self) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    a: Point,
    b: Point,
    length: f32,
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let a: Point = Point::from_str(parts[0]).unwrap();
        let b: Point = Point::from_str(parts[1]).unwrap();
        let length = a.euclid_distance(&b);
        Ok(Line { a, b, length })
    }
}

impl Line {
    fn intersects(&self, point: &Point) -> bool {
        // distance(A, C) + distance(B, C) == distance(A, B);
        self.a.euclid_distance(&point) + self.b.euclid_distance(&point) == self.length
    }
}

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let vents: Vec<Line> = lines.iter().map(|l| Line::from_str(l).unwrap()).collect();

    (part_one(&vents), part_two(&vents))
}

fn part_one(vents: &Vec<Line>) -> usize {
    let vert_or_horiz: Vec<Line> = vents
        .iter()
        .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
        .map(|v| *v)
        .collect();

    let overlaps = get_overlaps(&vert_or_horiz);

    overlaps.iter().filter(|(_, v)| **v > 1).count()
}

fn part_two(vents: &Vec<Line>) -> usize {
    let overlaps = get_overlaps(&vents);

    overlaps.iter().filter(|(_, v)| **v > 1).count()
}

fn get_overlaps(vents: &Vec<Line>) -> HashMap<Point, u32> {
    let ((min_x, min_y), (max_x, max_y)) = get_vent_range(vents);

    let mut overlaps: HashMap<Point, u32> = HashMap::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };

            for vent in vents {
                if vent.intersects(&p) {
                    let entry = overlaps.entry(p).or_insert(0);
                    *entry += 1;
                    // println!("{:?} is in {:?} for {} times", &p, &vent, entry);
                }
            }
        }
    }

    overlaps
}

fn get_vent_range(vents: &Vec<Line>) -> ((usize, usize), (usize, usize)) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for vent in vents {
        xs.push(vent.a.x);
        xs.push(vent.b.x);
        ys.push(vent.a.y);
        ys.push(vent.b.y);
    }

    (
        (
            *xs.iter().min().unwrap() as usize,
            *ys.iter().min().unwrap() as usize,
        ),
        (
            *xs.iter().max().unwrap() as usize,
            *ys.iter().max().unwrap() as usize,
        ),
    )
}
