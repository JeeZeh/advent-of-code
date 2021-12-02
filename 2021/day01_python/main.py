print(sum(a < b for a, b in zip([""] + list(open("i")), open("i"))))
print(sum(a[0] < a[-1] for a in zip([""] * 3 + list(open("i")), list(open("i")))))
