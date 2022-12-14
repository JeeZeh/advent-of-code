use itertools::{EitherOrBoth, Itertools};

pub fn solve(input: String) -> (usize, usize) {
    let packets = input
        .split("\n\n")
        .map(|p| p.split_once("\n").unwrap())
        .collect_vec();

    let part_one = packets
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| is_sorted(l, r).unwrap())
        .map(|(i, _)| i + 1)
        .sum();

    (part_one, part_two(input))
}

fn part_two(input: String) -> usize {
    let joined_input = input.replace("\n\n", "\n");
    let mut augmented_input = joined_input.lines().collect_vec();
    augmented_input.push("[[2]]");
    augmented_input.push("[[6]]");
    let all_sorted = sort_all(&augmented_input);
    let (div_one, _) = all_sorted
        .iter()
        .enumerate()
        .find(|(i, p)| *p == "[[2]]")
        .unwrap();
    let (div_two, _) = all_sorted
        .iter()
        .enumerate()
        .find(|(i, p)| *p == "[[6]]")
        .unwrap();

    (div_one + 1) * (div_two + 1)
}

fn sort_all(input: &[&str]) -> Vec<String> {
    input
        .iter()
        .map(|s| s.to_string())
        .sorted_by(|a, b| match is_sorted(a, b) {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        })
        .collect_vec()
}

fn is_sorted(left: &str, right: &str) -> Option<bool> {
    // println!("Compare '{}' vs '{}'", left, right);
    if !left.is_list() && !right.is_list() {
        let left_num = left.parse::<u32>().unwrap();
        let right_num = right.parse::<u32>().unwrap();
        if left_num == right_num {
            return None;
        }
        return Some(left.parse::<u32>().unwrap() < right.parse::<u32>().unwrap());
    }

    if left.is_list() && right.is_list() {
        if left == "[]" && right == "[]" {
            return None;
        }
        for next in left
            .get_list_slices()
            .iter()
            .zip_longest(right.get_list_slices().iter())
        {
            match next {
                EitherOrBoth::Both(l, r) => {
                    let sorted = is_sorted(l, r);
                    if sorted.is_some() {
                        return sorted;
                    }
                }
                EitherOrBoth::Left(_) => return Some(false),
                EitherOrBoth::Right(_) => return Some(true),
            }
        }
        return None;
    }

    if left.is_list() {
        return is_sorted(left, &format!("[{}]", right));
    }
    return is_sorted(&format!("[{}]", left), right);
}

trait PacketStr<T> {
    fn is_list(&self) -> bool;

    fn get_list_slices(&self) -> Vec<&str>;

    fn next_value_slice<'a>(&self, start: usize) -> Option<&str>;
}

impl<'a> PacketStr<&'a str> for &str {
    fn is_list(&self) -> bool {
        let mut chars = self.chars();
        chars.next().unwrap() == '[' && chars.last().unwrap() == ']'
    }

    fn get_list_slices(&self) -> Vec<&str> {
        let mut slices = Vec::new();

        let mut idx = 1;
        while idx < self.len() - 1 {
            let slice = self.next_value_slice(idx).unwrap();
            slices.push(slice);
            idx += slice.len() + 1;
        }

        slices
    }

