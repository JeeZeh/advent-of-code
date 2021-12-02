depths = list(open("i"))

print(sum(a < b for a, b in zip([""] + depths, depths)))
print(sum(a[0] < a[-1] for a in zip([""] * 3 + depths, depths)))
