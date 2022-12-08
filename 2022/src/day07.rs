use std::{
    collections::{HashMap, VecDeque},
    iter::Peekable,
    str::Lines,
};

use itertools::Itertools;

#[derive(Debug)]
enum Command {
    LS,
    CD(String),
}

#[derive(Debug)]
enum Pointer {
    File(String, u32),
    Directory(String),
}

impl From<&str> for Pointer {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(" ").unwrap();
        match left {
            "dir" => Pointer::Directory(right.to_string()),
            _ => Pointer::File(right.to_string(), left.parse().unwrap()),
        }
    }
}

pub fn solve(input: String) -> (u32, u32) {
    let file_system = create_filesystem(&input);
    let mut sizes = HashMap::new();

    get_directory_sizes(&file_system, &mut sizes, "/");

    let needed_space: u32 = 30_000_000;
    let free_space: u32 = 70_000_000 - sizes.get("/").unwrap();
    let need_to_free: u32 = needed_space - free_space;

    (
        sizes.values().filter(|s| **s <= 100_000).sum::<u32>(),
        *sizes
            .values()
            .sorted()
            .find(|s| **s >= need_to_free)
            .unwrap() as u32,
    )
}

// Recursively determine sizes of directory from a provided starting path
// Reads passed in `fs` and fills passed in `sizes` with results
fn get_directory_sizes(
    fs: &HashMap<String, Vec<Pointer>>,
    sizes: &mut HashMap<String, u32>,
    starting: &str,
) -> u32 {
    let mut total_size = 0;
    if let Some(pointers) = fs.get(starting) {
        for pointer in pointers {
            match pointer {
                Pointer::File(_, size) => total_size += size,
                Pointer::Directory(name) => {
                    total_size += get_directory_sizes(
                        fs,
                        sizes,
                        &(starting.to_string() + &name.to_owned() + "/"),
                    )
                }
            }
        }
    }
    sizes.insert(starting.to_string(), total_size);
    total_size
}

// From the given input, read all lines as terminal output, parsing contents into a
// 'flat' file system of [path => pointers] mappings, where a pointer can be a File or Directory
fn create_filesystem(input: &str) -> HashMap<String, Vec<Pointer>> {
    let mut file_system: HashMap<String, Vec<Pointer>> = HashMap::new();

    // - Keep track of cwd
    let mut cwd: VecDeque<String> = VecDeque::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        // Process each command starting with $:
        match read_command(line) {
            Command::LS => process_ls(&mut lines, &cwd, &mut file_system),
            Command::CD(target) => process_cd(&mut cwd, &target),
        }
    }

    file_system
}

// From the current working directory and provided target, either
// go up one level, return to root, or traverse into the target (extend cwd)
fn process_cd(cwd: &mut VecDeque<String>, target: &str) {
    match target {
        ".." => {
            cwd.pop_back();
        }
        "/" => cwd.clear(),
        dir => cwd.push_back(dir.to_string()),
    };
}

// From the current working directory and terminal output position
// read all lines of output, parsing into Pointers of Files and Directories,
// and store them in the file system.
fn process_ls(
    lines: &mut Peekable<Lines>,
    cwd: &VecDeque<String>,
    fs: &mut HashMap<String, Vec<Pointer>>,
) {
    let base_path = if cwd.len() == 0 {
        String::from("/")
    } else {
        format!("/{}/", cwd.iter().join("/"))
    };

    let mut pointers = Vec::new();
    loop {
        pointers.push(Pointer::from(lines.next().unwrap()));
        // Stop processing if we've reached the end or the next line is another command
        let ahead = lines.peek();
        if ahead.is_none() || ahead.unwrap().starts_with("$ ") {
            break;
        }
    }

    fs.insert(base_path, pointers);
}

// Extract the command from the passed string
fn read_command(s: &str) -> Command {
    let mut parts = s.split(" ").skip(1);

    let cmd = parts.next().unwrap();
    match cmd {
        "cd" => Command::CD(parts.next().unwrap().to_string()),
        "ls" => Command::LS,
        _ => panic!("Unexpected command {cmd}"),
    }
}
