a = [
    sum([-1 if b == ")" else 1 for b in open("i.txt").read()][:x])
    for x in range(len(open("i.txt").readline()) + 1)
]
print(a[-1])
print(a.index(-1))
