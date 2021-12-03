d = list(open("i"))

print(sum(a < b for a, b in zip([""] + d, d)))
print(sum(a < b for a, b in zip([""] * 3 + d, d)))
