from pprint import pprint
from operator import add
from collections import defaultdict

p = {0: (1, 0), 1: (-1, 0), 2: (0, -1), 3: (0, 1)}
grid = defaultdict(str)
seen_states = set()


for y, line in enumerate(open("input.txt")):
    for x, v in enumerate(line.rstrip()):
        grid[(x, y)] = v

WIDTH, HEIGHT = max(grid.keys())[0], max(grid.keys(), key=lambda x: x[1])[1]


def print_grid():
    for y in range(HEIGHT + 1):
        print("".join([grid[(x, y)] for x in range(WIDTH + 1)]))


def check_around(grid, pos):
    return [
        grid[pos]
        for pos in [tuple(map(add, pos, d)) for d in p.values()]
        if min(pos) >= 0 and max(pos) <= 4
    ]


def tick():
    begin_state = grid.copy()
    for pos, bug in begin_state.items():
        adjs = check_around(begin_state, pos)
        if bug == "." and adjs.count("#") in [1, 2]:
            grid[pos] = "#"
        elif bug == "#" and not adjs.count("#") == 1:
            grid[pos] = "."

def get_biodiversity(grid):
    bugs = [(k[1]*5) + k[0] for k, v in grid.items() if v == "#"]
    return [pow(2, b) for b in bugs]

while True:
    tick()
    hashed_bugs = tuple(sorted(grid.items()))
    if hashed_bugs in seen_states:
        print("SEEN!")
        print_grid()
        levels = get_biodiversity(grid)
        print(levels)
        print(sum(levels))
        break
    else:
        seen_states.add(hashed_bugs)
