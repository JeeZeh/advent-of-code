from collections import deque


def parse_groups(line: str):
    total = 0
    level = 0
    garbage = False
    removed = 0

    i = 0
    while i < len(line):
        char = line[i]
        skip_remove = False
        
        if char == "{" and not garbage:
            level += 1
        if char == "}" and not garbage:
            total += level
            level -= 1
        if char == "<" and not garbage:
            garbage = True
            skip_remove = True
        if char == ">" and garbage:
            garbage = False
        if char == "!":
            skip_remove = True
            i += 1

        if garbage and not skip_remove:
            removed += 1

        i += 1

    return total, removed


for line in open("Day9/input").read().splitlines():
    print(parse_groups(line))
