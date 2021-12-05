use std::{
    borrow::Borrow, collections::HashMap, num::ParseIntError, str::FromStr, string::ParseError,
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

#[derive(Clone, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let a: Point = Point::from_str(parts[0]).unwrap();
        let b: Point = Point::from_str(parts[1]).unwrap();

        Ok(Line { a, b })
    }
}

impl Line {
    fn get_points_iter(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        let (a, b) = (self.a.borrow(), self.b.borrow());

        if a.x == b.x {
            return Box::new((a.y.min(b.y)..=a.y.max(b.y)).map(|y| Point { x: a.x, y }));
        } else if a.y == b.y {
            return Box::new((a.x.min(b.x)..=a.x.max(b.x)).map(|x| Point { x, y: a.y }));
        } else {
            return Box::new((0..=(a.x - b.x).abs()).map(|d| Point {
                x: if a.x > b.x { a.x - d } else { a.x + d },
                y: if a.y > b.y { a.y - d } else { a.y + d },
            }));
        }
    }
}

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let vents: Vec<Line> = lines.iter().map(|l| Line::from_str(l).unwrap()).collect();

    let vert_or_horiz: Vec<&Line> = vents
        .iter()
        .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
        .collect();
    let diag: Vec<&Line> = vents
        .iter()
        .filter(|l| l.a.x != l.b.x && l.a.y != l.b.y)
        .collect();

    let mut overlaps: HashMap<Point, u32> = HashMap::new();

    add_overlaps(&vert_or_horiz, &mut overlaps);
    let part_one = overlaps.iter().filter(|(_, v)| **v > 1).count();

    add_overlaps(&diag, &mut overlaps);
    let part_two = overlaps.iter().filter(|(_, v)| **v > 1).count();

    (part_one, part_two)
}

fn add_overlaps(vents: &Vec<&Line>, overlaps: &mut HashMap<Point, u32>) {
    for vent in vents {
        for point in vent.get_points_iter() {
            let entry = overlaps.entry(point).or_insert(0);
            if *entry > 2 {
                continue;
            }
            *entry += 1;
        }
    }
}
