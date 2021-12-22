use ahash::AHashMap;

static OUTCOMES: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
struct Player(u32, u32);
#[derive(PartialEq, Eq, Hash)]
struct TurnState(u8, u8, u8, u8, bool);

pub fn solve(lines: Vec<String>) -> (u32, u64) {
    let player_one: u64 = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let player_two: u64 = lines[1].split_once(": ").unwrap().1.parse().unwrap();

    let part_one = play(player_one as u32, player_two as u32, true);

    let mut cache: AHashMap<u64, (u64, u64)> = AHashMap::new();
    let part_two = count_wins(&mut cache, player_one << 24 | (player_two << 1) | 1);

    (part_one.1 * part_one.2, part_two.0.max(part_two.1))
}

fn count_wins(cache: &mut AHashMap<u64, (u64, u64)>, mut turn: u64) -> (u64, u64) {
    if let Some(hit) = cache.get(&turn) {
        return *hit;
    }

    if ((turn >> 16) & 0xff) >= 21 || ((turn >> 1) & 0xff) >= 21 {
        if ((turn >> 16) & 0xff) > ((turn >> 1) & 0xff) {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    for (outcome, times) in OUTCOMES {
        if (turn & 1) == 0 {
            // (turn.0 << 24) | (turn.1 << 16) |   (turn.2 << 8) |   (turn.3 << 1) | (turn.4 as u8)
            turn &= ((((((turn >> 24) & 0xff) + outcome - 1) % 10) + 1) << 24) & 0xffffffff;
            turn &= ((((turn >> 24) & 0xff) + ((turn >> 16) & 0xff)) << 16) & 0xffffffff;
            turn &= !1
        } else {
            turn &= ((((((turn >> 8) & 0xff) + outcome - 1) % 10) + 1) << 8) & 0xffffffff;
            turn &= ((((turn >> 8) & 0xff) + ((turn >> 1) & 0xff)) << 1) & 0xffffffff;
            turn |= 1
        }

        let (future_p1_wins, future_p2_wins) = count_wins(cache, turn);

        p1_wins += future_p1_wins * times;
        p2_wins += future_p2_wins * times;
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
