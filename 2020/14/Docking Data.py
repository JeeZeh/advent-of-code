from itertools import product

inputs = list(map(lambda x: x.split(" = "), map(str.strip, open("input.txt"))))
mem, mask = {}, ""

### Part 1 ###
def apply_mask(num):
    bin_string = bin(num)[2:]
    bin_string = ("0" * (36 - len(bin_string))) + bin_string
    return "".join(mask[i] if mask[i] != "X" else s for i, s in enumerate(bin_string))


for line in inputs:
    if line[0] == "mask":
        mask = line[1]
    else:
        addr = line[0].split("[")[1][:-1]
        masked = apply_mask(int(line[1]))
        mem[addr] = masked

print(sum(int(f"0b{val}", 2) for val in mem.values()))

### Part 2 ###
def gen_addresses(num):
    bin_string = bin(num)[2:]
    bin_string = ("0" * (36 - len(bin_string))) + bin_string
    addr = "".join(mask[i] if mask[i] != "0" else s for i, s in enumerate(bin_string))
    floating = map(list, (product(("0", "1"), repeat=addr.count("X"))))

    return ("".join(d if d != "X" else f.pop(0) for d in addr) for f in floating)


mem, mask = {}, ""

for line in inputs:
    if line[0] == "mask":
        mask = line[1]
    else:
        addr = line[0].split("[")[1][:-1]
        num = int(line[1])
        mem.update({a: num for a in gen_addresses(int(addr))})

print(sum(mem.values()))
