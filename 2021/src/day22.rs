use std::{collections::HashSet, ops::Range};

use ahash::AHashSet;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    on: bool,
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
    vol: u64,
}

impl Cube {
    fn new(on: bool, x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Cube {
        let vol = ((x).len() * (y).len() * (z).len()) as u64;

        Cube { on, x, y, z, vol }
    }

    fn get_intersection(&self, other: &Self) -> Option<Self> {
        let left_x = self.x.start.max(other.x.start);
        let right_x = self.x.end.min(other.x.end);
        let top_y = self.y.start.max(other.y.start);
        let bottom_y = self.y.end.min(other.y.end);
        let close_z = self.z.start.max(other.z.start);
        let far_z = self.z.end.min(other.z.end);

        if left_x < right_x && top_y < bottom_y && close_z < far_z {
            Some(Cube::new(
                !other.on,
                left_x..right_x,
                top_y..bottom_y,
                close_z..far_z,
            ))
        } else {
            None
        }
    }
}

pub fn solve(lines: Vec<String>) -> (u32, u64) {
    let mut ranges: Vec<Cube> = parse_cubes(&lines);
    let part_two = part_two(&mut ranges);

    // for i in 0..ranges.len() {
    //     dbg!(part_one(&ranges[0..i+1]));
    // }

    // assert_eq!(part_two, 2758514936282235);
    (part_one(&ranges), part_two)
}

fn part_two(cubes: &mut Vec<Cube>) -> u64 {
    let mut total: u64 = 0;

    let mut overlaps: Vec<Cube> = Vec::new();
    for (i, current_cube) in cubes.iter().enumerate() {
        if !current_cube.on {
            continue;
        }
        println!("Adding cube: {:?}", &current_cube);
        total += &current_cube.vol;

        for previous_cube in cubes[0..i].iter() {
            if let Some(intersect) = current_cube.get_intersection(previous_cube) {
                // Don't count overlaps with OFF cubes
                if previous_cube.on {
                    total -= intersect.vol;
                    println!("Overlap created: {:?}", &intersect);
                    overlaps.push(intersect);
                }
            }
        }

        let mut inclusion = false;
        while overlaps.len() > 1 {
            let head = &overlaps[0];
            for other in overlaps[1..].iter() {
                if let Some(intersect) = other.get_intersection(head) {
                    if inclusion {
                        total += intersect.vol;
                        inclusion = false;
                    } else {
                        total -= intersect.vol;
                        inclusion = true;
                    }
                }
            }
            overlaps = overlaps[1..].iter().map(|c| c.clone()).collect_vec();
        }

        dbg!(total);
    }

    total
}

fn part_one(init_steps: &[Cube]) -> u32 {
    let mut count = 0;

    for x in -50..=50 {
        let x_steps = init_steps.iter().filter(|s| s.x.contains(&x)).collect_vec();
        for y in -50..=50 {
            let y_steps = x_steps.iter().filter(|s| s.y.contains(&y)).collect_vec();
            for z in -50..=50 {
                for step in y_steps.iter().filter(|s| s.z.contains(&z)).rev() {
                    if step.x.contains(&x) && step.y.contains(&y) && step.z.contains(&z) {
                        if step.on {
                            count += 1;
                        }
                        break;
                    }
                }
            }
        }
    }

    count
}

fn parse_cubes(lines: &[String]) -> Vec<Cube> {
    let mut cubes = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let (left, right) = line.split_once(" ").unwrap();
        let on = left == "on";

        let mut coords = right.split(',');
        let x_unparsed = coords.next().unwrap();
        let y_unparsed = coords.next().unwrap();
        let z_unparsed = coords.next().unwrap();

        let x_parts = x_unparsed
            .split_once("=")
            .unwrap()
            .1
            .split_once("..")
            .unwrap();
        let y_parts = y_unparsed
            .split_once("=")
            .unwrap()
            .1
            .split_once("..")
            .unwrap();
        let z_parts = z_unparsed
            .split_once("=")
            .unwrap()
            .1
            .split_once("..")
            .unwrap();

        let x = x_parts.0.parse().unwrap()..x_parts.1.parse::<i32>().unwrap() + 1;
        let y = y_parts.0.parse().unwrap()..y_parts.1.parse::<i32>().unwrap() + 1;
        let z = z_parts.0.parse().unwrap()..z_parts.1.parse::<i32>().unwrap() + 1;

        cubes.push(Cube::new(on, x, y, z));
    }
    cubes
}

#[cfg(test)]

mod day22_tests {
    use super::*;

    #[test]
    fn test_cube_intersect() {
        let a = Cube::new(true, 0..10, 0..10, 0..10);
        let b = Cube::new(true, 5..15, 5..15, 5..15);

        let intersect = a.get_intersection(&b).unwrap();

        assert_eq!(intersect.x, 5..10);
        assert_eq!(intersect.y, 5..10);
        assert_eq!(intersect.z, 5..10);

        assert_eq!(intersect.vol, 5 * 5 * 5);
    }
}
