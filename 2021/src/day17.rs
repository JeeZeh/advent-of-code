use std::{collections::HashSet, ops::Range};

use itertools::Itertools;

#[derive(Debug)]
struct Drone {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
    max_y: i32,
}

impl Drone {
    fn with_vel(vel_x: i32, vel_y: i32) -> Self {
        Drone {
            pos_x: 0,
            pos_y: 0,
            vel_x,
            vel_y,
            max_y: 0,
        }
    }

    fn step(&mut self) {
        // The probe's x position increases by its x velocity.
        self.pos_x += self.vel_x;
        // The probe's y position increases by its y velocity.
        self.pos_y += self.vel_y;
        // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
        self.vel_x = 0.max(self.vel_x + (if self.vel_x < 0 { 1 } else { -1 }));
        // Due to gravity, the probe's y velocity decreases by 1.
        self.vel_y -= 1;
        if self.pos_y > self.max_y {
            self.max_y = self.pos_y;
        }
    }
}

pub fn solve(string: String) -> (i32, i32) {
    let (target_x, target_y) = parse_targets(string);

    find_all_vels(&target_x, &target_y)
}

fn find_all_vels(target_x: &Range<i32>, target_y: &Range<i32>) -> (i32, i32) {
    let mut all_vels: Vec<(i32, i32, i32)> = Vec::new();

    for vx in 0..500 {
        for vy in -300..250 {
            let (success, drone) = fire(vx, vy, target_x, target_y);
            if success {
                all_vels.push((vx, vy, drone.max_y));
            }
        }
    }

    // dbg!(&all_vels);
    (
        all_vels.iter().map(|(_, _, h)| *h).max().unwrap(),
        all_vels.len() as i32,
    )
}
fn maximise_vy(target_x: &Range<i32>, target_y: &Range<i32>) -> i32 {
    let mut vx = 0;
    let mut vy = 0;

    let mut max_height = i32::MIN;

    while vy < 150 {
        let (success, drone) = fire(vx, vy, target_x, target_y);
        if success {
            max_height = drone.max_y;
        }

        if drone.pos_x < target_x.start {
            vx += 1;
        } else if drone.pos_x > target_x.end {
            vx -= 1;
        } else {
            vy += 1;
        }
    }

    max_height
}

fn fire(vel_x: i32, vel_y: i32, target_x: &Range<i32>, target_y: &Range<i32>) -> (bool, Drone) {
    let mut drone = Drone::with_vel(vel_x, vel_y);

    while !(drone.pos_x < target_x.start && drone.vel_x == 0) && !(drone.pos_y < target_y.start) {
        if target_x.contains(&drone.pos_x) && target_y.contains(&drone.pos_y) {
            return (true, drone);
        }

        // dbg!(&drone);

        drone.step();
    }

    return (false, drone);
}

fn parse_targets(string: String) -> (Range<i32>, Range<i32>) {
    let (_, parts) = string.split_once(": ").unwrap();
    let (x, y) = parts.split_once(", ").unwrap();

    let target_x: Range<i32> = x[2..]
        .split_once("..")
        .map(|(a, b)| (a.parse().unwrap()..b.parse::<i32>().unwrap() + 1))
        .unwrap();
    let target_y: Range<i32> = y[2..]
        .split_once("..")
        .map(|(a, b)| (a.parse().unwrap()..b.parse::<i32>().unwrap() + 1))
        .unwrap();

    (target_x, target_y)
}
