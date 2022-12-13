use itertools::Itertools;

pub fn solve(input: String) -> (usize, usize) {
    let packets = input
        .split("\n\n")
        .map(|p| p.split_once("\n").unwrap())
        .collect_vec();
    (0, 0)
}

trait PacketStr<T> {
    fn compare_packet(&self, right: T) -> bool;

    fn next_value_slice<'a>(&self, start: usize) -> Option<&str>;
}

impl<'a> PacketStr<&'a str> for &str {
    fn compare_packet(&self, right: &str) -> bool {
        let mut left_ptr: usize = 0;
        let mut right_ptr: usize = 0;

        false
    }

    fn next_value_slice(&self, start: usize) -> Option<&str> {
        let mut start_pos = None;
        let mut end_pos = None;
        for (offset, c) in self.chars().enumerate().skip(start) {
            if start_pos.is_none() && c != ']' && c != ',' {
                start_pos = Some(start + offset);
            }
            if end_pos.is_none() && (c == ']' || c == ',') {
                end_pos = Some(start + offset)
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
