use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
enum Inst {
    Mul(u32, u32),
    Do,
    DoNot,
}

pub fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let sequence = input.lines().map(|l| l.trim()).join("");

    let do_re = Regex::new(r"do\(\)").unwrap();
    let dos = do_re
        .captures_iter(&sequence)
        .map(|caps| (caps.get(0).unwrap().start(), Inst::Do));

    let donot_re = Regex::new(r"don\'t\(\)").unwrap();
    let donots = donot_re
        .captures_iter(&sequence)
        .map(|caps| (caps.get(0).unwrap().start(), Inst::DoNot));

    let mult_re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mults = mult_re.captures_iter(&sequence).map(|caps| {
        (
            caps.get(0).unwrap().start(),
            Inst::Mul(caps["a"].parse().unwrap(), caps["b"].parse().unwrap()),
        )
    });

    let mut all = 0;
    let mut when_enabled = 0;
    let mut enabled = true;
    for (_, inst) in mults.chain(dos).chain(donots).sorted_by_key(|&(i, _)| i) {
        match inst {
            Inst::Mul(a, b) => {
                let res = a * b;
                all += res;
                if enabled {
                    when_enabled += res;
                }
            }
            Inst::Do => enabled = true,
            Inst::DoNot => enabled = false,
        }
    }

    (Some(all), Some(when_enabled))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(161), Some(48)));
    }
}
