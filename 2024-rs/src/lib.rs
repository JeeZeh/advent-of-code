#![feature(trait_alias)]

use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
    slice::Iter,
    str::FromStr,
};

pub mod template;

// Use this file to add helper functions and additional modules.
/// A convenience function for splitting and parsing a string.
pub fn numbers<T>(line: &str, sep: char) -> impl Iterator<Item = T> + '_
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    line.split(sep).map(|x| x.parse::<T>().unwrap())
}

/// A trait to simplify printing and indexing of 2D data structures.
pub trait Grid<T> {
    /// Produces an iterator of positions (x, y) and item T from the Grid in
    /// reading order (left to right, top to bottom)
    fn scan<'a>(&'a self) -> impl Iterator<Item = ((usize, usize), &'a T)>
    where
        T: 'a;
    fn getxy_pos_mut(&mut self, pos: (usize, usize)) -> Option<&mut T>;
    fn getxy_pos(&self, pos: (usize, usize)) -> Option<&T>;
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
                .flat_map(|y| self.getyx(y, x))
                .map(|x| f(x).to_string().len())
                .max();
            widths.push(max_width.unwrap_or(0) as u8);
        }

        let all1s = widths.iter().all(|x| *x == 1);

        for y in 0..self.height() {
            for (x, c) in widths.iter().enumerate().take(self.width()) {
                let width = *c as usize + !all1s as usize;
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

    fn getxy_pos(&self, pos: (usize, usize)) -> Option<&T> {
        self.get(pos.1).and_then(|row| row.get(pos.0))
    }

    fn getxy_pos_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        self.get_mut(pos.1).and_then(|row| row.get_mut(pos.0))
    }

    fn width(&self) -> usize {
        self.first().map_or(0, |x| x.len())
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

    fn show_debug(&self)
    where
        T: Debug,
    {
        if cfg!(debug_assertions) {
            self.show_map(|x| std::format!("{:?}", x));
        }
    }

    fn show_display(&self)
    where
        T: Display,
    {
        if cfg!(debug_assertions) {
            self.show_map(|x| x.to_string());
        }
    }

    fn show_map<V: Display>(&self, f: impl Fn(&T) -> V) {
        let mut widths: Vec<u8> = Vec::with_capacity(self.width());
        for x in 0..self.width() {
            let max_width = (0..self.height())
                .flat_map(|y| self.getyx(y, x))
                .map(|x| f(x).to_string().len())
                .max();
            widths.push(max_width.unwrap_or(0) as u8);
        }

        let all1s = widths.iter().all(|x| *x == 1);

        for y in 0..self.height() {
            for (x, c) in widths.iter().enumerate().take(self.width()) {
                let width = *c as usize + !all1s as usize;
                if let Some(value) = self.getyx(y, x) {
                    std::print!("{:>w$}", format!("{:}", f(value)), w = width);
                } else {
                    std::print!("{:>w$}", "X", w = width);
                }
            }
            std::println!();
        }
    }

    fn scan<'a>(&'a self) -> impl Iterator<Item = ((usize, usize), &'a T)>
    where
        T: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, t)| ((x, y), t)))
    }
}

impl<T: Copy, const W: usize, const H: usize> Grid<T> for [[T; W]; H] {
    fn getyx(&self, y: usize, x: usize) -> Option<&T> {
        self.get(y).and_then(|row| row.get(x))
    }

    fn getyx_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        self.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn getxy_pos(&self, pos: (usize, usize)) -> Option<&T> {
        self.get(pos.1).and_then(|row| row.get(pos.0))
    }

    fn getxy_pos_mut(&mut self, pos: (usize, usize)) -> Option<&mut T> {
        self.get_mut(pos.1).and_then(|row| row.get_mut(pos.0))
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

    fn scan<'a>(&'a self) -> impl Iterator<Item = ((usize, usize), &'a T)>
    where
        T: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, t)| ((x, y), t)))
    }
}

pub trait PosNumber = Add + Sub + Mul + Div + Clone + Copy + Debug;
pub trait Pos2D<T: PosNumber> {
    fn sub(&self, other: &Self) -> (T, T);
    fn add(&self, other: &Self) -> (T, T);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }

    pub fn step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn step_usize(&self, pos: (usize, usize)) -> (usize, usize) {
        let step = self.step();
        (
            (pos.0 as i32 + step.0) as usize,
            (pos.1 as i32 + step.1) as usize,
        )
    }
}

impl<T: PosNumber> Pos2D<T> for (T, T)
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    fn add(&self, other: &Self) -> (T, T) {
        (self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: &Self) -> (T, T) {
        (self.0 - other.0, self.1 - other.1)
    }
}

pub trait Pairs<T> {
    fn pairs<'a>(&self) -> impl Iterator<Item = (T, T)>
    where
        T: 'a,
        T: Copy;
}

impl<T> Pairs<T> for Vec<T> {
    fn pairs<'a>(&self) -> impl Iterator<Item = (T, T)>
    where
        T: 'a,
        T: Copy,
    {
        return self
            .iter()
            .enumerate()
            .flat_map(|(i, a)| self[i + 1..].iter().map(|b| (*a, *b)));
    }
}
