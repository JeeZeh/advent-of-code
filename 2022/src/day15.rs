use itertools::Itertools;

pub fn solve(input: String) -> (usize, i64) {
    let sensors = input.lines().map(Sensor::from).collect_vec();
    // sensors
    //     .iter()
    //     .map(|s| (s, s.get_coverage_at_y(8)))
    //     .for_each(|s| println!("{:?}", s));

    let beacon = find_beacon(&sensors, 4000000);
    (
        beacon_exclusion_zone(&sensors, 2000000),
        (beacon.0 as i64 * 4000000) + beacon.1 as i64,
    )
}

fn find_beacon(sensors: &[Sensor], bound: i32) -> (i32, i32) {
    // Hint from reddit: you just need to find the 2 pairs of beacons
    // that are separated by their own radius + 2 (1 space between them)
    let mut bounding_sensors = Vec::new();
    for i in 0..sensors.len() {
        let a = sensors[i];
        for j in i + 1..sensors.len() {
            let b = sensors[j];
            let sensor_distance = b.center.0.abs_diff(a.center.0) + b.center.1.abs_diff(a.center.1);
            if sensor_distance == (a.radius + b.radius + 2).try_into().unwrap() {
                if a.center.1 < bound && b.center.1 < bound {
                    bounding_sensors.push(a);
                    bounding_sensors.push(b);
                }
            }
        }
    }
    // Then check the perimeter of the beacons
    for s in &bounding_sensors {
        // println!("{:?} {:?}", s, s.walk_perimeter());
        let others = bounding_sensors.iter().filter(|o| *o != s).collect_vec();
        for p in s.walk_perimeter((0, bound)) {
            if others.iter().all(|o| {
                p.0.abs_diff(o.center.0) + p.1.abs_diff(o.center.1) > o.radius.try_into().unwrap()
            }) {
                return p;
            }
        }
    }

    unreachable!();
}

fn beacon_exclusion_zone(sensors: &[Sensor], y: i32) -> usize {
    let ranges_at_y = sensors
        .iter()
        .filter_map(|s| s.get_coverage_range_at_y(y))
        .collect_vec();

    let sensors_at_y = sensors
        .iter()
        .map(|s| s.center)
        .unique()
        .filter(|c| c.1 == y)
        .count();
    let beacons_at_y = sensors
        .iter()
        .map(|s| s.beacon)
        .unique()
        .filter(|c| c.1 == y)
        .count();
    get_total_covered(&ranges_at_y) + sensors_at_y - beacons_at_y
}

// fn collapse_ranges(ranges: &[(i32, i32)])

fn get_total_covered(ranges: &[(i32, i32)]) -> usize {
    if ranges.len() == 0 {
        return 0;
    }

    let test_range = ranges[0].clone();
    let rest = ranges[1..].iter().map(|r| r.clone()).collect_vec();

    let covered: usize = (test_range.0.abs_diff(test_range.1) + 1)
        .try_into()
        .unwrap();
    covered + get_total_covered(&rest)
        - get_total_covered(
            &rest
                .iter()
                .filter_map(|other| test_range.get_intersection(other))
                .collect_vec(),
        )
}

trait Intersectable {
    fn get_intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl Intersectable for (i32, i32) {
    fn get_intersection(&self, other: &Self) -> Option<(i32, i32)> {
        if other.0 <= self.1 && self.0 <= other.1 {
            return Some((self.0.max(other.0), self.1.min(other.1)));
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Sensor {
    center: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

impl Sensor {
    fn walk_perimeter(&self, (min, max): (i32, i32)) -> Vec<(i32, i32)> {
        let mut pos = (self.center.0, self.center.1 - self.radius - 1);

        let mut positions = Vec::with_capacity((self.radius + 1) as usize * 4);
        positions.push(pos);
        let directions = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
        for (dx, dy) in directions {
            for _ in 0..self.radius + 1 {
                pos = (pos.0 + dx, pos.1 + dy);
                if pos.0 >= min && pos.0 <= max && pos.1 >= 0 && pos.1 <= max {
                    positions.push(pos);
                }
            }
        }
        positions
    }

    fn covers_y(&self, y: i32) -> bool {
        let y_min = self.center.1 - self.radius;
        let y_max = self.center.1 + self.radius;

        y >= y_min && y <= y_max
    }

    fn get_coverage_range_at_y(&self, y: i32) -> Option<(i32, i32)> {
        if !self.covers_y(y) {
            return None;
        }
        let reach = self.radius - self.center.1.abs_diff(y) as i32;
        Some((self.center.0 - reach, self.center.0 + reach))
    }

    fn get_coverage_at_y(&self, y: i32) -> usize {
        if let Some((min, max)) = self.get_coverage_range_at_y(y) {
            return (max as usize - min as usize) + 1;
        }
        0
    }
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let (sensor_part, beacon_part) = value.split_once(": ").unwrap();
        let (sensor_x_part, sensor_y_part) = sensor_part
            .split("at ")
            .nth(1)
            .unwrap()
            .split_once(", ")
            .unwrap();
        let center_x = sensor_x_part
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let center_y = sensor_y_part
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let (beacon_x_part, beacon_y_part) = beacon_part
            .split("at ")
            .nth(1)
            .unwrap()
            .split_once(", ")
            .unwrap();
        let beacon_x = beacon_x_part
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let beacon_y = beacon_y_part
            .split('=')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let radius: i32 = (beacon_x - center_x).abs() + (beacon_y - center_y).abs();

        Sensor {
            center: (center_x, center_y),
            beacon: (beacon_x, beacon_y),
            radius,
        }
    }
}
