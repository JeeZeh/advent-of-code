import pprint 

def count_k_v(d):
    keys = 0
    values = 0
    if type(d) == dict:
        for item in d.keys():
            if isinstance(d[item], (list, tuple, dict)):
                keys += 1
                k, v = count_k_v(d[item])
                values += v
                keys += k
            else:
                keys += 1
                values += 1

    elif type(d) == list or type(d) == tuple:
        for item in d:
            if isinstance(item, (list, tuple, dict)):
                k, v = count_k_v(item)
                values += v
                keys += k
            else:
                values += 1

    return keys, values

orbits = []
with open('input.txt') as o:
    for line in o:
        line = line.rstrip()
        orbits.append((line.split(')')[0], line.split(')')[1]))


t = {}
for o in orbits:
    if o[1] in t:
        t[o[1]] = t[o[1]].update({o[0]: {}})
    else:
        t[o[1]] = {o[0]: {}}

print(count_k_v({1: {2: {3: {}}}}))


tot = 0
for k in t:
    c = count_k_v(t[k])[0]
    while c > 0:
        tot += c
        c-= 1

print(tot + len(t))