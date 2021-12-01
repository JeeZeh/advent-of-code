d = list(map(int, open("in")))

print(sum(d[i] < d[i + 1] for i in range(len(d) - 1)))
print(sum(sum(d[i : i + 3]) < sum(d[i + 1 : i + 4]) for i in range(len(d) - 3)))
