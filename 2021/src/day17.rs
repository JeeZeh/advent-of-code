use std::ops::Range;

#[derive(Debug)]
struct Drone {
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Drone {
    fn with_vel(vel_x: i32, vel_y: i32) -> Self {
        Drone {
            pos_x: 0,
            pos_y: 0,
            vel_x,
            vel_y,
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
    }
}

pub fn solve(string: String) -> (i32, i32) {
    let (target_x, target_y) = parse_targets(string);

    find_all_vels(&target_x, &target_y)
}

fn find_all_vels(target_x: &Range<i32>, target_y: &Range<i32>) -> (i32, i32) {
    let mut all_vels = 0;

    let min_x = (-1 + (1.00 + 8.00 * target_x.start as f32).sqrt() as i32) / 2;

    for vx in min_x..target_x.end {
        let vy_range = if vx > target_x.start {
            // Use this bound if we're immediately landing in the target (y vel must be inside the target)
            target_y.start..target_y.end
        } else {
            // I'm sure you can bound this more, given the number of steps it takes to reach the target
            // in the x direction you should be able to determine at least the minimum bound
            target_y.start..target_y.start.abs()
        };
        for vy in vy_range {
            if fire(vx, vy, target_x, target_y) {
                all_vels += 1;
            }
        }
    }

    (
        target_y.start.abs() * (target_y.start.abs() - 1) / 2,
        all_vels,
    )
}

fn fire(vel_x: i32, vel_y: i32, target_x: &Range<i32>, target_y: &Range<i32>) -> bool {
    let mut drone = Drone::with_vel(vel_x, vel_y);

    while !(drone.pos_x < target_x.start && drone.vel_x == 0)
        && !(drone.pos_y < target_y.start)
        && !(drone.pos_x > target_x.end)
    {
        if target_x.contains(&drone.pos_x) && target_y.contains(&drone.pos_y) {
            return true;
        }

        drone.step();
    }

    return false;
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
