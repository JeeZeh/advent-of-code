use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
}

impl Button {
    fn from_str(s: &str) -> Button {
        let (x, y) = s.split_once(": ").unwrap().1.split_once(", ").unwrap();
        Button {
            x: x.split_once("+").unwrap().1.parse().unwrap(),
            y: y.split_once("+").unwrap().1.parse().unwrap(),
        }
    }

    fn press(&self, times: i64) -> (i64, i64) {
        (self.x * times, self.y * times)
    }
}

#[derive(Debug)]
struct System {
    a: Button,
    b: Button,
    prize: (i64, i64),
}

impl System {
    fn from_str(s: &str) -> System {
        let mut lines = s.lines();
        let a = Button::from_str(lines.next().unwrap());
        let b = Button::from_str(lines.next().unwrap());
        let (x, y) = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(",")
            .unwrap();
        System {
            a,
            b,
            prize: (
                x.split_once("=").unwrap().1.parse().unwrap(),
                y.split_once("=").unwrap().1.parse().unwrap(),
            ),
        }
    }

    fn get_cost(&self, extend: i64) -> Option<i64> {
        let bottom_coeff = -self.b.y;
        let top_coeff = self.b.x;
        let lhs = (self.a.y * top_coeff) + (self.a.x * bottom_coeff);
        let rhs = ((self.prize.1 + extend) * top_coeff) + ((self.prize.0 + extend) * bottom_coeff);
        if rhs.rem_euclid(lhs) == 0 {
            let a = rhs.div_euclid(lhs);
            let b = ((self.prize.0 + extend) - self.a.x * a).div_euclid(self.b.x);
            return self.validate_presses(a, b, extend);
        }
        None
    }

    fn validate_presses(&self, a: i64, b: i64, extend: i64) -> Option<i64> {
        let press_a = self.a.press(a);
        let press_b = self.b.press(b);
        let p = (press_a.0 + press_b.0, press_a.1 + press_b.1);
        if (p.0 == self.prize.0 + extend) && p.1 == self.prize.1 + extend {
            return Some((a * 3) + b);
        }
        None
    }
}

// HINT USED: Confirmed that linear equations was the right way to do this.
pub fn solve(input: &str) -> (Option<i64>, Option<i64>) {
    let systems = input.split("\n\n").map(System::from_str).collect_vec();

    let part_one: i64 = systems.iter().filter_map(|sys| sys.get_cost(0)).sum();
    let part_two: i64 = systems
        .iter()
        .filter_map(|sys| sys.get_cost(10000000000000))
        .sum();

    (
        Some((part_one as u64).try_into().unwrap()),
        Some((part_two as u64).try_into().unwrap()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(480), Some(875318608908)));
    }
}
