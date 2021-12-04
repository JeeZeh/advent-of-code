pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let bit_len = lines[0].len();
    let bins: Vec<u32> = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    println!("{:?}", &lines);
    (part_one(&bins, bit_len), 0)
}

fn part_one(bins: &Vec<u32>, bit_len: usize) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;

    let most_common = get_most_common_bit_per_idx(&bins, bit_len);
    for bit in most_common {
        gamma <<= 1;
        epsilon <<= 1;
        if bit {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    dbg!(gamma) * dbg!(epsilon)
}

fn part_two(bins: &Vec<u32>, bit_len: usize) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;

    let oxygen = bins.clone();
    let co2 = bins.clone();

    let most_common = get_most_common_bit_per_idx(&bins, bit_len);
    for bit in most_common {
        gamma <<= 1;
        epsilon <<= 1;
        if bit {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    dbg!(gamma) * dbg!(epsilon)
}

fn get_bit(input: u32, n: usize) -> bool {
    input & (1 << n) != 0
}

fn get_most_common_bit_per_idx(bits_strings: &Vec<u32>, bit_len: usize) -> Vec<bool> {
    let mut ones: Vec<usize> = vec![0; bit_len];
    for bit_string in bits_strings {
        for i in 0..bit_len {
            ones[i] += get_bit(*bit_string, bit_len - i - 1) as usize;
        }
    }

    ones.iter()
        .map(|count| *count > bits_strings.len() - count)
        .collect()
}
