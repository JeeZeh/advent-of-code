from collections import OrderedDict
from pprint import pprint
import numpy as np
import time

system = OrderedDict()
initial = OrderedDict()
init_np = None
locks = {}


def init():
    global system, initial, init_np, locks
    system = OrderedDict()
    locks = {}
    with open("input.txt") as f:
        for line in f:
            p = [int(p[2:]) for p in line.rstrip()[1:-1].split(", ")]
            system[tuple(p)] = [0, 0, 0]
    initial = system.copy()
    init_np = np.array(list(initial.keys()))


def step(curr_step):
    global system
    buffer = OrderedDict()

    for m1, s1 in system.items():
        for m2 in list(filter(lambda x: x != m1, system.keys())):
            for a in range(3):
                if m1[a] < m2[a]:
                    s1[a] += 1
                if m1[a] > m2[a]:
                    s1[a] -= 1

        m1 = list(m1)
        for a in range(3):
            m1[a] += s1[a]
        buffer[tuple(m1)] = s1

    system = buffer
    for i in range(3):
        if not np.array(list(system.values()))[:, i].any():
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
