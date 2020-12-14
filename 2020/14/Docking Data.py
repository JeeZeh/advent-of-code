mem = {}
mask = "0100X011110X11111100000010X1XXX1X100"

def apply_mask(num):
    bin_string = bin(num)[2:]
    bin_string = ("0" * (36 - len(bin_string))) + bin_string
    return "".join(mask[i] if mask[i] != "X" else s for i, s in enumerate(bin_string))
    
for line in map(lambda x: x.split(" = "), map(str.strip, open("input.txt"))):
    if line[0] == "mask":
        mask = line[1]
    else:
        addr = line[0].split("[")[1][:-1]
        masked = apply_mask(int(line[1]))
        mem[addr] = masked
        
print(sum(int(f"0b{val}", 2) for val in mem.values()))