from math import prod


walls = list(map(lambda x: tuple(map(int, x.split(": "))), open("Day13/input")))


def pass_through(s):
    iter_ = (w for w in walls if ((w[0] + s) % ((2 * w[1]) - 2)) == 0)

    return any(iter_) if s else list(iter_)


print(f"Part 1: {sum(map(prod, pass_through(0)))}")


n = 0
while pass_through(n := n + 1): pass

print(f"Part 2: {n} picoseconds")