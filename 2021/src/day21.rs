use cached::proc_macro::cached;

static OUTCOMES: [(u32, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
struct Player(u32, u32);

pub fn solve(lines: Vec<String>) -> (u32, u64) {
    let player_one: u32 = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let player_two: u32 = lines[1].split_once(": ").unwrap().1.parse().unwrap();

    let part_one = play(player_one, player_two, true);
    let part_two = count_wins(player_one, 0, player_two, 0, true);

    (part_one.1 * part_one.2, part_two.0.max(part_two.1))
}

#[cached]
fn count_wins(p1_pos: u32, p1_score: u32, p2_pos: u32, p2_score: u32, p1_next: bool) -> (u64, u64) {
    if p1_score >= 21 || p2_score >= 21 {
        if p1_score > p2_score {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    for (outcome, times) in OUTCOMES {
        if p1_next {
            let next_p1_pos = ((p1_pos + outcome - 1) % 10) + 1;
            let next_p1_score = p1_score + next_p1_pos;
            let next_wins = count_wins(next_p1_pos, next_p1_score, p2_pos, p2_score, false);
            p1_wins += next_wins.0 * times;
            p2_wins += next_wins.1 * times;
        } else {
            let next_p2_pos = ((p2_pos + outcome - 1) % 10) + 1;
            let next_p2_score = p2_score + next_p2_pos;
            let next_wins = count_wins(p1_pos, p1_score, next_p2_pos, next_p2_score, true);
            p1_wins += next_wins.0 * times;
            p2_wins += next_wins.1 * times;
        }
    }

    (p1_wins, p2_wins)
}

fn play(mut player_one: u32, mut player_two: u32, deterministic: bool) -> (u32, u32, u32) {
    let mut p1_score: u32 = 0;
    let mut p2_score: u32 = 0;

    let mut die = 0;
    let mut rolls = 0;
    let mut rolled = 0;

    let mut p1_turn = true;

    while p1_score < 1000 && p2_score < 1000 {
        if deterministic {
            let roll = deterministic_role(die, 3);
            rolled = roll.0;
            die = roll.1;
            rolls += 3
        }
        if p1_turn {
            player_one = ((player_one + rolled - 1) % 10) + 1;
            p1_score += player_one;
        } else {
            player_two = ((player_two + rolled - 1) % 10) + 1;
            p2_score += player_two;
        }

        p1_turn = !p1_turn;
    }

    (p1_score.max(p2_score), p1_score.min(p2_score), rolls)
}

fn deterministic_role(mut start: u32, n: usize) -> (u32, u32) {
    let mut sum = 0;

    for _ in 0..n {
        start = (start % 100) + 1;
        sum += start;
    }

    (sum, start)
}
