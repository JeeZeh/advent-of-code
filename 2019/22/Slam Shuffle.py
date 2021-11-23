from collections import deque

deck = [c for c in range(10007)]


def deal(deck):
    deck = deck[::-1]
    return deck


def reverse_deal(ptr: int, decksize: int):
    ptr = decksize - ptr - 1
    return ptr


def cut(deck, n):
    deck = deck[n:] + deck[:n]
    return deck


def reverse_cut(n: int, ptr: int, decksize: int):
    return (ptr + n) % decksize


def incr_deal(deck, n):
    deck = deck[::-1]
    table = [0 for _ in range(len(deck))]
    i = 0
    while deck:
        table[i % len(table)] = deck.pop()
        i += n

    deck = table
    return deck


def reverse_incr_deal(n: int, ptr: int, decksize: int):
    return (ptr * pow(n, -1, mod=decksize)) % decksize


inputs = list(open("input.txt"))


def position_of_card(card, deck):
    for line in inputs:
        if "increment" in line:
            deck = incr_deal(deck, int(line.split(" ")[-1]))
        elif "cut" in line:
            deck = cut(deck, int(line.split(" ")[-1]))
        else:
            deck = deal(deck)

    return deck.index(card)


def number_of_card(position, decksize):
    # Start with the position of the card after shuffling
    ptr = position

    # Follow each step in reverse to undo the shuffling
    for line in reversed(inputs):
        if "increment" in line:
            ptr = reverse_incr_deal(int(line.split(" ")[-1]), ptr, decksize)
        elif "cut" in line:
            ptr = reverse_cut(int(line.split(" ")[-1]), ptr, decksize)
        else:
            ptr = reverse_deal(ptr, decksize)

    # The pointer is now at its starting position. Since the positions of the deck
    # are ordered, this is the number of the card
    return ptr


position = position_of_card(2019, deck)
print(f"2019 is at position {position}")

print(f"{number_of_card(position, 10007)} is at position {position}")
