use std::{collections::VecDeque, usize};

struct Game {
    marbles: VecDeque<usize>,
    players: Vec<usize>,
    player: usize,
}

impl Game {
    fn new(players: usize, final_marble: usize) -> Game {
        let mut game = Game {
            marbles: VecDeque::with_capacity(final_marble),
            players: Vec::with_capacity(players),
            player: 0,
        };

        game.marbles.push_back(0);
        (0..players).for_each(|_| game.players.push(0));

        return game;
    }

    fn something_entirely_different(&mut self, marble: usize) {
        // Remove 7th marble to the left and add it to the current player's score
        // Also add the to-be-placed marble to the current player's score
        self.marbles.rotate_right(7);
        self.players[self.player] += marble + self.marbles.pop_front().unwrap();
    }

    fn insert(&mut self, marble: usize) {
        if marble % 23 == 0 {
            self.something_entirely_different(marble);
        } else {
            self.marbles.rotate_left(2 % self.marbles.len());
            self.marbles.push_front(marble);
            self.player = (self.player + 1) % self.players.len();
        }
    }
}

fn main() {
    println!("Part 1: {}", part_one(447, 71510));
    println!("Part 2: {}", part_one(447, 71510 * 100));
}

fn part_one(players: usize, final_marble: usize) -> usize {
    let mut game = Game::new(players, final_marble);

    for marble in 1..=final_marble {
        game.insert(marble);
    }

    *game.players.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_examples() {
        assert_eq!(part_one(9, 25), 32);
        assert_eq!(part_one(10, 1618), 8317); // This one breaks?
        assert_eq!(part_one(13, 7999), 146373);
        assert_eq!(part_one(17, 1104), 2764);
        assert_eq!(part_one(21, 6111), 54718);
        assert_eq!(part_one(30, 5807), 37305);
    }
}
