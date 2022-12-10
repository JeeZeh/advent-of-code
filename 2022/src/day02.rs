#[derive(PartialEq, Clone)]
struct Hand(i8);

// A strategy represents a set of three hands to be applied to (A|X), (B|Y), (C|Z) respectively
struct Strategy(Hand, Hand, Hand);

const ROCK: Hand = Hand(0);
const PAPER: Hand = Hand(1);
const SCISSORS: Hand = Hand(2);
const HONEST_STRATEGY: Strategy = Strategy(ROCK, PAPER, SCISSORS);

impl Hand {
    fn beats(&self) -> Hand {
        Hand((self.0 - 1).rem_euclid(3))
    }
    fn beaten_by(&self) -> Hand {
        Hand((self.0 + 1).rem_euclid(3))
    }
    fn with_strategy(code: &str, Strategy(a, b, c): Strategy) -> Hand {
        match code {
            "A" | "X" => a,
            "B" | "Y" => b,
            "C" | "Z" => c,
            _ => panic!("Unexpected hand code!"),
        }
    }
}

fn get_round_score(opponent_hand: &Hand, player_hand: &Hand) -> usize {
    let game_score = if player_hand == &opponent_hand.beaten_by() {
        6
    } else if player_hand == opponent_hand {
        3
    } else {
        0
    };
    (game_score + player_hand.0 + 1) as usize
}

pub fn solve(input: String) -> (usize, usize) {
    let undecided_game: Vec<(Hand, &str)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(l, r)| (Hand::with_strategy(l, HONEST_STRATEGY), r))
        .collect();

    let honest_game = undecided_game
        .iter()
        .map(|(op, player)| get_round_score(op, &Hand::with_strategy(player, HONEST_STRATEGY)))
        .sum();

    let rigged_game = undecided_game
        .iter()
        .map(|(op, player)| {
            get_round_score(
                op,
                &Hand::with_strategy(player, Strategy(op.beats(), op.clone(), op.beaten_by())),
            )
        })
        .sum();

    (honest_game, rigged_game)
}
