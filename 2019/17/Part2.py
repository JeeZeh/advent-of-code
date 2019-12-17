from intcode import Intcode
tape = list(map(int, open("input.txt").readline().split(",")))
tape[0] = 2
comp = Intcode()

R = "A,B,A,C,A,C,B,C,C,B"
A = "L,4,L,4,L,10,R,4"
B = "R,4,L,4,L,4,R,8,R,10"
C = "R,4,L,10,R,10"

q = list(map(ord, list("\n".join([R, A, B, C, "n", ""]))[::-1]))
drive = comp.init(tape.copy(), q)

data = []

try:
    while True:
        x = next(drive)
        if x is not None:
            data.append(x)       
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

print(data[-1])