    fn next_value_slice(&self, start: usize) -> Option<&str> {
        let mut start_pos = None;
        let mut end_pos = None;
        let mut expect_list = false;
        let mut relative_nesting = 0;
        for (offset, c) in self.chars().skip(start).enumerate() {
            if start_pos.is_none() && c != ']' && c != ',' {
                start_pos = Some(start + offset);
                expect_list = c == '[';
            }
            if c == '[' {
                relative_nesting += 1;
            }
            if c == ']' {
                relative_nesting -= 1;
            }
            if start_pos.is_some()
                && end_pos.is_none()
                && relative_nesting <= 0
                && (c == ']' || c == ',')
            {
                if expect_list {
                    end_pos = Some(start + offset)
                } else {
                    end_pos = Some(start + offset - 1)
                }
            }
            if start_pos.is_some() && end_pos.is_some() {
                return Some(&self[start_pos.unwrap()..=end_pos.unwrap()]);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{is_sorted, PacketStr};

    #[test]
    fn get_slices() {
        assert_eq!("[1,2,3]".get_list_slices(), vec!["1", "2", "3"]);
        assert_eq!("[1,[[]],3]".get_list_slices(), vec!["1", "[[]]", "3"]);
    }

    #[test]
    fn next_value_list() {
        let input = "[1,2,3]";
        assert_eq!(input.next_value_slice(0).unwrap(), "[1,2,3]");
    }

    #[test]
    fn next_value_long_list() {
        let input = "[10,2,3]";
        assert_eq!(input.next_value_slice(1).unwrap(), "10");
    }

    #[test]
    fn next_value_integer_in_list() {
        let input = "[1,2,3]";
        assert_eq!(input.next_value_slice(1).unwrap(), "1");
    }
    #[test]
    fn next_value_list_in_list() {
        let input = "[1,[2,3]]";
        assert_eq!(input.next_value_slice(0).unwrap(), "[1,[2,3]]");
        assert_eq!(input.next_value_slice(1).unwrap(), "1");
        assert_eq!(input.next_value_slice(2).unwrap(), "[2,3]");
    }

    #[test]
    fn next_value_empty_list_in_list() {
        let input = "[1,[[],3]]";
        assert_eq!(input.next_value_slice(0).unwrap(), "[1,[[],3]]");
        assert_eq!(input.next_value_slice(1).unwrap(), "1");
        assert_eq!(input.next_value_slice(2).unwrap(), "[[],3]");
        assert_eq!(input.next_value_slice(4).unwrap(), "[]");
        assert_eq!(input.next_value_slice(6).unwrap(), "3");
    }

    #[test]
    fn is_list() {
        assert_eq!("[]".is_list(), true);
        assert_eq!("[1]".is_list(), true);
        assert_eq!("[1,2,3]".is_list(), true);
        assert_eq!("1".is_list(), false);
    }

    #[test]
    fn is_sorted_integer() {
        assert_eq!(is_sorted("1", "2").unwrap(), true);
        assert_eq!(is_sorted("2", "1").unwrap(), false);
    }

    #[test]
    fn is_sorted_list() {
        assert_eq!(is_sorted("[1,1,3,1,1]", "[1,1,5,1,1]").unwrap(), true);
    }

    #[test]
    fn is_sorted_list_mixed() {
        assert_eq!(is_sorted("[1,1,3,1,1]", "[[1],1,5,1,1]").unwrap(), true);
    }
    #[test]
    fn is_sorted_list_empty() {
        assert_eq!(is_sorted("[]", "[3]").unwrap(), true);
        assert_eq!(is_sorted("[]", "[[]]").unwrap(), true);
        assert_eq!(is_sorted("[[]]", "[]").unwrap(), false);
    }

    #[test]
    fn is_sorted_list_not_helpful() {
        assert_eq!(is_sorted("[[1,1,1],2]", "[[1,1,1],3]").unwrap(), true);
        assert_eq!(is_sorted("[[1,1,1],2]", "[[1,1,1],1]").unwrap(), false);
    }

    #[test]
    fn is_sorted_all_test_cases() {
        assert_eq!(is_sorted("[1,1,3,1,1]", "[1,1,5,1,1]").unwrap(), true);
        assert_eq!(is_sorted("[[1],[2,3,4]]", "[[1],4]").unwrap(), true);
        assert_eq!(is_sorted("[9]", "[[8,7,6]]").unwrap(), false);
        assert_eq!(is_sorted("[[4,4],4,4]", "[[4,4],4,4,4]").unwrap(), true);
        assert_eq!(is_sorted("[7,7,7,7]", "[7,7,7]").unwrap(), false);
    }

    #[test]
    fn real_8() {
        let left = "[[10,[]]]";
        let right = "[[[4],[3,4,2]],[3,[[],[6,5],6,4,[8,4,5]],[],[3,[],4,[0]],3],[7,[[0]],[[],[5,9,5],4,[2,3]]]]";
        assert_eq!(is_sorted(left, right).unwrap(), false);
    }

    #[test]
    fn real_9() {
        let left = "[[10,2,4],[],[]]";
        let right = "[[4,[7,0,9]]]";
        assert_eq!(is_sorted(left, right).unwrap(), false);
    }

    #[test]
    fn real_85() {
        let left = "[[],[8],[8,3],[]]";
        let right = "[[],[2,[4,3,6]],[8]]";
        assert_eq!(is_sorted(left, right).unwrap(), false);
    }
}
