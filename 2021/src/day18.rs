use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Pair {
    left: Option<usize>,
    right: Option<usize>,
    value: Option<i32>,
    parent: Option<usize>,
    ptr: usize,
}

impl Pair {
    fn with_value(value: i32, ptr: usize) -> Self {
        Pair {
            left: None,
            right: None,
            value: Some(value),
            parent: None,
            ptr,
        }
    }

    fn update_parents(&mut self, memory: &mut Vec<Option<Pair>>) {
        let mut left_node = memory[self.left.unwrap()].unwrap();
        left_node.parent = Some(self.ptr);
        memory[self.left.unwrap()] = Some(left_node);

        let mut right_node = memory[self.right.unwrap()].unwrap();
        right_node.parent = Some(self.ptr);
        memory[self.right.unwrap()] = Some(right_node);
    }

    fn add(&self, other: Self, memory: &mut Vec<Option<Pair>>) -> (usize, Self) {
        let mut addition = Pair {
            left: Some(self.ptr),
            right: Some(other.ptr),
            value: None,
            parent: None,
            ptr: memory.len(),
        };

        addition.update_parents(memory);

        memory.push(Some(addition));

        (addition.ptr, addition)
    }

    fn get(idx: usize, memory: &[Option<Pair>]) -> Self {
        memory[idx].unwrap()
    }

    fn get_left(&self, memory: &[Option<Pair>]) -> Option<Self> {
        if let Some(left) = self.left {
            return memory[left];
        }

        None
    }

    fn get_right(&self, memory: &[Option<Pair>]) -> Option<Self> {
        if let Some(right) = self.right {
            return memory[right];
        }

        None
    }

    fn get_parent(&self, memory: &[Option<Pair>]) -> Option<Self> {
        if let Some(parent) = self.parent {
            return memory[parent];
        }

        None
    }

    fn explode(&mut self, memory: &mut [Option<Pair>]) {
        let left_lit = self.get_left(memory).unwrap().value.unwrap();
        let right_lit = self.get_right(memory).unwrap().value.unwrap();

        let mut current_node = *self;
        let mut last_node;

        // Drill up to parent with a left node
        'left: while let Some(parent) = current_node.get_parent(memory) {
            last_node = current_node;
            current_node = parent;

            // Enter the left node
            if let Some(left) = current_node.get_left(memory) {
                if last_node.ptr == left.ptr {
                    continue;
                }

                // Drill down to rightmost
                current_node = left;
                while let Some(right) = current_node.get_right(memory) {
                    current_node = right;
                }

                let mut find_right = current_node;
                if let Some(val) = find_right.value {
                    find_right.value = Some(val + left_lit);
                    memory[find_right.ptr] = Some(find_right);

                    break 'left;
                }
            }
        }

        current_node = *self;

        // Drill up to parent with a right node
        'right: while let Some(parent) = current_node.get_parent(memory) {
            last_node = current_node;
            current_node = parent;

