pub fn solve(data: String) -> (u64, u64) {
    // Convert hex to binary (4 bits)
    let bits = get_bits(&data.as_str());

    let mut ptr = 0;
    let mut sum_versions = 0;
    let result = recurse_packets(&bits, &mut ptr, &mut sum_versions);

    (sum_versions, result)
}

fn part_one(bits: &[u64]) -> (u64, u64) {
    // what do you get if you add up the version numbers in all packets?
    let mut ptr = 0;
    let mut sum_versions = 0;

    (
        recurse_packets(bits, &mut ptr, &mut sum_versions),
        sum_versions,
    )
}

fn recurse_packets(bits: &[u64], ptr: &mut usize, sum_versions: &mut u64) -> u64 {
    if *ptr >= 4 * (bits.len()) {
        return 0;
    }
    let (version, type_id) = get_header(bits, ptr);
    *sum_versions += version;

    if type_id == 4 {
        return extract_literal(bits, ptr);
    }

    let num_packets;

    if get_absolute_bit(bits, *ptr) == 1 {
        *ptr += 1;
        num_packets = get_n_bits(bits, 11, ptr);
    } else {
        *ptr += 1;
        num_packets = get_n_bits(bits, 15, ptr);
    }

    let mut values = Vec::new();

    for _ in 0..num_packets {
        values.push(recurse_packets(bits, ptr, sum_versions));
    }

    // dbg!(&values);
    return match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknon type ID"),
    };
}

fn get_bits(s: &str) -> Vec<u64> {
    s.chars().map(|c| c.to_digit(16).unwrap() as u64).collect()
}

trait BitOps {
    fn get_nth_bit(&self, n: usize) -> u64;
    fn add_lsb(&mut self, b: u64);
}

impl BitOps for u64 {
    fn get_nth_bit(&self, n: usize) -> u64 {
        (self >> n) & 1
    }

    fn add_lsb(&mut self, b: u64) {
        *self = (*self << 1) + (1 & b);
    }
}

fn get_absolute_bit(bits: &[u64], abs: usize) -> u64 {
    let rel_nibble = abs / 4;
    let rel_bit = 3 - (abs % 4);
    bits[rel_nibble].get_nth_bit(rel_bit)
}

fn get_n_bits(bits: &[u64], n: usize, ptr: &mut usize) -> u64 {
    let mut number = 0;
    for _ in 0..n {
        number.add_lsb(get_absolute_bit(bits, *ptr));
        *ptr += 1;
    }
    number
}

fn extract_literal(bits: &[u64], ptr: &mut usize) -> u64 {
    let mut literal: u64 = 0;
    let mut last = false;
    while !last {
        last = get_absolute_bit(bits, *ptr) != 1;
        *ptr += 1;

        literal <<= 4;
        literal += get_n_bits(bits, 4, ptr);
    }

    literal
}

/// 6 bit header, 3 bits for packet version, 3 for packet type
/// Each 3 bit value is actually a binary value, a 'u3' (0 -> 7)
fn get_header(bits: &[u64], ptr: &mut usize) -> (u64, u64) {
    (get_n_bits(bits, 3, ptr), get_n_bits(bits, 3, ptr))
}

#[cfg(test)]

mod day06 {

    use super::*;

    #[test]
    fn test_header() {
        assert_eq!(get_header(&vec![13, 2], &mut 0), (6, 4));
        assert_eq!(get_header(&vec![0, 3, 8], &mut 4), (1, 6));
    }

    #[test]
    fn test_extract_literal() {
        assert_eq!(extract_literal(&vec![13, 2, 15, 14, 2, 8], &mut 6), 2021);
        assert_eq!(extract_literal(&get_bits(&"38006F45291200"), &mut 28), 10);
        assert_eq!(extract_literal(&get_bits(&"38006F45291200"), &mut 39), 20);
    }
}
