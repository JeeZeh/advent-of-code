pub fn solve(input: String) -> (i64, String) {
    let sum: i64 = input.lines().map(snafu_to_decimal).sum();

    (sum, decimal_to_snafu(sum))
}

/// Lovely solution: https://github.com/Philippe-Cholet/rusty-aoc/blob/main/aoc2022/day25/src/lib.rs
fn decimal_to_snafu(mut dec: i64) -> String {
    let mut snafu = String::new();

    while dec != 0 {
        let remainder = dec.rem_euclid(5);
        dec = dec.div_euclid(5) + i64::from(remainder > 2);
        snafu.insert(
            0,
            match remainder {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => panic!("Unexpected remainder {}", remainder),
            },
        );
    }

    snafu
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut num = 0;
    for (pow, c) in snafu.chars().rev().enumerate() {
        num += match c {
            '-' => -1,
            '=' => -2,
            d => d.to_digit(10).unwrap() as i64,
        } * (5i64.pow(pow as u32));
    }

    num
}