            // Enter the right node
            if let Some(right) = current_node.get_right(memory) {
                if last_node.ptr == right.ptr {
                    continue;
                }

                // Drill down to leftmost
                current_node = right;
                while let Some(left) = current_node.get_left(memory) {
                    current_node = left;
                }

                let mut find_left = current_node;
                if let Some(val) = find_left.value {
                    find_left.value = Some(val + right_lit);
                    memory[find_left.ptr] = Some(find_left);

                    break 'right;
                }
            }
        }

        self.right = None;
        self.left = None;
        self.value = Some(0);

        memory[self.ptr] = Some(*self);
    }

    fn process_explodes(&mut self, memory: &mut [Option<Pair>], level: usize) -> bool {
        let mut exploded = false;
        if level == 4 {
            self.explode(memory);
            exploded = true;
        } else {
            if let Some(mut left) = self.get_left(memory) {
                if left.value.is_none() {
                    exploded |= left.process_explodes(memory, level + 1)
                }
            }
            if let Some(mut right) = self.get_right(memory) {
                if right.value.is_none() {
                    exploded |= right.process_explodes(memory, level + 1)
                }
            }
        }

        exploded
    }

    fn split(&mut self, memory: &mut Vec<Option<Pair>>) {
        let lit = self.value.unwrap();

        self.left = Some(memory.len());
        memory.push(Some(Pair::with_value(lit / 2, memory.len())));

        self.right = Some(memory.len());
        memory.push(Some(Pair::with_value((lit + 1) / 2, memory.len())));

        self.update_parents(memory);

        self.value = None;
        memory[self.ptr] = Some(*self);
    }

    fn try_split(&mut self, memory: &mut Vec<Option<Pair>>) -> bool {
        if let Some(val) = self.value {
            if val >= 10 {
                self.split(memory);
                return true;
            }
        } else {
            if let Some(mut left) = self.get_left(memory) {
                if left.try_split(memory) {
                    return true;
                }
            }
            if let Some(mut right) = self.get_right(memory) {
                if right.try_split(memory) {
                    return true;
                }
            }
        }
        return false;
    }

    fn reduce(&mut self, memory: &mut Vec<Option<Pair>>) {
        while self.process_explodes(memory, 0) || self.try_split(memory) {}
    }

    fn magnitude(&mut self, memory: &mut [Option<Pair>]) -> i32 {
        let mut left_mag = 0;
        let mut right_mag = 0;

        if let Some(mut left) = self.get_left(memory) {
            if left.value.is_none() {
                left_mag = left.magnitude(memory);
            } else {
                left_mag = left.value.unwrap();
            }
        }
        if let Some(mut right) = self.get_right(memory) {
            if right.value.is_none() {
                right_mag = right.magnitude(memory);
            } else {
                right_mag = right.value.unwrap();
            }
        }

        left_mag * 3 + right_mag * 2
    }

    fn print(&self, memory: &[Option<Pair>]) -> String {
        if let Some(val) = self.value {
            return format!("{}", val);
        }

        let left = self.get_left(memory).unwrap().print(memory);
        let right = self.get_right(memory).unwrap().print(memory);
        return format!("[{},{}]", left, right);
    }
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut memory: Vec<Option<Pair>> = Vec::new();

    let mut pairs: Vec<usize> = Vec::new();

    for line in lines {
        let new_pair = parse_pairs(&mut memory, &line.chars().collect_vec(), &mut 0);
        pairs.push(new_pair);
    }

    (
        part_one(memory.clone(), pairs.clone()),
        part_two(memory, pairs),
    )
}

fn part_one(mut memory: Vec<Option<Pair>>, pairs: Vec<usize>) -> i32 {
    let mut rolling_pair = memory[pairs[0]].unwrap();

    for pair in pairs[1..].iter() {
        rolling_pair = rolling_pair.add(memory[*pair].unwrap(), &mut memory).1;
        rolling_pair.reduce(&mut memory);
    }

    rolling_pair.magnitude(&mut memory)
}

fn part_two(memory: Vec<Option<Pair>>, pairs: Vec<usize>) -> i32 {
    let mut max = 0;

    for a in pairs.iter() {
        for b in pairs.iter() {
            if a == b {
                continue;
            }

            let mut mem_ = memory.clone();

            let mut add = mem_[*a].unwrap().add(mem_[*b].unwrap(), &mut mem_).1;
            add.reduce(&mut mem_);

            let mag = add.magnitude(&mut mem_);

            if mag > max {
                max = mag;
            }
        }
    }

    max
}

fn parse_pair(
    delim: char,
    memory: &mut Vec<Option<Pair>>,
    chars: &[char],
    ptr: &mut usize,
) -> Option<usize> {
    let left;
    if chars[*ptr] == '[' {
        *ptr += 1;
        left = Some(parse_pairs(memory, chars, ptr));
        *ptr += 1;
    } else {
        let delim_idx = chars.find_next(delim, *ptr).unwrap();
        let lit = String::from_iter(&chars[*ptr..delim_idx]).parse().unwrap();
        memory.push(Some(Pair::with_value(lit, memory.len())));
        left = Some(memory.len() - 1);
        *ptr = delim_idx + 1;
    }

    left
}

fn parse_pairs(memory: &mut Vec<Option<Pair>>, chars: &[char], ptr: &mut usize) -> usize {
    let left = parse_pair(',', memory, chars, ptr);

    if *ptr > chars.len() {
        return left.unwrap();
    }

    let right = parse_pair(']', memory, chars, ptr);

    let mut pair: Pair = Pair {
        left: left,
        right: right,
        value: None,
        parent: None,
        ptr: memory.len(),
    };

    memory.push(Some(pair));
    pair.update_parents(memory);

    pair.ptr
}

trait FindNext {
    fn find_next(&self, find: char, start: usize) -> Option<usize>;
}

impl FindNext for [char] {
    fn find_next(&self, find: char, mut start: usize) -> Option<usize> {
        for char in self[start..].iter() {
            if *char == find {
                return Some(start);
            }
            start += 1;
        }
        None
    }
}
