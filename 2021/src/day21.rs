use ahash::AHashMap;

static OUTCOMES: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct TurnState(u8, u8, u8, u8, bool);

pub fn solve(lines: Vec<String>) -> (u32, u64) {
    let player_one: u8 = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let player_two: u8 = lines[1].split_once(": ").unwrap().1.parse().unwrap();

    let part_one = part_one(player_one as u32, player_two as u32);

    let mut cache: AHashMap<u64, (u64, u64)> = AHashMap::new();
    let part_two = count_wins(&mut cache, TurnState(player_one, 0, player_two, 0, true));

    (part_one.1 * part_one.2, part_two.0.max(part_two.1))
}

fn count_wins(cache: &mut AHashMap<u64, (u64, u64)>, turn: TurnState) -> (u64, u64) {
    if turn.1 >= 21 || turn.3 >= 21 {
        return ((turn.1 > turn.3) as u64, (turn.1 < turn.3) as u64);
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    for (outcome, times) in OUTCOMES {
        let mut next_turn = turn.clone();
        if turn.4 {
            next_turn.0 = ((next_turn.0 + outcome - 1) % 10) + 1;
            next_turn.1 += next_turn.0;
            next_turn.4 = false;
        } else {
            next_turn.2 = ((next_turn.2 + outcome - 1) % 10) + 1;
            next_turn.3 += next_turn.2;
            next_turn.4 = true;
        }

        let next_key = ((next_turn.0 as u64) << 24)
            | ((next_turn.1 as u64) << 16)
            | ((next_turn.2 as u64) << 8)
            | ((next_turn.3 as u64) << 1)
            | (next_turn.4 as u64);

        if let Some((p1_w, p2_w)) = cache.get(&next_key) {
            p1_wins += *p1_w * times;
            p2_wins += *p2_w * times;
        } else {
            let (p1_w, p2_w) = count_wins(cache, next_turn);
            p1_wins += p1_w * times;
            p2_wins += p2_w * times;
        }
    }

    let key = ((turn.0 as u64) << 24)
        | ((turn.1 as u64) << 16)
        | ((turn.2 as u64) << 8)
        | ((turn.3 as u64) << 1)
        | (turn.4 as u64);
    cache.insert(key, (p1_wins, p2_wins));
    (p1_wins, p2_wins)
}

fn part_one(mut player_one: u32, mut player_two: u32) -> (u32, u32, u32) {
    let mut p1_score: u32 = 0;
    let mut p2_score: u32 = 0;

    let mut die = 0;
    let mut rolls = 0;

    let mut p1_turn = true;

    while p1_score < 1000 && p2_score < 1000 {
        let mut sum = 0;

        for _ in 0..3 {
            die = (die % 100) + 1;
            sum += die;
        }
        rolls += 3;

        if p1_turn {
            player_one = ((player_one + sum - 1) % 10) + 1;
            p1_score += player_one;
        } else {
            player_two = ((player_two + sum - 1) % 10) + 1;
            p2_score += player_two;
        }

        p1_turn = !p1_turn;
    }

    (p1_score.max(p2_score), p1_score.min(p2_score), rolls)
}
