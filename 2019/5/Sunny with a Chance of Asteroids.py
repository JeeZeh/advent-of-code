p = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3}
o = list(map(int, open("input.txt").readline().split(",")))
f = 0

while o[f] != 99:
    op = f"{o[f]:05}"
    c = int(op[-2:])
    e = p[c]
    m = list(map(int, list(op[:-2][::-1][:e])))
    j = None
    d = []

    for i in range(1, len(m) + 1):
        d.append(o[f + i])

    if c == 3:
        o[o[f + 1]] = 5
    elif c == 4:
        print(o[o[f + 1]])
    else:
        v = [d[i] if m[i] == 1 else o[d[i]] for i in range(e)]

        if c in [1, 2]:
            o[d[2]] = v[0] + v[1] if c == 1 else v[0] * v[1]
        elif (c == 5 and v[0] != 0) or (c == 6 and v[0] == 0):
            f = v[1]
            j = 1
        elif c == 7:
            o[d[2]] = 1 if v[0] < v[1] else 0
        elif c == 8:
            o[d[2]] = 1 if v[0] == v[1] else 0

    if not j:
        f += len(m) + 1
