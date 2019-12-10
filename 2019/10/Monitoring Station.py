from pprint import pprint
from math import atan2, sqrt, pi
from collections import defaultdict
from time import sleep

text = open("input.txt").readlines()

field = []

for i, line in enumerate(text):
    for j, obj in enumerate(line):
        if obj == "#":
            field.append((j, i))


def get_los(x):
    rays = defaultdict(list)
    for a in list(filter(lambda p: p != x, field)):
        dx = x[0] - a[0]
        dy = x[1] - a[1]
        rays[atan2(dy, dx)].append((sqrt(dx ** 2 + dy ** 2), a))

    return rays


def rotate_targets(targets):
    t = list(targets.items())
    s = sorted(t)
    f = [r for r in s if r[0] >= pi / 2]
    rot = s.index(f[0])
    return s[rot:] + s[:rot]


def destroy_all(targets):
    destroyed = []
    rot = rotate_targets(targets)
    while len(destroyed) < len(field) - 1:
        for degree, asteroids in rot:
            if len(asteroids) > 0:
                target = min(asteroids)
                destroyed.append(target[1])
                targets[degree].remove(target)

    return destroyed


def get_station(samples):
    station = max([(len(v.keys()), k) for k, v in samples.items()])
    return station[1]


rays = {a: get_los(a) for a in field}
station = get_station(rays)
print(station)
destoryed = destroy_all(rays[station])

print(destoryed[199][0] * 100 + destoryed[199][1])
