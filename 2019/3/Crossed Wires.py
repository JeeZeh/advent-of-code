dirs = {"U": ["y", 1, "x", "i"],"D": ["y", -1, "x", "i"],"L": ["x", -1, "i", "y"],"R": ["x", 1, "i", "y"]}
wires = [line.split(",") for line in open("input.txt")]
paths = [[None] * 20000 for _ in range(20000)]
collisions = []

for x in [0, 1]:
    pos = {"x": 0, "y": 0, "i": 0}
    steps = 0
    for trace in wires[x]:
        d, l = dirs[trace[0]], int(trace[1:])
        for i in range(pos[d[0]] + d[1], pos[d[0]] + d[1] * (l + 1), d[1]):
            steps +=1
            pos["i"] = i
            if x == 0:
                paths[pos[d[2]]][pos[d[3]]] = steps
            elif paths[pos[d[2]]][pos[d[3]]]:
                collisions.append((pos[d[2]], pos[d[3]], steps + paths[pos[d[2]]][pos[d[3]]]))
        pos[d[0]] = pos[d[0]] + d[1] * l
   
dists = [(abs(p[0]) + abs(p[1]), p[0], p[1], p[2]) for p in collisions][1:]
dists.sort(key=lambda tuple: tuple[0])
print(f"Distance: {dists[0][0]}")
dists.sort(key=lambda tuple: tuple[3])
print(f"Shortest Length: {dists[0][3]}")