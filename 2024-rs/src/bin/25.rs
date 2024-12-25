use advent_of_code::lines_no_empty;

advent_of_code::solution!(25);

pub fn solve(input: &str) -> (Option<String>, Option<String>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut shape = [0u8; 5];
    let mut counting = None;
    let mut height = 0;
    for line in lines_no_empty(input) {
        if height == 0 {
            counting = line.chars().next();
        }
        line.chars().enumerate().for_each(|(i, c)| {
            if Some(c) == counting && (height == 0 || shape[i] == height - 1) {
                shape[i] = height
            }
        });
        if height == 6 {
            match counting {
                Some('.') => locks.push(shape),
                Some('#') => keys.push(shape),
                _ => panic!("Unmatched shape: {:?}", counting),
            }
            shape = [0u8; 5];
            height = 0;
        } else {
            height += 1;
        }
    }

    // Test keys against locks
    let mut matches = 0;
    for key in keys {
        matches += locks
            .iter()
            .filter(|lock| {
                lock.iter()
                    .zip(key)
                    .all(|(&lock_slot, key_slot)| key_slot <= lock_slot)
            })
            .count();
    }

    (Some(format!("{}", matches)), Some("🎄🎄🎄".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some("3".to_string()), None));
    }
}
