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
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut pairs = Vec::new();
    for line in lines {
        parse_pairs(&mut pairs, &line.chars().collect_vec(), &mut 0);
    }

    (0, 0)
}

fn parse_pairs(pairs: &mut Vec<Option<Pair>>, chars: &[char], ptr: &mut usize) -> usize {
    // [[1,2],3]
    let mut pair: Pair = Pair {
        left: None,
        right: None,
        value: None,
        parent: None,
    };

    pairs.push(Some(pair));
    let this = pairs.len() - 1;

    if chars[*ptr] == '[' {
        *ptr += 1;
        pair.left = Some(parse_pairs(pairs, chars, ptr));
    } else {
        let comma_idx = chars.find_next(',', *ptr).unwrap();
        let lit = dbg!(String::from_iter(&chars[*ptr..comma_idx]))
            .parse()
            .unwrap();
        pairs.push(Some(Pair::with_value(lit)));
        pair.left = Some(pairs.len() - 1);
        *ptr = comma_idx + 1;
    }

    if chars[*ptr] == '[' {
        *ptr += 1;
        pair.right = Some(parse_pairs(pairs, chars, ptr));
    } else {
        let end_idx = chars.find_next(']', *ptr).unwrap();
        let lit = dbg!(String::from_iter(&chars[*ptr..end_idx])).parse().unwrap();
        pairs.push(Some(Pair::with_value(lit)));
        pair.right = Some(pairs.len() - 1);
        *ptr = end_idx + 1;
    }

    pairs[pair.left.unwrap()].unwrap().parent = Some(this);
    pairs[pair.right.unwrap()].unwrap().parent = Some(this);

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
