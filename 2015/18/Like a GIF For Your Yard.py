from math import sqrt
from operator import add
from os import sep


initial = {}

for y, line in enumerate(open("input.txt").read().splitlines()):
    for x, light in enumerate(line):
        initial[(x, y)] = True if light == "#" else False


def step(yard: dict, corners=False):
    size = int(sqrt(len(yard))) - 1

    yard_copy = yard.copy()
    
    if corners:
        for p in {(0, 0), (0, size), (size, 0), (size, size)}:
            yard[p] = True

    for pos, light in yard.items():
        surrounding = 0
        for dy in [-1, 0, 1]:
            for dx in [-1, 0, 1]:
                comp = tuple(map(add, pos, (dx, dy)))
                if comp == pos:
                    continue
                if comp in yard and yard[comp]:
                    surrounding += 1
        if light:
            if not 2 <= surrounding <= 3:
                yard_copy[pos] = False
        elif surrounding == 3:
            yard_copy[pos] = True

    if corners:
        for p in {(0, 0), (0, size), (size, 0), (size, size)}:
            yard_copy[p] = True

    return yard_copy


def print_yard(yard):
    size = int(sqrt(len(yard)))

    rows = []
    for y in range(size):
        row = []
        for x in range(size):
            row.append(yard[(x, y)])
        rows.append("".join(["#" if r else "." for r in row]))

    print("\n".join(rows))
    print()


yard = initial.copy()
for _ in range(100):
    # print_yard(yard)

    yard = step(yard)

# print_yard(yard)
print("Part 1", sum(yard.values()))


yard = initial.copy()
for _ in range(100):
    # print_yard(yard)
    yard = step(yard, True)

print("Part 2", sum(yard.values()))
