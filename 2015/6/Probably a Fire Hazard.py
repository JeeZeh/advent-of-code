grid = [[0 for i in range(1000)] for _ in range(1000)]

def turn_on(start, end):
    for y in range(start[1], end[1] + 1):
        for x in range(start[0], end[0] + 1):
            grid[y][x] += 1
            
def turn_off(start, end):
    for y in range(start[1], end[1] + 1):
        for x in range(start[0], end[0] + 1):
            grid[y][x] = max(0, grid[y][x] - 1)
            
def toggle(start, end):
    for y in range(start[1], end[1] + 1):
        for x in range(start[0], end[0] + 1):
            grid[y][x] += 2
            
instructions = open("input.txt").readlines()

def get_start_end(ins):
    print(ins)
    return tuple(map(int, ins[0].split(","))), tuple(map(int, ins[2].split(",")))


for ins in instructions:
    if ins.startswith("toggle"):
        ins = ins[6:].split()
        toggle(*get_start_end(ins))
    elif ins.startswith("turn on"):
        ins = ins[7:].split()
        turn_on(*get_start_end(ins))
    else:
        ins = ins[8:].split()
        turn_off(*get_start_end(ins))
        
# turn_on((499,499), (500,500))
print(sum(sum(row) for row in grid))