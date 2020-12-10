from collections import defaultdict

adapters = sorted(map(int, open("input.txt"))) 
adapters = [0] + adapters + [max(adapters) + 3]

difference = defaultdict(int)

for i, a in enumerate(adapters[:-1]):
    difference[adapters[i + 1] -a] += 1
    
print(difference[1] * difference[3])

cache = {}

# Could not get this without a hint to try recursion, I had been trying to eliminate 3-jolt pairs and get 
# some from of permutations :(
# https://www.reddit.com/r/adventofcode/comments/kacdbl/2020_day_10c_part_2_no_clue_how_to_begin/gf9q9ji/
def get_ways_forward(starting):   
    if starting not in adapters:
        return 0
    if starting == adapters[-1]:
        return 1
    if precomputed := cache.get(starting):
        return precomputed
    
    ways = sum(get_ways_forward(starting + i) for i in range(1, 4))

    cache[starting] = ways
    return ways

print(get_ways_forward(0))
