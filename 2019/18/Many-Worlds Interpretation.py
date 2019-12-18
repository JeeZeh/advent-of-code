from collections import defaultdict
from operator import add
from pprint import pprint
grid = defaultdict(str)
p = {0: (1, 0), 1: (-1,0), 2:(0,-1), 3:(0, 1)}

lines = open('input.txt').readlines()

for i, y in enumerate(lines):
    for j, x in enumerate(y):
        grid[(j, i)] = x.rstrip()


pos = [k for k, v in grid.items() if v == "@"][0]

def explore(grid, pos, explored, locks, keys, steps, x):
    explored.append(pos)
    v = grid[pos]
    if v == "#" or v == "":
        return
    
    if v != "." and v != "@":     
        if v.lower() == v:
            keys[v] = (pos, steps, x)
            return
        else:
            locks[v] = (pos, steps, x)
            x.append(v)
        

    for d in p.values():
        new_pos = tuple(map(add, pos, d))
        if new_pos not in explored:
            explore(grid, new_pos, explored,locks, keys, steps + 1, x.copy())

def find_reachable(pos, grid):
    explored, locks, keys = [pos], {}, {}

    for d in p.values():
        explore(grid, tuple(map(add, pos, d)), explored, locks, keys, 1, [])
    
    return (locks, keys, explored)

def try_collect(order, grid, pos):
    steps = 0
    collected = []
    while order:
        locks, keys, explored = find_reachable(pos, grid)
        nk = order.pop()
        
        if nk in keys:
            if not keys[nk][2] or all(e in collected for e in keys[nk][2]):
                collected.append(nk.upper())
                steps+= keys[nk][1]
                grid[keys[nk][0]] = "."
                pos = keys[nk][0]
            else:
                return -1
        else:
            return -1

    return steps 
        

locks, keys, explored = find_reachable(pos, grid)
full = [k[0] for k in keys.items()]
n_keys = len(full)

path = {}
lens = []
def recurse(pos, grid, have, branch, steps):
    locks, keys, explored = find_reachable(pos, grid)
    possible = {k: v for k, v in keys.items() if not v[2] or all(e.lower() in have for e in v[2])}
    if not keys:
        lens.append(steps)
        return steps
    elif possible:
        for k, v in possible.items():
            alt_grid = grid.copy()
            alt_steps = steps
            pos = v[0]
            alt_grid[v[0]] = "."
            alt_steps += v[1]
            b = recurse(pos, alt_grid, [k] + have.copy(), {}, alt_steps)
            branch[k] = b 
        return branch
    else:
        return {}
    
    

recurse(pos, grid, [], path, 0)



pprint(path)
pprint(min(lens))
        
# a = try_collect(['a', 'b', 'c', 'd', 'e', 'f'][::-1], grid.copy(), pos)
# print(a)
        
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


