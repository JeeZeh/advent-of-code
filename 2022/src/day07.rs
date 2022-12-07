use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;

struct Context {
    cwd: String,
}

enum PointerType {
    File,
    Directory,
}

struct Pointer {
    name: String,
    size: u32,
    ptype: PointerType,
}

pub fn solve(input: String) {}

fn create_filesystem(input: &str) -> HashMap<String, Pointer> {
    let file_system: HashMap<String, Pointer> = HashMap::new();

    // Process each command starting with $:
    // - Keep track of cwd
    // - ls gives a list of files and directory pointers to create
    // - cd changes the cwd
    // - store path in fs with size and name

    file_system
}
