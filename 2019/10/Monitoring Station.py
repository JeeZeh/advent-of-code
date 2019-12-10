from pprint import pprint
from math import atan2, sqrt
text = open('input.txt').readlines()

asteroids = []

for i, line in enumerate(text):
    for j, obj in enumerate(line):
        if obj == '#':
            asteroids.append((j, i))


def get_los(x):
    rays = []
    for a in list(filter(lambda p: p != x, asteroids)):
        dx = (a[0] - x[0])
        dy = (a[1] - x[1])

        rays.append((sqrt(dx**2 + dy**2), atan2(dy, dx)))
    return rays
    
rays = []
for a in asteroids:
    rays.append((a, len(dict.fromkeys(get_los(a)))))

pprint(max(rays, key=lambda tup: tup[1][1]))
