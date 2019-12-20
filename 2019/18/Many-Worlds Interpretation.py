from collections import defaultdict, deque
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

keys = get_reachable_from(start)

b = ["d", "t", "j", "o", "n", "s", "e", "r", "x", "g", "z", "k", "l", "m", "h", "i", "w", "c", "y", "u", "v", "q", "a", "b", "p", "f"]
ext = [ "x", "g", "z", "k", "l", "m", "h", "i", "w", "c", "y", "u", "v", "q", "a", "b", "p", "f"]


for perm in itertools.permutations(["d", "t", "j", "o", "n", "s", "e", "r"]):
    a = try_collect((list(perm)+ext)[::-1], grid.copy(), pos)
    print(a)


def generate_pairs():
    pairs = defaultdict(dict)
    pairs["@"] = keys

    for k, v in grid_keys.items():
        dests = get_reachable_from(v)
        for k2, v2 in dests.items():
            pairs[k][k2] = v2

    return pairs



n_keys = len(grid_keys) - 1
best_states = {}
<<<<<<< HEAD

with open('pairs.json', mode="w+") as f:
    f.write(json.dumps(generate_pairs()))

=======
>>>>>>> 5d3adb0b40b55289bf3af7b5a295ccb0c02e3f6d
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
<<<<<<< HEAD
    
_, keys, _ = find_reachable(pos, grid)
starting_keys = {k: v for k, v in keys.items() if not v[2]}
get_paths("@", [], 0)
print(lowest)

path = {}
lens = []
def recurse(pos, grid, have, steps):
    
    locks, keys, explored = find_reachable(pos, grid)
    possible = [(k, v) for k, v in keys.items() if not v[2] or all(e.lower() in have for e in v[2])]
    if not keys:
        lens.append((steps, have))
        return
    elif possible:
        for k, v in possible.items():
            alt_grid = grid.copy()
            pos = v[0]
            alt_grid[v[0]] = "."
            recurse(pos, alt_grid, have + [k], steps + v[1])
        return
    
    

# recurse(pos, grid, [], 0)



# a = try_collect(['a', 'b', 'c', 'e', 'd', 'f'][::-1], grid.copy(), pos)
# print(a)
        
# a = try_collect(['a', 'b', 'e', 'c', 'd', 'f'][::-1], grid.copy(), pos)
# print(a)


# def simulate(k, grid, steps, can_unlock, have_keys, depth):
#     if depth == 0:
#         return steps + k[2]
#     steps+= k[2]
#     pos = k[0]
#     have_keys.append(k[1].upper())
#     grid[pos] = "."
#     locks, keys, explored = find_reachable(pos, grid)
#     return play(locks, keys, explored, steps, can_unlock, have_keys, grid, depth-1)



# def play(locks, keys, explored, steps, can_unlock, have_keys, grid, depth):
#     while len(locks) > 0:
#         can_unlock = sorted([lock for lock in locks if lock[1] in have_keys and not lock[3]], key=lambda x: x[2])
#         can_collect = [key for key in keys if not key[3]]
#         if can_unlock:
#             u = can_unlock[0]
#             steps+= u[2]
#             pos = u[0]
#         elif can_collect:
#             length = [(simulate(k, grid.copy(), steps, can_unlock.copy(), have_keys.copy(), depth), k) for k in can_collect]
#             length.sort()
#             k = length[0][1]
#             steps+= k[2]
#             pos = k[0]
#             have_keys.append(k[1].upper())
#         grid[pos] = "."
#         locks, keys, explored = find_reachable(pos, grid)

#     return steps
=======
>>>>>>> 5d3adb0b40b55289bf3af7b5a295ccb0c02e3f6d


t = time_ns()
get_paths("@", [], 0)
print((time_ns() - t)/1000/1000)
print(lowest)
