import os, sys

PACKAGE_PARENT = ".."
SCRIPT_DIR = os.path.dirname(
    os.path.realpath(os.path.join(os.getcwd(), os.path.expanduser(__file__)))
)
sys.path.append(os.path.normpath(os.path.join(SCRIPT_DIR, PACKAGE_PARENT)))

from Day10.day10 import knot_hash
from operator import add

dirs = {(0, 1), (1, 0), (0, -1), (-1, 0)}


def count_squars(rows):
    return sum(sum(map(int, r)) for r in map("".join, rows))


def generate_grid(key: str):
    return (
        map("{:04b}".format, map(lambda x: int(x, 16), knot_hash(f"{key}-{x}")))
        for x in range(128)
    )


def part_one(key: str):
    print(f"Total bits: {count_squars(generate_grid(key))}")


def part_two(key: str):
    seen = set()
    grid = [list(map(int, "".join(line))) for line in generate_grid(key)]

    def explore(pos: tuple[int, int]):
        x, y = pos

        if (0 > x or x >= len(grid[0])) or (0 > y or y >= len(grid)):
            return

        bit = grid[y][x]

        if bit == 0 or pos in seen:
            return

        seen.add(pos)

        for dir in dirs:
            explore(tuple(map(add, pos, dir)))

    regions = 0

    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if (x, y) not in seen and grid[y][x] == 1:
                explore((x, y))
                regions += 1

    print(f"Regions: {regions}")


if __name__ == "__main__":
    key = "uugsqrei"
    part_one(key)
    part_two(key)