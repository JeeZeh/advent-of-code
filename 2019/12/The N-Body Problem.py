from collections import OrderedDict
from pprint import pprint
import numpy as np
import time

system = np.array([])
initial = np.array([])
init_np = None
locks = {}


def init():
    global system, initial, init_np, locks
    system = np.zeros((0,2,3), dtype=np.int16)
    locks = {}
    with open("input.txt") as f:
        for line in f:
            p = [int(p[2:]) for p in line.rstrip()[1:-1].split(", ")]
            system = np.vstack((system, [[p, [0, 0, 0]]]))
    initial = system.copy()


def step(curr_step):
    global system
    buffer = system.copy()

    for i in range(4):
        for j in range(4):
            if i == j:
                continue
            
            for a in range(3):
                if system[i][0][a] < system[j][0][a]:
                    buffer[i][1][a] += 1
                elif system[i][0][a] > system[j][0][a]:
                    buffer[i][1][a] -= 1

        buffer[i][0] += buffer[i][1]
       
    system = buffer
    for i in range(3):
        v = system[:,1, i]
        if not v.any():
            if i not in locks:
                locks[i] = curr_step + 1


def state_equal(step):
    return initial.keys() == system.keys()


def simulate(steps):
    init()

    if steps == -1:
        c = 1
        step(c)
        while len(locks) < 3:
            step(c)
            c += 1
        print(c + 1)
    else:
        for i in range(steps):
            step(i)


def get_energy():
    return sum(
        [sum(map(abs, list(p))) * sum(map(abs, list(v))) for p, v in system.items()]
    )

ns = time.time_ns()
simulate(-1)
v = list(locks.values())
lcm = np.lcm(np.lcm(v[0], v[1]), v[2], dtype=np.int64)
print(2*lcm)
print((time.time_ns() - ns) / 1000000000)
