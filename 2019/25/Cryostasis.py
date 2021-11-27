from intcode import Intcode, IntcodeStatus
from itertools import combinations
from tqdm import tqdm

program = list(map(int, open("input.txt").read().split(",")))

get_to_checkpoint_with_all_items = [
    "east",
    "take festive hat",
    "east",
    "take food ration",
    "east",
    "take spool of cat6",
    "west",
    "west",
    "west",
    "west",
    "take hologram",
    "north",
    "take space heater",
    "east",
    "take space law space brochure",
    "east",
    "take tambourine",
    "west",
    "west",
    "south",
    "east",
    "east",
    "south",
    "east",
    "east",
    "take fuel cell",
    "east",
    "inv",
]

items = [inst[5:] for inst in get_to_checkpoint_with_all_items if inst.startswith("take")]


def send_instructions(intcode: Intcode, instructions: list[str], live=True):
    status = intcode.run()
    last_output = ""

    if status == IntcodeStatus.WAITING:
        last_output = intcode.drain_output(True)
        if live:
            print(last_output)

    while instructions:
        next_inst = instructions.pop(0)
        if live:
            print(">", next_inst)
        intcode.send(next_inst + "\n")
        status = intcode.run()
        if status == IntcodeStatus.WAITING:
            last_output = intcode.drain_output(True)
            if live:
                print(last_output)

    return last_output


def all_combinations(items):
    combs = []
    for r in range(1, len(items) + 1):
        for c in combinations(items, r):
            combs.append(set(c))

    return combs


def pickup_door_items(combination, holding):
    instructions = [f"drop {item}" for item in holding if item not in combination]
    instructions += [f"take {c}" for c in combination if c not in holding]

    return instructions


def try_combination(droid, combination, holding=items) -> bool:
    output = send_instructions(droid, pickup_door_items(combination, holding) + ["south"], False)
    if "you are ejected back to the checkpoint" not in output:
        print(output)
        return True
    return False


def crack_door_combination():
    print("Cracking door...")

    droid = Intcode(program)
    send_instructions(droid, get_to_checkpoint_with_all_items[:], False)

    last_comb = items
    for c in tqdm(all_combinations(items)):
        if try_combination(droid, c, holding=last_comb):
            print("Found combination!", c)
            return c
        last_comb = c


crack = crack_door_combination()

droid = Intcode(program)
send_instructions(droid, get_to_checkpoint_with_all_items[:] + pickup_door_items(crack, items) + ["south"], False)
print(droid.drain_output(for_humans=True))
