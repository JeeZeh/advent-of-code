from collections import defaultdict

dirs = [
    (x, y, z, w)
    for w in range(-1, 2)
    for z in range(-1, 2)
    for y in range(-1, 2)
    for x in range(-1, 2)
    if (x, y, z, w) != (0, 0, 0, 0)
]

pocket = open("real.txt").read().splitlines()


def generate_space(layer):
    space = defaultdict(int)
    for w in range(-1, 2):
        for z in range(-1, 2):
            for y in range(-1, len(layer) + 1):
                for x in range(-1, len(layer[0]) + 1):
                    space[(x, y, z, w)] = 0

    for y, r in enumerate(layer):
        for x, c in enumerate(r):
            space[(x, y, 0, 0)] = 1 if c == "#" else 0

    return space


space = generate_space(pocket)


def cycle(space):
    state = space.copy()
    eval_space = space.copy()
    for pos, active in space.items():
        x, y, z, w = pos
        surrounding = []
        for dx, dy, dz, dw in dirs:
            look_at = (x + dx, y + dy, z + dz, w + dw)
            if eval_space[look_at]:
                surrounding.append(look_at)

        if active and len(surrounding) not in (2, 3):
            state[(x, y, z, w)] = 0
        elif not active and len(surrounding) == 3:
            state[(x, y, z, w)] = 1

    return eval_space | state


for t in range(6):
    space = cycle(space)

print(sum(v for k, v in space.items()))