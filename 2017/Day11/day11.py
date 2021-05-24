import operator


directions = {
    "n": (0, 2),
    "ne": (1, 1),
    "e": (2, 0),
    "se": (1, -1),
    "s": (0, -2),
    "sw": (-1, -1),
    "w": (-2, 0),
    "nw": (-1, 1),
}


distances = []
for path in open("Day11/input").read().splitlines():
    pos = (0, 0)

    for step in path.split(","):
        pos = tuple(map(operator.add, pos, directions[step]))
        distances.append(sum(map(abs, pos)) // 2)

    print(distances[-1])
    print(max(distances))
