from collections import defaultdict

paths = open("real.txt").read().splitlines()


dirs = {
    "nw": (-0.5, 0.5),
    "w": (-1, 0),
    "sw": (-0.5, -0.5),
    "ne": (0.5, 0.5),
    "e": (1, 0),
    "se": (0.5, -0.5),
}
floor = defaultdict(bool)


def trace(path: str, floor):
    pos = (0, 0)

    while path:
        if path[0] in ("n", "s"):
            d = dirs[path[0:2]]
            path = path[2:]
        else:
            d = dirs[path[0]]
            path = path[1:]

        pos = pos[0] + d[0], pos[1] + d[1]

    return pos


for path in paths:
    pos = trace(path, floor)
    floor[pos] = not floor[pos]

print(sum(floor.values()))


def expand(floor):
    new_state = floor.copy()
    for pos, tile in floor.items():
        for d in dirs.values():
            p = pos[0] + d[0], pos[1] + d[1]
            new_state[p]

    floor |= new_state


def step(floor):
    new_state = floor.copy()
    eval_state = floor.copy()
    for pos, tile in floor.items():
        adj = 0
        for d in dirs.values():
            p = pos[0] + d[0], pos[1] + d[1]
            adj += eval_state[p]

        if tile and adj == 0 or adj > 2:
            new_state[pos] = False
        elif not tile and adj == 2:
            new_state[pos] = True

    return eval_state | new_state


expand(floor)
for _ in range(100):
    floor = step(floor)

print(sum(floor.values()))
