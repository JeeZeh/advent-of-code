op = {"^": (0, 1), "v": (0, -1), "<": (-1, 0), ">": (1, 0)}
houses = [(0, 0)]
santas = [(0,0), (0,0)]

for i, d in enumerate(open("input.txt").readline()):
        santas[i % 2] = tuple(map(lambda x, y: x + y, santas[i % 2], op[d]))
        houses.append(santas[i % 2])
    

print(len(dict.fromkeys(houses)))
