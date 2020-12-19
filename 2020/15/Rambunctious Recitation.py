start = "2,0,6,12,1,3"


def get_turn(n):
    mem = {int(v): i + 1 for i, v in enumerate(start.split(",")[:-1])}

    turn = len(mem)
    last = int(start[-1])
    next = None
    while turn + 1 != n:
        turn += 1
        if last not in mem:
            next = 0
        else:
            next = turn - mem[last]

        mem[last] = turn
        last = next
    print(last)


get_turn(2020)
get_turn(30000000)