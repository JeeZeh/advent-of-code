from collections import deque

decks = open("real.txt").read().split("\n\n")

def create_players():
    player1 = deque(map(int, decks[0].splitlines()[1:]))
    player2 = deque(map(int, decks[1].splitlines()[1:]))
    return player1, player2

def get_score(deck):
    return sum((i + 1) * c for i, c in enumerate(reversed(deck)))


def combat(p1_deck: deque, p2_deck: deque, rec=False):
    game_history = set()

    while p1_deck and p2_deck:
        if rec:
            game = tuple(p1_deck) + (0,) + tuple(p2_deck)
            if game in game_history:
                return 1, p1_deck
            game_history.add(game)

        p1_card, p2_card = p1_deck.popleft(), p2_deck.popleft()

        winner = 1 if p1_card > p2_card else 2

        if rec and len(p1_deck) >= p1_card and len(p2_deck) >= p2_card:
            winner, _ = combat(
                deque(list(p1_deck)[:p1_card]),
                deque(list(p2_deck)[:p2_card]),
                True
            )
        

        if winner == 1:
            p1_deck.extend((p1_card, p2_card))
        else:
            p2_deck.extend((p2_card, p1_card))

    return (1, p1_deck) if p1_deck else (2, p2_deck)


def part1():
    winner, deck = combat(*create_players())
    print(f"Player {winner} wins!")
    print(get_score(deck))

def part2():
    winner, deck = combat(*create_players(), True)
    print(f"Player {winner} wins!")
    print(get_score(deck))
    
part1()
part2()