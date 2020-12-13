from operator import add, mul

route = list(map(lambda x: (x[0], int(x[1:].strip())), open("input.txt")))

dirs = {"N": (0, 1), "E": (1, 0), "S": (0, -1), "W": (-1, 0)}

## Part 1

pos = 0, 0
heading = 1

for r, d in route:
    if r == "R":
        heading = (heading + d // 90) % len(dirs)
    elif r == "L":
        heading = (heading - d // 90) % len(dirs)
    elif r == "F":
        dir = list(dirs.values())[heading]
        pos = pos[0] + dir[0] * d, pos[1] + dir[1] * d
    else:
        pos = pos[0] + dirs[r][0] * d, pos[1] + dirs[r][1] * d

print(sum(map(abs, pos)))

## Part 2 ##
pos = 0, 0
waypoint = 10, 1

import math


# Quick maths - 2D rotation around origin (0,0 or the ship)
def rotate(point, angle):
    px, py = point

    qx = math.cos(angle) * px - math.sin(angle) * py
    qy = math.sin(angle) * px + math.cos(angle) * py
    return round(qx), round(qy)


def move_ship_and_waypoint(pos, waypoint, a, v):
    if a == "N":
        waypoint = waypoint[0], waypoint[1] + v
    elif a == "S":
        waypoint = waypoint[0], waypoint[1] - v
    elif a == "E":
        waypoint = waypoint[0] + v, waypoint[1]
    elif a == "W":
        waypoint = waypoint[0] - v, waypoint[1]
    elif a == "L":
        waypoint = rotate(waypoint, math.radians(v))
    elif a == "R":
        waypoint = rotate(waypoint, -math.radians(v))
    elif a == "F":
        pos = pos[0] + waypoint[0] * v, pos[1] + waypoint[1] * v

    return pos, waypoint

for a, v in route:
    pos, waypoint = move_ship_and_waypoint(pos, waypoint, a, v)
    
print(sum(map(abs, pos)))