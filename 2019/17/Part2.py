from intcode import Intcode

tape = list(map(int, open("input.txt").readline().split(",")))
tape[0] = 2
comp = Intcode()
drive = comp.init(tape.copy())

R = "A,B,A,C,A,C,B,C,C,B"
A = "L,4,L,4,L,10,R,4"
B = "R,4,L,4,L,4,R,8,R,10"
C = "R,4,L,10,R,10"

data = []
try:
    while True:      
        if len(data) > 3 and data[-1] == 110:
            break
        data.append(next(drive))
except Exception as si:
    print(data)


for r in R:
    print(next(drive))
    a= print(drive.send(ord(r)))
    if a is not None:
        print(chr(a))

try:
    while True:      
        if len(data) > 3 and data[-1] == 110:
            break
        data.append(next(drive))
except Exception as si:
    print(data)


for a in A:
    drive.send(ord(a))
for b in B:
    drive.send(ord(b))
for c in C:
    drive.send(ord(c))
drive.send("y")


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

