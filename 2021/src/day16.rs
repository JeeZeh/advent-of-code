pub fn solve(data: String) -> (u64, u64) {
    // Convert hex to binary (4 bits)
    let bits: Vec<u8> = data
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();
    let (version, type_id) = get_header(&bits);

    // Chop off any 0 bits?

    // ID 4 = literal value
    // To extract, take next 3 groups of 5 bits
    // discard the first bit of each
    // add

    (0, 0)
}

fn drop_nth_bit(number: u8, n: usize) -> u8 {
    number & !(1 << n)
}

fn extract_literal(bits: &[u8]) -> u16 {
    // 1101 0010 1111 1110 0010 1000
    // VVVT TTAA AAAB BBBB CCCC C
    let a = ((bits[1] & 1) << 3) + (bits[2] >> 1);
    let b = bits[2];
    let c = ((bits[3] & 1) << 3) + (bits[4] >> 1);

}

/// 6 bit header, 3 bits for packet version, 3 for packet type
/// Each 3 bit value is actually a binary value, a 'u3' (0 -> 7)
fn get_header(bits: &[u8]) -> (u8, u8) {
    let version = bits[0] >> 1;
    let id = ((bits[0] & 1) << 2) + (bits[1] >> 2);

    (version, id)
}

#[cfg(test)]

mod day06 {

    use super::*;
    #[test]
    fn test_header() {
        assert_eq!(get_header(&vec![13, 2]), (6, 4));
        assert_eq!(get_header(&vec![3, 8]), (1, 6));
    }
}
