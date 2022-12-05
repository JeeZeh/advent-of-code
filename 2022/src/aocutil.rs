//! Advent of Code utilities.

#[doc(no_inline)]
pub use itertools::Itertools;

use std::fmt::{Debug, Display};
use std::fs;
use std::str::FromStr;
use std::time::{Duration, Instant};

/// A trait responsible for pre-parsing input for individual Advent of Code solutions.
pub trait AocInput {
    /// Returns the parsed input.
    fn make(input: String) -> Self
    where
        Self: Sized;
}

impl AocInput for String {
    fn make(input: String) -> String {
        input.replace("\r\n", "\n")
    }
}

/// A trait implemented by all integer types `AocInput` accepts.
pub trait AocNumber {}

#[doc(hidden)]
macro_rules! impl_aoc_number {
    ($($ty:ty),*) => {
        $(impl AocNumber for $ty {})*
    };
}

impl_aoc_number!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

impl<T> AocInput for Vec<T>
where
    T: AocNumber,
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

/// Treat the input string as a bytestring.
impl AocInput for Vec<Vec<u8>> {
    fn make(input: String) -> Vec<Vec<u8>> {
        input.lines().map(|x| x.bytes().collect()).collect()
    }
}

impl AocInput for Vec<String> {
    fn make(input: String) -> Vec<String> {
        input.lines().map(|x| x.to_string()).collect()
    }
}

pub trait AocOutput {
    fn show(&self);
}

impl AocOutput for () {
    fn show(&self) {}
}

impl<A: Display, B: Display> AocOutput for (A, B) {
    fn show(&self) {
        println!("Part 1: {}", self.0);
        println!("Part 2: {}", self.1);
    }
}

pub fn run<T, R: 'static>(
    day: u32,
    solution: impl Fn(T) -> R,
    sample: bool,
) -> (Box<dyn AocOutput>, Duration)
where
    T: AocInput,
    R: AocOutput,
{
    let input_prefix = if sample { "inputs/test" } else { "inputs" };
    let raw_input =
        fs::read_to_string(format!("{input_prefix}/day{day:02}.txt")).expect("input file");
    let prepared_input = T::make(raw_input);

    let now = Instant::now();
    let output = Box::new(solution(prepared_input));

    (output, now.elapsed())
}

/// A convinience function for splitting and parsing a string.
pub fn numbers<T>(line: &str, sep: char) -> impl Iterator<Item = T> + '_
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    line.split(sep).map(|x| x.parse::<T>().unwrap())
}

/// A trait to simplify printing and indexing of 2D data structures.
pub trait Grid<T> {
    /// Returns a reference to an element at a given index or `None` if index is
    /// out of bounds.
    fn getyx(&self, y: usize, x: usize) -> Option<&T>;
    /// Returns a mutable reference to an element at a given index or `None` if
    /// index is out of bounds.
    fn getyx_mut(&mut self, y: usize, x: usize) -> Option<&mut T>;
    /// Returns the maximum horizontal extent of the grid.
    fn width(&self) -> usize;
    /// Returns the maximum vertical extent of the grid.
    fn height(&self) -> usize;

    fn rot90(&self) -> Self;

    /// Prints a grid using the `Debug` trait.
    fn show_debug(&self)
    where
        T: Debug,
    {
        self.show_map(|x| format!("{:?}", x));
    }

    /// Prints a grid using the `Display` trait.
    fn show_display(&self)
    where
        T: Display,
    {
        self.show_map(|x| x.to_string());
    }

    /// Prints the grid, using a closure to decide how to display each element.
    /// Should the grid be sparse (`getyx()` returns `None` within the
    /// width/height bounds), "X" is printed for the missing elements instead.
    /// If every element within the grid is one character long, separating
    /// spaces are omitted.
    fn show_map<V: Display>(&self, f: impl Fn(&T) -> V) {
        let mut widths: Vec<u8> = Vec::with_capacity(self.width());
        for x in 0..self.width() {
            let max_width = (0..self.height())
                .map(|y| self.getyx(y, x))
                .flatten()
                .map(|x| f(x).to_string().len())
                .max();
            widths.push(max_width.unwrap_or(0) as u8);
        }

        let all1s = widths.iter().all(|x| *x == 1);

        for y in 0..self.height() {
            for x in 0..self.width() {
                let width = widths[x] as usize + !all1s as usize;
                if let Some(value) = self.getyx(y, x) {
                    print!("{:>w$}", format!("{:}", f(value)), w = width);
                } else {
                    print!("{:>w$}", "X", w = width);
                }
            }
            println!();
        }
    }
}

impl<T: Copy> Grid<T> for Vec<Vec<T>> {
    fn getyx(&self, y: usize, x: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }

    fn getyx_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        self.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn width(&self) -> usize {
        self.get(0).map_or(0, |x| x.len())
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn rot90(&self) -> Vec<Vec<T>> {
        let new_col_len = self.len();
        let new_row_len = self.first().unwrap().len();
        let mut new_vec: Vec<Vec<T>> = Vec::with_capacity(new_row_len);

        for x in 0..new_col_len {
            let mut row: Vec<T> = Vec::with_capacity(new_col_len);
            for y in 0..new_row_len {
                row.push(*self.getyx((new_col_len - 1) - y, x).unwrap());
            }
            new_vec.push(row);
        }

        new_vec
    }
}

impl<T: Copy, const W: usize, const H: usize> Grid<T> for [[T; W]; H] {
    fn getyx(&self, y: usize, x: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }

    fn getyx_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        self.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn rot90(&self) -> Self {
        *self
    }
}
