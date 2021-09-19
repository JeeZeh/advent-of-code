from collections import defaultdict
from itertools import permutations

nodes = defaultdict(dict)


for route in open("input.txt").read().splitlines():
    # Tambi to Arbre = 40
    a, _, b, _, dist = route.split(" ")
    
    nodes[a][b] = int(dist)
    nodes[b][a] = int(dist)
    
routes = {}
    
for p in permutations(nodes.keys()):
    if tuple(reversed(p)) in routes:
        continue
    distance = 0
    for i, n in enumerate(p[:-1]):
        distance += nodes[n][p[i+1]]
        
    routes[p] = distance
    
print("Part 1: ", min(routes.items(), key=lambda kv: kv[1]))
print("Part 1: ", max(routes.items(), key=lambda kv: kv[1]))