cards = map(str.strip, open("input.txt").readlines())


def get_pos(card):
    low, high = 0, 2 ** (len(card)) - 1

    for c in card:
        if c in ("B", "R"):
            low += (high - low) // 2 + 1
        else:
            high -= (high - low) // 2 + 1

    return low


ids = [(get_pos(card[:7]) * 8) + get_pos(card[-3:]) for card in cards]

print(max(ids))
print(sum(range(min(ids), max(ids) + 1)) - sum(ids))
