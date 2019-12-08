W, H = 25, 6
S = W * H
data = open("input.txt").readline()

layers = [data[i : i + S] for i in range(0, len(data), S)]

least = min(layers, key=lambda l: l.count("0"))
print(least.count("1") * least.count("2"))

output = [
    list(filter(lambda x: x != "2", map(lambda l: l[i][0], layers)))[0]
    for i in range(S)
]

for y in range(H):
    print("".join(output[y * W : y * W + W]).replace("0", " "))

