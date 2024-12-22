#![feature(int_roundings)]
use std::{num::ParseIntError, str::FromStr};

use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(22);

struct Buyer {
    secret: u64,
}

impl FromStr for Buyer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u64>() {
            Ok(secret) => Ok(Buyer { secret }),
            Err(e) => Err(e),
        }
    }
}

impl Buyer {
    fn mix(&mut self, value: u64) {
        self.secret ^= value;
    }

    fn prune(&mut self) {
        self.secret %= 16777216;
    }

    fn evolve(&mut self) {
        self.mix(self.secret * 64);
        self.prune();

        self.mix(self.secret.div_floor(32));
        self.prune();

        self.mix(self.secret * 2048);
        self.prune();
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let mut buyers = lines_no_empty(input)
        .map(|l| Buyer::from_str(l).unwrap())
        .collect_vec();

    for buyer in buyers.iter_mut() {
        for _ in 0..2000 {
            buyer.evolve();
        }
    }
    let sum_2000: u64 = buyers.iter().map(|b| b.secret).sum();

    (Some(sum_2000), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, (Some(37327623), None));
    }
}
