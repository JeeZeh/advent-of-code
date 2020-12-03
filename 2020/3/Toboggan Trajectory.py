from math import prod

course = open("input.txt").readlines()
wrap = len(course[0]) - 1


def get_treejectory(slope):
    r, d = slope
    trees, x, y = 0, 0, 0
    while y < len(course):
        if course[y][x] == "#":
            trees += 1

        x = (x + r) % wrap
        y += d

    return trees


print(get_treejectory((3, 1)))
print(prod(map(get_treejectory, [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])))
