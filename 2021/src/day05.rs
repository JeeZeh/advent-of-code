use std::{
    collections::HashMap, f64::EPSILON, num::ParseIntError, str::FromStr, string::ParseError,
};

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
    fn euclid_distance(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    a: Point,
    b: Point,
    length: f64,
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
        let diff = self.a.euclid_distance(&point) + self.b.euclid_distance(&point) - self.length;
        -0.000005 < diff && diff < 0.000005 // Horrible precision errors
    }
}

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    test_point_intersection();

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

    // let mut output = String::new();
    // for y in 0..10 {
    //     let mut row = String::new();
    //     for x in 0..10 {
    //         if let Some(val) = overlaps.get(&Point { x, y }) {
    //             row.push_str(format!("{}", val).as_str());
    //         } else {
    //             row.push('.');
    //         }
    //     }
    //     output.push_str(row.as_str());
    //     output.push('\n');
    // }
    // println!("{}", output);
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

fn test_point_intersection() {
    let line = Line::from_str("0,0 -> 500,500").unwrap();
    let back_line = Line::from_str("500,500 -> 0,0").unwrap();
    let short = Line::from_str("100,100 -> 101,101").unwrap();
    let weird = Line::from_str("433,296 -> 148,581").unwrap();

    assert!(line.intersects(&Point { x: 250, y: 250 }));
    assert!(back_line.intersects(&Point { x: 250, y: 250 }));
    assert!(short.intersects(&Point { x: 100, y: 100 }));
    assert!(short.intersects(&Point { x: 101, y: 101 }));
    assert!(weird.intersects(&Point { x: 283, y: 446 }));
}
