from collections import defaultdict
from time import sleep
from pprint import pprint
from operator import add
from intcode import Intcode

############### FUNCTIONS #####################

def paint(starting):
    x = Intcode().init()
    plate = defaultdict(int)
    pos = (0, 0)
    c = x.send(starting)
    operators = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    operator_idx = 0
    d = next(x)
    try:
        while True:
            plate[pos] = c
            operator_idx += 1 if d == 1 else -1
            operator_idx %= 4

            pos = tuple(map(add, pos, operators[operator_idx]))
            under = plate[pos]
            next(x)
            c = x.send(under)
            d = next(x)
    except Exception as e:
        pass
    return plate

############### PART 1 #####################

print(len(paint(0)))

############### PART 2 #####################

plate = paint(1)

# Get the painted
M = -1 * min(plate.keys(), key=lambda tup: tup[1])[1]  # Offset Y
W = [tuple(map(add, k, (0, M))) for k, v in plate.items() if v == 1] # Perform offset on white tiles
for y in range(M, -1, -1):
    row = ""
    for x in range(max(W)[0] + 1):
        row += "#" if (x, y) in W else " "
    print(row)
