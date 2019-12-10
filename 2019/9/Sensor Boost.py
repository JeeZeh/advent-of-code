from collections import defaultdict
from time import sleep


p = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1}
o = list(map(int, open("input.txt").readline().split(",")))
mem = defaultdict(int)
ptr = 0
r = 0

for i, data in enumerate(o):
    mem[i] = data

ops = mem


def get_pos(mode, param, param_idx):
    if mode == 0:
        return param[param_idx]
    elif mode == 1:
        return ptr + param_idx + 1
    elif mode == 2:
        return r + param[param_idx]


def write(mode, param, param_idx, value):
    global ops, ptr
    ops[get_pos(mode, param, param_idx)] = value
    ptr += len(param) + 1


def read(mode, param, param_idx):
    global ops, ptr
    print(ops[get_pos(mode, param, param_idx)])
    ptr += len(param) + 1


while ops[ptr] != 99:
    op = f"{ops[ptr]:05}"
    code = int(op[-2:])
    e = p[code]
    modes = list(map(int, list(op[:-2][::-1][:e])))
    params = [ops[ptr + i] for i in range(1, e + 1)]
    data = [ops[get_pos(modes[i], params, i)] for i in range(e)]

    if code == 1:
        write(modes[-1], params, e - 1, data[0] + data[1])
    elif code == 2:
        write(modes[-1], params, e - 1, data[0] * data[1])
    elif code == 3:
        write(modes[-1], params, e - 1, 2)
    elif code == 4:
        read(modes[-1], params, e - 1)
    elif code == 5:
        if data[0] != 0:
            ptr = data[1]
        else:
            ptr += e + 1
    elif code == 6:
        if data[0] == 0:
            ptr = data[1]
        else:
            ptr += e + 1
    elif code == 7:
        write(modes[-1], params, e - 1, int(data[0] < data[1]))
    elif code == 8:
        write(modes[-1], params, e - 1, int(data[0] == data[1]))
    elif code == 9:
        r += data[0]
        ptr += 2
