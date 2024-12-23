use std::iter::once;

use advent_of_code::lines_no_empty;
use itertools::Itertools;

advent_of_code::solution!(23);

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
/// A Union-Find (Disjoint Set) data structure implementation in Rust.
///
/// The Union-Find data structure keeps track of elements partitioned into
/// disjoint (non-overlapping) sets.
/// It provides near-constant-time operations to add new sets, to find the
/// representative of a set, and to merge sets.
///
/// See: https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/union_find.rs
pub struct UnionFind<T: Debug + Eq + Hash> {
    payloads: HashMap<T, usize>, // Maps values to their indices in the parent_links array.
    parent_links: Vec<usize>,    // Holds the parent pointers; root elements are their own parents.
    sizes: Vec<usize>,           // Holds the sizes of the sets.
    count: usize,                // Number of disjoint sets.
}

impl<T: Debug + Eq + Hash> UnionFind<T> {
    /// Creates an empty Union-Find structure with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent_links: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            payloads: HashMap::with_capacity(capacity),
            count: 0,
        }
    }

    /// Inserts a new item (disjoint set) into the data structure.
    pub fn insert(&mut self, item: T) {
        let key = self.payloads.len();
        self.parent_links.push(key);
        self.sizes.push(1);
        self.payloads.insert(item, key);
        self.count += 1;
    }

    /// Returns the root index of the set containing the given value, or `None` if it doesn't exist.
    pub fn find(&mut self, value: &T) -> Option<usize> {
        self.payloads
            .get(value)
            .copied()
            .map(|key| self.find_by_key(key))
    }

    /// Unites the sets containing the two given values. Returns:
    /// - `None` if either value hasn't been inserted,
    /// - `Some(true)` if two disjoint sets have been merged,
    /// - `Some(false)` if both elements were already in the same set.
    pub fn union(&mut self, first_item: &T, sec_item: &T) -> Option<bool> {
        let (first_root, sec_root) = (self.find(first_item), self.find(sec_item));
        match (first_root, sec_root) {
            (Some(first_root), Some(sec_root)) => Some(self.union_by_key(first_root, sec_root)),
            _ => None,
        }
    }

    /// Finds the root of the set containing the element with the given index.
    fn find_by_key(&mut self, key: usize) -> usize {
        if self.parent_links[key] != key {
            self.parent_links[key] = self.find_by_key(self.parent_links[key]);
        }
        self.parent_links[key]
    }

    /// Unites the sets containing the two elements identified by their indices.
    fn union_by_key(&mut self, first_key: usize, sec_key: usize) -> bool {
        let (first_root, sec_root) = (self.find_by_key(first_key), self.find_by_key(sec_key));

        if first_root == sec_root {
            return false;
        }

        match self.sizes[first_root].cmp(&self.sizes[sec_root]) {
            Ordering::Less => {
                self.parent_links[first_root] = sec_root;
                self.sizes[sec_root] += self.sizes[first_root];
            }
            _ => {
                self.parent_links[sec_root] = first_root;
                self.sizes[first_root] += self.sizes[sec_root];
            }
        }

        self.count -= 1;
        true
    }

    /// Checks if two items belong to the same set.
    pub fn is_same_set(&mut self, first_item: &T, sec_item: &T) -> bool {
        matches!((self.find(first_item), self.find(sec_item)), (Some(first_root), Some(sec_root)) if first_root == sec_root)
    }

    /// Returns the number of disjoint sets.
    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: Debug + Eq + Hash> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            parent_links: Vec::default(),
            sizes: Vec::default(),
            payloads: HashMap::default(),
            count: 0,
        }
    }
}

impl<T: Debug + Eq + Hash> FromIterator<T> for UnionFind<T> {
    /// Creates a new UnionFind data structure from an iterable of disjoint elements.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut uf = UnionFind::default();
        for item in iter {
            uf.insert(item);
        }
        uf
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let connections = lines_no_empty(input)
        .map(|l| l.split_once("-").unwrap())
        .collect_vec();
    let computers = connections
        .iter()
        .flat_map(|conn| once(conn.0).chain(once(conn.1)))
        .unique()
        .collect_vec();

    let mut udfs: UnionFind<&str> = UnionFind::with_capacity(computers.len());
    computers.iter().for_each(|c| udfs.insert(c));
    connections.iter().for_each(|(a, b)| {
        udfs.union(a, b);
    });

    let networks_of_three = computers
        .iter()
        .permutations(3)
        .unique()
        .map(|network| {
            (
                udfs.find(network[0]).unwrap(),
                udfs.find(network[1]).unwrap(),
                udfs.find(network[2]).unwrap(),
            )
        })
        .filter(|(a, b, c)| a == b && b == c)
        .collect_vec();

    if cfg!(debug_assertions) {
        println!("{connections:?}");
        println!("{computers:?}");
        println!("{networks_of_three:?}");
    }

    println!(
        "{:?}",
        computers
            .iter()
            .filter(|c| c.starts_with("t"))
            .map(|c| udfs.find(c))
            .collect_vec()
    );

    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(7), None));
    }
}
