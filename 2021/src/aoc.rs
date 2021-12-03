/**
 * Author: mgoszcz2@
 */
pub use itertools::Itertools;

use std::fmt::{Debug, Display};
use std::fs;
use std::str::FromStr;

pub trait AocInput {
    fn make(input: String) -> Self;
}

pub trait AocOutput {
    fn show(&self);
}

impl AocInput for String {
    fn make(input: String) -> String {
        input
    }
}

impl<T> AocInput for Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    fn make(input: String) -> Vec<T> {
        input
            .lines()
            .map(|x| x.parse().expect("parse failed"))
            .collect()
    }
}

impl AocOutput for () {
    fn show(&self) {
        println!("Forgot to return output?");
    }
}

impl<A: Display> AocOutput for (A,) {
    fn show(&self) {
        println!("Part 1: {}", self.0);
    }
}

impl<A: Display, B: Display> AocOutput for (A, B) {
    fn show(&self) {
        println!("Part 1: {}", self.0);
        println!("Part 2: {}", self.1);
    }
}

pub fn run<T, R>(day: u32, solution: impl Fn(T) -> R) -> Box<R>
where
    T: AocInput,
    R: AocOutput,
{
    let input = fs::read_to_string(format!("inputs/day{day:02}.txt")).expect("input file");
    Box::new(solution(T::make(input)))
}
