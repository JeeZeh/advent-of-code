use counter::Counter;

advent_of_code::solution!(1);

pub fn solve(input: &str) -> (Option<u64>, Option<u64>) {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(a, b)| {
            (
                u64::from_str_radix(a, 10).unwrap(),
                u64::from_str_radix(b, 10).unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    let right_counts = Counter::from_iter(&right);
    (
        Some(left.iter().zip(&right).map(|x| x.0.abs_diff(*x.1)).sum()),
        Some(
            left.iter()
                .map(|num| num * right_counts.get(num).unwrap_or(&0))
                .sum(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (Some(11), Some(31)));
    }
}
