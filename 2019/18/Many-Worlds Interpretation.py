from collections import defaultdict, deque
import itertools
from operator import add
from time import time_ns
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
                if v not in {".", "@", "1", "2", "3", "4"}:
                    if v.lower() == v:
                        keys[v] = {"req": req[n], "dist": dist[n]}
                    else:
                        req[n].append(v.lower())

                queue.append(n)

    return keys


b = ["d", "t", "j", "o", "n", "s", "e", "r", "x", "g", "z", "k", "l", "m", "h", "i", "w", "c", "y", "u", "v", "q", "a", "b", "p", "f"]
ext = [ "x", "g", "z", "k", "l", "m", "h", "i", "w", "c", "y", "u", "v", "q", "a", "b", "p", "f"]


# for perm in itertools.permutations(["d", "t", "j", "o", "n", "s", "e", "r"]):
#     a = try_collect((list(perm)+ext)[::-1], grid.copy(), pos)
#     print(a)


def generate_pairs():
    pairs = defaultdict(dict)
    
    bot1 = [k for k, v in grid.items() if v == "1"][0]
    bot2 = [k for k, v in grid.items() if v == "2"][0]
    bot3 = [k for k, v in grid.items() if v == "3"][0]
    bot4 = [k for k, v in grid.items() if v == "4"][0]
    pairs["1"] = get_reachable_from(bot1)
    pairs["2"] = get_reachable_from(bot2)
    pairs["3"] = get_reachable_from(bot3)
    pairs["3"] = get_reachable_from(bot4)

    for k, v in grid_keys.items():
        dests = get_reachable_from(v)
        for k2, v2 in dests.items():
            pairs[k][k2] = v2

    return pairs



n_keys = len(grid_keys) - 1
best_states = {}

with open('pairs.json', mode="w+") as f:
    f.write(json.dumps(generate_pairs()))

pairs = generate_pairs()
lowest = 100000
perms = 0


def get_paths(pos, keys, steps):
    hashed = (pos, "".join(keys))
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
        if steps < lowest:
            lowest = steps
        return

    for dest, v in potential.items():
        ckeys = keys.copy()
        ckeys.append(dest)
        get_paths(dest, ckeys, steps + v["dist"])


t = time_ns()
get_paths("@", [], 0)
print((time_ns() - t)/1000/1000)
print(lowest)
