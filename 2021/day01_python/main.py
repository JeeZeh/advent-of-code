print(sum(a < b for a, b in zip([""] + list(open("i")), open("i"))))
print(sum(sum(a[:3]) < sum(a[1:]) for a in zip(*tuple(list(map(int, ["999"] * i + list(open("i")))) for i in range(3, -1, -1)))))