from collections import defaultdict, deque
from operator import add
from pprint import pprint

grid = defaultdict(str)
p = {0: (1, 0), 1: (-1, 0), 2: (0, -1), 3: (0, 1)}

lines = [list(l.rstrip("\n").replace(" ", "#")) for l in open("input.txt").readlines()]

def check_around(pos, grid):
    for d in p.values():
        adj = tuple(map(add, pos, d))
        if grid[adj] == ".":
            dimension = 1 if d in [p[0], p[3]] else -1
            return (adj, dimension)

for j, y in enumerate(lines):
    for i, x in enumerate(y):
        if x not in [".", "", "#"]:
            if 0 < j < len(lines) and 0 < i < len(y):
                if x not in ["", " ", ".", "#"]:
                    print(j, i)
                if lines[j][i-1] == ".":
                    lines[j][i] += lines[j][i+1]
                    lines[j][i+1] = "#"
                elif lines[j][i+1] == ".":
                    lines[j][i] = lines[j][i-1] + lines[j][i]
                    lines[j][i-1] = "#"
                elif lines[j+1][i] == ".":
                    lines[j][i] = lines[j-1][i] + lines[j][i]
                    lines[j-1][i] = "#"
                elif lines[j-1][i] == ".":
                    lines[j][i] += lines[j+1][i]
                    lines[j+1][i] = "#"


for i, y in enumerate(lines):
    for j, x in enumerate(y):
        grid[(j, i)] = x

for y in range(len(lines)):
    print("".join([grid[(x, y)] for x in range(len(lines[0]))]))

doors = {k: v for k, v in grid.items() if len(v) == 2 and v not in ["AA", "ZZ"]}
portals = {}
for d, v in doors.items():
    a = check_around(d, grid)
    for d2, v2 in doors.items():
        if d != d2 and v == v2:
            b = check_around(d2, grid)
            portals[d] = b



def get_reachable_from(pos):
    queue = deque([pos])
    destinations = {}
    dists = [{pos: 0}]
    dimension = 0

    while queue:
        pos = queue.popleft()
        for d in p.values():
            n = tuple(map(add, pos, d))
            if n in portals:
                n = portals[n]
                dimension += n[1]
                if n[1] > 1:
                    dists.append({n: 0})
                n =  n[0]
                
            if n in grid and grid[n] != "#" and n not in dists[dimension]:
                v = grid[n]
                dists[dimension][n] = dists[dimension][pos] + 1
                
                if len(v) == 2:
                   destinations[v] = dists[dimension][n]
                    
                queue.append(n)

    return destinations

start = [k for k, v in grid.items() if v == "AA"][0]

dests = get_reachable_from(start)

pprint(dests["ZZ"]-2)
