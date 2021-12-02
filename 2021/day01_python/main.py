d = list(open("i"))

print(sum(a < b for a, b in zip([""] + d, d)))
print(sum(a[0] < a[-1] for a in zip([""] * 3 + d, d)))
