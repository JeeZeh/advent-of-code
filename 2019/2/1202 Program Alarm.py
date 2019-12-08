data = list(map(int, open("input.txt").read().split(",")))

data[1] = 12
data[2] = 2

for x in range(100):
    for y in range(100):
        mem = data.copy()
        i = 0
        mem[1] = x
        mem[2] = y
        while mem[i] != 99:
            if mem[i] == 1:
                mem[mem[i + 3]] = mem[mem[i + 1]] + mem[mem[i + 2]]
            if mem[i] == 2:
                mem[mem[i + 3]] = mem[mem[i + 1]] * mem[mem[i + 2]]

            i += 4

        if mem[0] == 19690720:
            print(100 * mem[1] + mem[2])

