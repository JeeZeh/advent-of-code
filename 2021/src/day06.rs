pub fn solve(line: String) -> (usize, usize) {
    let starting: Vec<i32> = line.split(",").map(|c| c.parse().unwrap()).collect();

    let mut total = 0;

    for fish in starting {
        total += get_new_fish_count(fish, 18);
    }

    (total as usize, 0)
}

fn get_new_fish_count(fish: i32, remaining: i32) -> i32 {
    let mut total = 1;
    let mut new_fis = 0;
    let mut new_rem = remaining - fish;

    while new_fis > 0 {
        new_fis = new_rem / 7;
        total += new_fis;
        new_rem -= 8;       
    }

    total
}
