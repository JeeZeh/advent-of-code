use itertools::Itertools;

pub fn solve(input: String) -> (usize, usize) {
    let packets = input
        .split("\n\n")
        .map(|p| p.split_once("\n").unwrap())
        .collect_vec();

    (0, 0)
}

fn is_sorted(left: &str, right: &str) -> bool {
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = 0;

    false
}

trait PacketStr<T> {
    fn next_value_slice<'a>(&self, start: usize) -> Option<&str>;
}

impl<'a> PacketStr<&'a str> for &str {
    fn next_value_slice(&self, start: usize) -> Option<&str> {
        let mut start_pos = None;
        let mut end_pos = None;
        let mut relative_nesting = 0;
        for (offset, c) in self.chars().skip(start).enumerate() {
            if start_pos.is_none() && c != ']' && c != ',' {
                start_pos = Some(start + offset);
            }
            if c == '[' {
                relative_nesting += 1;
            }
            if c == ']' {
                relative_nesting -= 1;
            }
            if start_pos.is_some()
                && end_pos.is_none()
                && c != '['
                && c != ','
                && relative_nesting == 0
            {
                end_pos = Some(start + offset)
            }
            if start_pos.is_some() && end_pos.is_some() {
                break;
            }
        }

        if start_pos.is_some() && end_pos.is_some() {
            return Some(&self[start_pos.unwrap()..=end_pos.unwrap()]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::PacketStr;

    #[test]
    fn next_value_list() {
        let input = "[1,2,3]";
        assert_eq!(input.next_value_slice(0).unwrap(), "[1,2,3]");
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
}
