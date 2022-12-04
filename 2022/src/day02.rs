const ROCK: Hand = Hand(0);
const PAPER: Hand = Hand(1);
const SCISSORS: Hand = Hand(2);

#[derive(PartialEq, Clone)]
struct Hand(usize);

// A strategy represents a set of three hands to be applied to (A|X), (B|Y), (C|Z) respectively
struct Strategy(Hand, Hand, Hand);

impl Hand {
    fn loses_to(&self) -> Hand {
        Hand((self.0 + 1).rem_euclid(3))
    }
    fn from_code_with_strategy(code: &char, Strategy(a, b, c): Strategy) -> Hand {
        match code {
            'A' | 'X' => a,
            'B' | 'Y' => b,
            'C' | 'Z' => c,
            _ => panic!("Unexpected hand code!"),
        }
    }
    fn from_code(code: &char) -> Hand {
        Hand::from_code_with_strategy(code, Strategy(ROCK, PAPER, SCISSORS))
    }
}

fn get_round_score(opponent_hand: &Hand, player_hand: &Hand) -> usize {
    let base_score = player_hand.0 + 1;
    if player_hand == &opponent_hand.loses_to() {
        return base_score + 6;
    } else if opponent_hand == player_hand {
        return base_score + 3;
    }
    base_score
}

// Get the honest and rigged outcomes (player scores) of a round with specified opponent and player scores
fn get_outcomes(opponent_code: char, player_code: char) -> (usize, usize) {
    let opponent_hand = &Hand::from_code(&opponent_code);
    (
        get_round_score(opponent_hand, &Hand::from_code(&player_code)),
        get_round_score(
            opponent_hand,
            &Hand::from_code_with_strategy(
                &player_code,
                Strategy(
                    opponent_hand.loses_to().loses_to(),
                    opponent_hand.clone(),
                    opponent_hand.loses_to(),
                ),
            ),
        ),
    )
}

pub fn solve(input: String) -> (usize, usize) {
    input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(l, r)| (l.chars().next().unwrap(), r.chars().next().unwrap()))
        .map(|(opponent_code, player_code)| get_outcomes(opponent_code, player_code))
        .reduce(|total, (honest, rigged)| (total.0 + honest, total.1 + rigged))
        .unwrap()
}
