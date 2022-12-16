use itertools::Itertools;

pub fn solve(input: String) -> (usize, usize) {
    let sensors = input.lines().map(Sensor::from).collect_vec();
    // sensors
    //     .iter()
    //     .map(|s| (s, s.get_coverage_at_y(8)))
    //     .for_each(|s| println!("{:?}", s));
    (beacon_exclusion_zone(&sensors, 2000000), 0)
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
                .filter_map(|other| test_range.get_insersection(other))
                .collect_vec(),
        )
}

trait Intersectable {
    fn get_insersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
}

impl Intersectable for (i32, i32) {
    fn get_insersection(&self, other: &Self) -> Option<(i32, i32)> {
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

    fn get_intersection_coverage_at_y(&self, other: &Self, y: i32) -> usize {
        if let Some((self_min, self_max)) = self.get_coverage_range_at_y(y) {
            if let Some((other_min, other_max)) = other.get_coverage_range_at_y(y) {
                if other_min <= self_max && self_min <= other_max {
                    return (self_max.min(other_max) - self_min.max(other_min))
                        .try_into()
                        .unwrap();
                }
            }
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
