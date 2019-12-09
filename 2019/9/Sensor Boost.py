p = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1}
o = list(map(int, open("input.txt").readline().split(",")))
M = max(o)
f = 0
s = False
for i in range(M):
    o.append(0)

while o[f] != 99:
    op = f"{o[f]:05}"
    c = int(op[-2:])
    e = p[c]
    m = list(map(int, list(op[:-2][::-1][:e])))
    j = None
    d = []
    r = 0

    for i in range(1, len(m) + 1):
        d.append(o[f + i])
    if c == 3:
        o[o[f + 1]] = 1
    elif c == 4:
        print(o[o[f + 1]])
    else:
        v = []
        for i in range(e):
            if m[i] == 0:
                v.append(o[d[i]])
            elif m[i] == 1:
                v.append(d[i])
            elif m[i] == 2:
                v.append(o[r])
        if c in [1, 2]:
            o[d[2]] = v[0] + v[1] if c == 1 else v[0] * v[1]
        elif (c == 5 and v[0] != 0) or (c == 6 and v[0] == 0):
            f = v[1]
            j = 1
        elif c == 7:
            o[d[2]] = 1 if v[0] < v[1] else 0
        elif c == 8:
            o[d[2]] = 1 if v[0] == v[1] else 0
        elif c == 9:
            r += v[0]
    if not j:
        f += len(m) + 1

