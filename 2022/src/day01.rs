use itertools::Itertools;

pub fn solve(input: String) -> (usize, usize) {
    let elves: Vec<usize> = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|elf| elf.lines().map(|c| c.parse::<usize>().unwrap()).sum())
        .sorted()
        .rev()
        .take(3)
        .collect();

    (*elves.first().unwrap(), elves.iter().sum())
}
