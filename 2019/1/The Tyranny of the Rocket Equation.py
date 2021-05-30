modules = list(map(int, open("input.txt")))


def fuel(mass, rec=False):
    needs = (mass // 3) - 2

    if not rec:
        return needs

    if needs <= 0:
        return 0

    return needs + fuel(needs, True)


print(f"Part 1: {sum(map(fuel, modules))}")

print(f"Part 2: {sum(map(lambda x: fuel(x, True), modules))}")
