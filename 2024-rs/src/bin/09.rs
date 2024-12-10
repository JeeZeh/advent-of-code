use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
enum Block {
    File(usize, usize),
    Free(usize),
}

impl Block {
    fn from_disk_map((idx, c): (usize, char)) -> Self {
        let size = c.to_digit(10).unwrap_or_else(|| panic!("Could not parse char: {c}")) as usize;
        if idx.rem_euclid(2) == 0 {
            Block::File(idx / 2, size)
        } else {
            Block::Free(size)
        }
    }

    fn transfer(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.size()).map(|_| self.id())
    }

    fn size(&self) -> usize {
        match self {
            Block::File(_, size) => *size,
            Block::Free(size) => *size,
        }
    }

    fn id(&self) -> usize {
        match self {
            Block::File(id, _) => *id,
            Block::Free(_) => panic!("Cannot get ID for Block::Free"),
        }
    }
}

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let disk_map = input
        .chars()
        .enumerate()
        .map(Block::from_disk_map)
        .collect_vec();

    // println!("{disk_map:?}");
    (
        Some(part_one(&disk_map) as u64),
        Some(part_two(&disk_map) as u64),
    )
}

// Abomination, please look no further.
fn part_two(disk_map: &Vec<Block>) -> usize {
    let mut moved = HashSet::<usize>::new();
    let mut working_disk = disk_map.clone();
    // let mut last_move = 0;
    for block in disk_map.iter().rev() {
        if !matches!(block, Block::File(_, _)) || moved.contains(&block.id()) {
            continue;
        }
        let (src, _) = working_disk
            .iter()
            .find_position(|find_src| {
                matches!(find_src, Block::File(_, _)) && find_src.id() == block.id()
            })
            .unwrap();

        if let Some((dest, space)) = working_disk
            .iter_mut()
            .find_position(|space| matches!(space, Block::Free(_)) && space.size() >= block.size())
        {
            if src < dest {
                continue;
            }
            if block.size() == space.size() {
                // println!("Inserting {src}={block:?} @ {dest}={space:?}");
                working_disk[src] = *space;
                working_disk[dest] = *block;
            } else if block.size() < space.size() {
                // println!("Inserting {src}={block:?} @ {dest}={space:?}");
                // working_disk[src] = *space;
                working_disk.insert(dest, *block);
                match working_disk[dest + 1] {
                    Block::File(_, _) => panic!("Unexpected file"),
                    Block::Free(old_space) => {
                        working_disk[dest + 1] = Block::Free(old_space - block.size())
                    }
                }
            } else {
                unreachable!();
            }
            moved.insert(block.id());
            // println!("{working_disk:?}");
        }
    }
    let mut left_ptr = 0;
    let mut check_sum = 0;
    let mut counted = HashSet::<usize>::new();
    for block in working_disk {
        match block {
            Block::File(id, size) => {
                if !counted.contains(&id) {
                    check_sum += (left_ptr..(left_ptr + size)).map(|p| p * id).sum::<usize>();
                }
                left_ptr += size;
                counted.insert(block.id());
            }
            Block::Free(size) => {
                left_ptr += size;
            }
        };
    }
    check_sum
}

// Tried to be too smart here, it's not even fast...
fn part_one(disk_map: &Vec<Block>) -> usize {
    let mut move_from = disk_map.iter().rev();
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = disk_map.iter().map(Block::size).sum();

    let mut transfer = move_from.next().unwrap().transfer();
    let mut check_sum: usize = 0;
    for block in disk_map {
        if left_ptr >= right_ptr {
            break;
        }
        match block {
            Block::File(id, size) => {
                check_sum += (left_ptr..(left_ptr + size).min(right_ptr))
                    .map(|p| p * id)
                    .sum::<usize>();
                left_ptr += size;
            }
            Block::Free(size) => {
                let block_start = left_ptr;
                while left_ptr < (block_start + size).min(right_ptr) {
                    if let Some(id) = transfer.next() {
                        check_sum += left_ptr * id;
                        left_ptr += 1;
                        right_ptr -= 1;
                    } else {
                        // Skip space after file
                        let free = move_from.next().unwrap();
                        right_ptr -= free.size();
                        // Start processing next file
                        transfer = move_from.next().unwrap().transfer();
                    }
                }
            }
        };
    }
    check_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(1928), Some(2858)));
    }
}
