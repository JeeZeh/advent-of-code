from collections import defaultdict, deque
from operator import add
from pprint import pprint
import json

grid = defaultdict(str)
p = {0: (1, 0), 1: (-1, 0), 2: (0, -1), 3: (0, 1)}

lines = open("input.txt").readlines()

for i, y in enumerate(lines):
    for j, x in enumerate(y):
        grid[(j, i)] = x.rstrip()

grid_keys = {
    v: k
    for k, v in grid.items()
    if v != "#" and v != "." and v != "" and v.lower() == v
}

start = [k for k, v in grid.items() if v == "@"][0]


def get_reachable_from(pos):
    queue = deque([pos])
    keys = {}
    dist = {pos: 0}
    req = {pos: []}

    while queue:
        pos = queue.popleft()
        for d in p.values():
            n = tuple(map(add, pos, d))
            if n in grid and grid[n] != "#" and n not in dist:
                v = grid[n]
                dist[n] = dist[pos] + 1
                req[n] = req[pos].copy()
                if v != "." and v != "@":
                    if v.lower() == v:
                        keys[v] = {"req": req[n], "dist": dist[n]}
                    else:
                        req[n].append(v.lower())

                queue.append(n)

    return keys


def generate_pairs():
    pairs = defaultdict(dict)
    keys = get_reachable_from(start)
    pairs["@"] = keys

    for k, v in grid_keys.items():
        dests = get_reachable_from(v)
        for k2, v2 in dests.items():
            pairs[k][k2] = v2

    return pairs


keys = get_reachable_from(start)
n_keys = len(grid_keys) - 1
best_states = {}
pairs = generate_pairs()
lowest = 100000
perms = 0


def get_paths(pos, keys, steps):
    hashed = (pos, frozenset(keys))
    if hashed in best_states:
        if best_states[hashed] <= steps:
            return

    best_states[hashed] = steps

    global lowest, perms
    potential = {
        k: v
        for k, v in pairs[pos].items()
        if all(x in keys for x in v["req"]) and k not in keys
    }
    if len(keys) == n_keys:
        perms += 1
        if steps < lowest:
            lowest = steps
        if perms % 100000 == 0:
            print("Permutations:", perms)
            print("Lowest:", lowest)

        return

    for dest, v in potential.items():
        ckeys = keys.copy()
        ckeys.append(dest)
        get_paths(dest, ckeys, steps + v["dist"])


get_paths("@", [], 0)
print(lowest)
