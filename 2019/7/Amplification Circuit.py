import itertools

init = itertools.permutations([0, 1, 2, 3, 4])
feedback = itertools.permutations([5, 6, 7, 8, 9])

def comp(phase):
    p = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3}
    o = list(map(int, open("input.txt").readline().split(",")))
    f = 0
    s = False
    power = yield
   

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
            if not s:
                o[o[f + 1]] = phase
                s = True
            else:
                o[o[f + 1]] = power
        elif c == 4:
            power = yield o[o[f + 1]]
            
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


    return power
    

finals = []


for phases in init:
    output = 0
    amps = [comp(phase) for phase in phases]
    for amp in amps:
        amp.send(None)
        output = amp.send(output)
    
    finals.append((output, phases))

print("\n---- Direct Mode ----")
print(f"Power: {max(finals)[0]}")
print(f"Phases: {max(finals)[1]}")
finals = []

print("\n---- Feedback Loop Mode ----")
for phases in feedback:
    output = 0
    amps = [comp(phase) for phase in phases]
    for amp in amps:
        amp.send(None)
    
    halted = 0
    
    while halted < len(amps): 
        halted = 0
        for amp in amps:
            try: 
                output = amp.send(output)
            except:
                halted += 1

    finals.append((output, phases))
            
print(f"Power: {max(finals)[0]}")
print(f"Phases: {max(finals)[1]}")
print()
