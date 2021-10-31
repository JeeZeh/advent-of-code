from typing import Dict, List, Literal, Tuple, Union

Opcode = Union[
    Literal["hlf"],
    Literal["tpl"],
    Literal["inc"],
    Literal["jmp"],
    Literal["jie"],
    Literal["jio"],
]

Instruction = Tuple[Opcode, List[Union[str, int]]]
Registers = Dict[str, int]


def try_parse(arg):
    try:
        return int(arg)
    except:
        return arg


def read_instructions() -> List[Instruction]:
    instructions: List[Instruction] = []
    with open("input.txt") as infile:
        for line in infile.read().splitlines():
            first_space = line.index(" ")
            instr, rest = line[:first_space], line[first_space + 1 :]
            args = list(map(try_parse, rest.split(", ")))
            instructions.append((instr, args))

    return instructions


def execute_instruction(ptr: int, registers: Registers, instr: Instruction) -> int:
    op, args = instr

    if op == "hlf":
        registers[args[0]] //= 2
        ptr += 1
    elif op == "tpl":
        registers[args[0]] *= 3
        ptr += 1
    elif op == "inc":
        registers[args[0]] += 1
        ptr += 1
    elif op == "jmp":
        ptr += args[0]
    elif op == "jie":
        ptr += args[1] if registers[args[0]] % 2 == 0 else 1
    elif op == "jio":
        ptr += args[1] if registers[args[0]] == 1 else 1

    return ptr


def run(registers: Registers, instructions: List[Tuple[Opcode, List]]):
    ptr = 0

    while ptr < len(instructions):
        ptr = execute_instruction(ptr, registers, instructions[ptr])

    return registers


part_one = run(
    {
        "a": 0,
        "b": 0,
    },
    read_instructions(),
)
print("Part 1:", part_one["b"])

part_two = run(
    {
        "a": 1,
        "b": 0,
    },
    read_instructions(),
)
print("Part 1:", part_two["b"])
