pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let bit_len = lines[0].len();
    let bins: Vec<u32> = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    (part_one(&bins, bit_len), part_two(&bins, bit_len))
}

fn part_one(bins: &Vec<u32>, bit_len: usize) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in (0..bit_len).rev() {
        gamma <<= 1;
        epsilon <<= 1;
        if most_common_bit_at_idx(&bins, bit) {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn part_two(bins: &Vec<u32>, bit_len: usize) -> usize {
    let mut oxygen = bins.clone();
    let mut co2 = bins.clone();

    for bit_idx in (0..bit_len).rev() {
        if oxygen.len() > 1 {
            let common = most_common_bit_at_idx(&oxygen, bit_idx);
            oxygen.retain(|bin| get_bit(*bin, bit_idx) == common);
        }
        if co2.len() > 1 {
            let common = most_common_bit_at_idx(&co2, bit_idx);
            co2.retain(|bin| get_bit(*bin, bit_idx) != common);
        }
    }

    (co2[0] * oxygen[0]) as usize
}

fn get_bit(input: u32, n: usize) -> bool {
    input & (1 << n) != 0
}

fn most_common_bit_at_idx(bits_strings: &Vec<u32>, idx: usize) -> bool {
    let mut ones = 0;
    for bit_string in bits_strings {
        ones += get_bit(*bit_string, idx) as usize;
    }

    ones >= bits_strings.len() - ones
}
