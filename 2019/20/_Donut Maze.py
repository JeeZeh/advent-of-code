from collections import defaultdict, deque, namedtuple
from operator import add
from pprint import pprint

grid = defaultdict(str)
directions = {0: (1, 0), 1: (-1, 0), 2: (0, -1), 3: (0, 1)}

lines = [list(l.rstrip("\n").replace(" ", "#")) for l in open("input.txt").readlines()]


def check_around(pos, grid):
    for d in directions.values():
        adj = tuple(map(add, pos, d))
        if grid[adj] == ".":
            return adj


for j, y in enumerate(lines):
    for i, x in enumerate(y):
        if x not in [".", "", "#"]:
            if 0 < j < len(lines) and 0 < i < len(y):
                if x not in ["", " ", ".", "#"]:
                    pass
                if lines[j][i - 1] == ".":
                    lines[j][i] += lines[j][i + 1]
                    lines[j][i + 1] = "#"
                elif lines[j][i + 1] == ".":
                    lines[j][i] = lines[j][i - 1] + lines[j][i]
                    lines[j][i - 1] = "#"
                elif lines[j + 1][i] == ".":
                    lines[j][i] = lines[j - 1][i] + lines[j][i]
                    lines[j - 1][i] = "#"
                elif lines[j - 1][i] == ".":
                    lines[j][i] += lines[j + 1][i]
                    lines[j + 1][i] = "#"


for i, y in enumerate(lines):
    for j, x in enumerate(y):
        grid[(j, i)] = x

# for y in range(len(lines)):
#     print("".join([grid[(x, y)] for x in range(len(lines[0]))]))

doors = {k: v for k, v in grid.items() if len(v) == 2 and v not in ["AA", "ZZ"]}
portals = {}
for d, v in doors.items():
    a = check_around(d, grid)
    for d2, v2 in doors.items():
        if d != d2 and v == v2:
            b = check_around(d2, grid)
            portals[d] = b


def is_outside_portal(pos):
    x, y = pos

    if x - 4 < 0 or x + 4 > len(lines[0]) or y - 4 < 0 or y + 4 > len(lines):
        return True

    return False


Pos = namedtuple("Pos", ["coord", "depth"])


def get_reachable_from(pos: Pos):
    queue = deque([pos])
    destinations = {}
    dists = {pos: 0}

    while queue:
        pos = queue.popleft()
        for d in directions.values():
            position = Pos(tuple(map(add, pos.coord, d)), pos.depth)

            # Walk into portal to teleport
            if position.coord in portals:
                is_outside = is_outside_portal(position.coord)
                # We can't teleport using any outer tiles when we're in dimension 0
                if position.depth == 0 and is_outside:
                    pass
                else:
                    # Either go back up (-1) or further down
                    position = Pos(portals[position.coord], position.depth + (-1 if is_outside else 1))

            # For each step that isn't into a wall, add 1 top the current distance to reach whatever
            if grid.get(position.coord, "#") != "#" and position not in dists:
                tile = grid[position.coord]
                dists[position] = dists[pos] + 1

                # We've walked into a portal (excluding AA and ZZ when in another dimension)
                if len(tile) == 2 and not (tile in {"ZZ", "AA"} and position.depth > 0):
                    destinations[position] = dists[position]
                    if tile == "ZZ":
                        print(tile, position, dists[position] - 2)
                        return destinations

                queue.append(position)

    return destinations


start = [k for k, v in grid.items() if v == "AA"][0]

get_reachable_from(Pos(start, 0))

# print(dests)
