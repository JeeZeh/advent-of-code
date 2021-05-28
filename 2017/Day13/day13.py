from math import prod


walls = list(map(lambda x: tuple(map(int, x.split(": "))), open("Day13/input")))


def pass_through(starting):
    return [w for w in walls if ( (w[0] + starting) % ((2 * w[1])-2)) == 0]
    

print(f"Part 1: {sum(map(prod, pass_through(0)))}")

# Naive
n = 0
while pass_through(n):
    n += 1
    
print(f"Part 2: {n} picoseconds")