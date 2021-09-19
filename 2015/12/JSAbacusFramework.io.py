from json import load

obj = load(open("input.txt"))


def number_values(d, ignore_red=False):
    if isinstance(d, int):
        return d

    if isinstance(d, list):
        return sum(map(lambda v: number_values(v, ignore_red), d))

    if isinstance(d, dict):
        if ignore_red and "red" in d.values():
            return 0
        return sum(map(lambda v: number_values(v, ignore_red), d.values()))

    return 0


print("Part 1:", number_values(obj))
print("Part 2:", number_values(obj, ignore_red=True))