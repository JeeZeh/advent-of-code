from operator import add, mul

route = list(map(lambda x: (x[0], int(x[1:].strip())), open("input.txt")))

dirs = {"N": (0, 1), "E": (1, 0), "S": (0, -1), "W": (-1, 0)}

print(route)


pos = 0, 0
heading = 1

for r, d in route:
    if r == "R":
        heading = (heading + d//90) % len(dirs)
    elif r == "L":
        heading = (heading - d//90) % len(dirs)
    elif r == "F":
        dir = list(dirs.values())[heading]
        pos = pos[0] + dir[0] * d, pos[1] + dir[1] * d
    else:
        pos = pos[0] + dirs[r][0] * d, pos[1] + dirs[r][1] * d

    print(pos)
    
print(sum(map(abs, pos)))