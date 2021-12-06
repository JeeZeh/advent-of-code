pub fn solve(line: String) -> (usize, usize) {
    let starting: Vec<i32> = line.split(",").map(|c| c.parse().unwrap()).collect();

    let mut total = 0;

    for fish in starting {
        total += get_new_fish_count(fish, 800);
    }

    (total as usize, 0)
}

fn get_new_fish_count(fish: i32, remaining: i32) -> i32 {
    let mut total = 1;

    for rem in (0..remaining - fish).rev().step_by(7) {
        total += get_new_fish_count(8, rem);
    }

    total
}
