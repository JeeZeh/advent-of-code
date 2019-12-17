from intcode import Intcode

tape = list(map(int, open("input.txt").readline().split(",")))
comp = Intcode()
a = comp.init(tape.copy())
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
        row.append(chr(d))


for y in range(0, len(grid)):
    row = [x for x in grid[y]]
    print("".join(row))

alignments = []
s = 0
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
            s += x*y
            alignments.append((x, y))

print(s)
load = comp.save()
load[1][0] = 2

print(grid[1][1])
