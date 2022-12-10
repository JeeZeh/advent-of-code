use itertools::Itertools;

const ASCII_A_LOWERCASE: u8 = 97;

pub fn solve(input: String) -> (usize, usize) {
    // Represent each character of the alphabet in a 32bit bit-mask
    let mask_vec = input
        .bytes()
        .map(|c| (1) << (c - ASCII_A_LOWERCASE))
        .collect_vec();

    (get_marker_pos(&mask_vec, 4), get_marker_pos(&mask_vec, 14))
}

// Pass a window of size `need_unique` over the slice of bit-masks `marker_masks` and return
// the position of the last character in the first window that contains only unique bit-masks
fn get_marker_pos(marker_masks: &[u32], need_unique: usize) -> usize {
    marker_masks
        .windows(need_unique)
        .position(all_unique_bits)
        .unwrap()
        + need_unique
}

// For each bit-mask in the slice provided,
fn all_unique_bits(masks: &[u32]) -> bool {
    // bitwise-or all masks together to determine if they are unique
    let mut unique = 0;
    for mask in masks {
        if unique | mask == unique {
            return false;
        }
        unique |= mask;
    }
    true
}
