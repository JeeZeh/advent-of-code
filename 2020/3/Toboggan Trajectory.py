from time import sleep
from math import prod
import os

course = open("input.txt").readlines()
wrap_x = len(course[0]) - 1
wrap_y = len(course) - 1
max_x = len(course)+len(course[0])

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


def render_slice(width, height, center_x, center_y, pos_x, pos_y, slope):
    trees = set()
    y = pos_x - slope[1]
    for x in reversed(range(center_x, pos_x, slope[0])):
        if course[y][x % wrap_x] == "#":
            trees.add((x, y))
        y -= slope[1]
    
    for y in range(center_y, height + center_y):
        row = []
        for x in range(center_x, width + center_x):
            char = "@" if (x,y) == (pos_x, pos_y) else course[y % wrap_y][x % wrap_x]
            if (x, y) in trees:
                char = f"\u001b[31m{char}\u001b[0m"
            row.append(char)
        print("".join(row))

def cls():
   # for mac and linux(here, os.name is 'posix')
   if os.name == 'posix':
      os.system('clear')
   else:
      # for windows platfrom
      os.system('cls')
   # print out some text

def play_path(slope, width, height):
    right, down = slope
    sled_x, sled_y = 0, 0
    offset_x, offset_y = width//2, height//2
    max_x =  wrap_y*right - width - right + 1
    max_y = wrap_y - offset_y*2
    while sled_y < wrap_y:
        cls()
        origin_x = max(0, min(sled_x - offset_x, max_x))
        origin_y = max(0, min(sled_y - offset_y, max_y))
        render_slice(width, height, origin_x, origin_y, sled_x, sled_y, slope)
        sleep(0.05)
        sled_x += right
        sled_y += down

play_path((2, 1), 50, 20)