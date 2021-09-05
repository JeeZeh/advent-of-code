from collections import defaultdict

ops = {
    "AND": lambda x, y: (x & y) % (2 ** 16),
    "OR": lambda x, y: (x | y) % (2 ** 16),
    "LSHIFT": lambda x, y: (x << y) % (2 ** 16),
    "RSHIFT": lambda x, y: (x >> y) % (2 ** 16),
    "NOT": lambda x: ~x % (2 ** 16),
}


def is_int(x):
    try:
        int(x)
        return True
    except:
        return False


def resolve_input_to_value(i: str):
    global gates

    if is_int(i):
        return int(i)

    if i in ops:
        return i

    input_ = gates[i]

    if isinstance(input_, int):
        return input_

    input_ = list(map(resolve_input_to_value, input_))

    if len(input_) == 1:
        gates[i] = input_[0]
    elif len(input_) == 2:
        gates[i] = ops[input_[0]](input_[1])
    else:
        gates[i] = ops[input_[1]](input_[0], input_[2])

    return gates[i]


circuit = list(map(lambda x: x.split(" -> "), map(str.strip, open("input.txt"))))
gates = defaultdict(int)
for part in circuit:
    inputs = part[0].split()
    output = part[1]
    gates[output] = inputs

resolve_input_to_value("a")
b_override = gates["a"]
print(b_override)

gates = defaultdict(int)
for part in circuit:
    inputs = part[0].split()
    output = part[1]
    gates[output] = inputs

gates["b"] = b_override
resolve_input_to_value("a")
print(gates["a"])
