use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Pair {
    left: Option<usize>,
    right: Option<usize>,
    value: Option<i32>,
    parent: Option<usize>,
}

impl Pair {
    fn with_value(value: i32) -> Self {
        Pair {
            left: None,
            right: None,
            value: Some(value),
            parent: None,
        }
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
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut memory = Vec::new();

    let mut pairs: Vec<usize> = Vec::new();
    for line in lines {
        let new_pair = parse_pairs(&mut memory, &line.chars().collect_vec(), &mut 0);
        pairs.push(new_pair);
    }

    let first = memory[pairs[0]].unwrap();
    dbg!(&first);

    dbg!(&first
        .get_left(&memory)
        .unwrap()
        .get_right(&memory)
        .unwrap()
        .get_left(&memory)
        .unwrap()
        .get_left(&memory)
        .unwrap()
        .value
        .unwrap());

    (0, 0)
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
        memory.push(Some(Pair::with_value(lit)));
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

    let pair: Pair = Pair {
        left: left,
        right: right,
        value: None,
        parent: None,
    };

    let this = memory.len();
    memory.push(Some(pair));

    memory[left.unwrap()].unwrap().parent = Some(this);
    memory[right.unwrap()].unwrap().parent = Some(this);

    this
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
