from time import sleep
from math import prod, sqrt
import os

course = open("input.txt").readlines()
wrap_x = len(course[0]) - 1
wrap_y = len(course) - 1
max_x = len(course) + len(course[0])


def get_treejectory(slope):
    right, down = slope
    trees, x, y = [], 0, 0
    while y < len(course):
        if course[y][x] == "#":
            trees.append((x, y))

        x = (x + right) % wrap_x
        y += down

    return trees


# print(len(get_treejectory((3, 1))))
# print(prod(map(get_treejectory, [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])))

trees = set()
trail = set()

def render_slice(width, height, center_x, center_y, pos_x, pos_y):
    display = ""
    for y in range(center_y, height + center_y):
        row = []
        for x in range(center_x, width + center_x):
            symbol = course[y % wrap_y][x % wrap_x]
            player_char = course[pos_y % wrap_y][pos_x % wrap_x]
            char = "@" if (x, y) == (pos_x, pos_y) else symbol if (x, y) not in trail else " "
            if char != "@":
                if (x, y) in trees and char == "#":
                    char = f"\033[31m{char}\033[0m"
                elif char == ".":
                    char = f"\033[2m{char}\033[0m"
                elif char == "#":
                    char = f"\033[32m{'Ä'}\033[0m"
            else:
                char = f"\033[1;4m{char}\033[0m"
            if player_char == "#":
                trees.add((pos_x, pos_y))
            if player_char == ".":
                trail.add((pos_x, pos_y))
            row.append(char)
        display += "".join(row) + "\n"

    print("\033[F" * 100)
    print(display)
    print(f"Trees hit: \u001b[31m{len(trees)}\u001b[0m")


def cls():
    if os.name == "posix":
        os.system("clear")
    else:
        os.system('cls')



def play_path(slope, width, height):
    cls()
    right, down = slope
    refresh = 0.05 * max(right, down)
    sled_x, sled_y = 0, 0
    offset_x, offset_y = width // 2, height // 2
    max_x = wrap_y * right - width - right + 1
    max_y = wrap_y - offset_y * 2
    while sled_y < wrap_y:
        origin_x = max(0, min(sled_x - offset_x, max_x))
        origin_y = max(0, min(sled_y - offset_y, max_y))
        render_slice(width, height, origin_x, origin_y, sled_x, sled_y)
        sleep(refresh)
        sled_x += right
        sled_y += down


play_path((3,2), 100, 30)