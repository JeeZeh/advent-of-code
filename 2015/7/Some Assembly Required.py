from collections import defaultdict

circuit = map(lambda x: x.split(" -> "), map(str.strip, open("input.txt")))
wires = defaultdict(int)

ops = {
    "AND": lambda x, y: wires[x] & wires[y] % (2 ** 16),
    "OR": lambda x, y: wires[x] | wires[y] % (2 ** 16),
    "LSHIFT": lambda x, y: wires[x] << int(y) % (2 ** 16),
    "RSHIFT": lambda x, y: wires[x] >> int(y) % (2 ** 16),
    "NOT": lambda x: ~wires[x] % 2 ** 16
}

def is_int(x):
    try:
        int(x)
        return True
    except:
        return False


for part in circuit:
    inputs = part[0].split()
    output = part[1]
    print(inputs, output)
    if len(inputs) == 1:
        wires[output] =  int(inputs[0]) if is_int(inputs[0]) else wires.get(inputs[0])
    elif len(inputs) == 2:
        wires[output] = ops[inputs[0]](inputs[1])
    else:
        wires[output] = ops[inputs[1]](inputs[0], inputs[2])

print(wires["a"])