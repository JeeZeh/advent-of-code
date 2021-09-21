from collections import defaultdict
from itertools import islice, permutations
from typing import Tuple


def parse_input(line: str) -> Tuple[str, int, str]:
    parts = line.split()
    a = parts[0]
    op = -1 if parts[2] == "lose" else 1
    units = int(parts[3])
    b = parts[-1][:-1]

    return a, op * units, b


knights = defaultdict(dict)

for a, effect, b in map(parse_input, open("input.txt").read().splitlines()):
    knights[a][b] = effect


def total_happiness(permutation: Tuple) -> int:
    total = 0
    for i, k in enumerate(permutation):
        total += (
            knights[k][permutation[(i + 1) % len(permutation)]]
            + knights[k][permutation[(i - 1) % len(permutation)]]
        )

    return total


# Sawada's algorithm might be good here http://www.cis.uoguelph.ca/~sawada/papers/alph.pdf
arrangements = [(total_happiness(p), p) for p in permutations(knights.keys())]
print("Part 1:", max(arrangements))

for knight in list(knights.keys()):
    knights["You"][knight] = 0
    knights[knight]["You"] = 0

arrangements = [(total_happiness(p), p) for p in permutations(knights.keys())]
print("Part 2:", max(arrangements))
