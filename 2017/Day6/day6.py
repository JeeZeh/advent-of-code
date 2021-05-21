def get_max(banks) -> tuple[int, int]:
    chosen = 0, banks[0]
    for i, x in enumerate(banks):
        if x > chosen[1]:
            chosen = i, x

    return chosen


def distribute(banks, chosen):
    start, bank = chosen
    banks[start] = 0

    while bank:
        start = (start + 1) % len(banks)
        banks[start] += 1
        bank -= 1

    return banks


def balance(banks):
    states = {",".join(map(str, banks)): 0}

    while True:

        chosen = get_max(banks)
        banks = distribute(banks, chosen)

        new_state = ",".join(map(str, banks))
        if new_state in states:
            return banks, len(states), len(states) - states[new_state]

        states[new_state] = len(states)


banks = list(map(int, open("2017/Day6/input").read().split("	")))
print(balance(banks))
