from collections import defaultdict
from re import L

Instruction = tuple[str, str, int, str]

registers = defaultdict(int)
max_values = []

def parse_instruction(t: str) -> Instruction:
    # b inc 5 if a > 1
    ins, condition = t.split(" if ")

    register, operation, amount = ins.split()

    amount = int(amount)

    return register, operation, amount, condition


def check_condition(condition: str) -> bool:
    # a == 1
    reg, check, value = condition.split(" ")

    value = int(value)

    # return eval(f"{registers[reg]} {check} {value}")

    if check == "<":
        return registers[reg] < value
    if check == "<=":
        return registers[reg] <= value
    if check == ">":
        return registers[reg] > value
    if check == ">=":
        return registers[reg] >= value
    if check == "==":
        return registers[reg] == value
    if check == "!=":
        return registers[reg] != value


def execute(ins: Instruction):
    reg, op, amount, condition = ins

    if not check_condition(condition):
        return

    if op == "inc":
        registers[reg] += amount
    elif op == "dec":
        registers[reg] -= amount


def run():
    instructions = map(parse_instruction, open("Day8/input").read().splitlines())

    for ins in instructions:
        execute(ins)
        max_values.append(max(registers.values()))
        
def reset():
    global registers
    registers = defaultdict(int)
    
def part_one():
    run()
    print(max(registers.values()))
    reset()
    
def part_two():
    run()
    print(max(max_values))
    
part_one()
part_two()