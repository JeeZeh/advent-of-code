from intcode import Intcode

a = Intcode().init()
data = []
try:
    while True:
        data.append(next(a))
except StopIteration as si:
    print(si)

grid = []
row = []
for d in data:
    if d == 10:
        grid.append(row)
        row = []
    else:
        char = "#" if d == 35 else "."
        row.append(char)


for y in range(1, len(grid)):
    row = [x for x in grid[y]]
    print("".join(row))

alignments = []
a = 0
for y in range(1, len(grid) - 2):
    for x in range(1, len(grid[y]) - 2):
        if (
            grid[y][x] == "#"
            and grid[y - 1][x] == "#"
            and grid[y + 1][x] == "#"
            and grid[y][x - 1] == "#"
            and grid[y][x + 1] == "#"
        ):
            print(f"Intersection at {(x,y)}")
            a += x*y
            alignments.append((x, y))

print(a)
sum()

print(grid[1][1])
