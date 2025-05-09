from dataclasses import field
from enum import Enum
from pathlib import Path


class Opcode(Enum):
    Copy = "cpy"
    Incr = "inc"
    Decr = "dec"
    Jump = "jnz"


Operand = str
Inst = tuple[Opcode, tuple[Operand, Operand] | tuple[Operand]]
Tape = list[Inst]


class Program:
    ptr: int
    reg: list[int]
    tape: Tape
    size: int

    def __init__(self, tape: Tape, reg=[0, 0, 0, 0]) -> None:
        self.ptr = 0
        self.reg = list(reg)
        self.tape = tape
        self.size = len(tape)

    def run(self):
        while self.ptr < self.size:
            inst = self.tape[self.ptr]
            self.execute_instruction(inst)

    @staticmethod
    def compute_reg_idx(name):
        return ord(name[0]) - 97

    def resolve_operand(self, operand: str) -> int:
        if ord(operand[0]) >= 97:
            return self.reg[self.compute_reg_idx(operand)]

        return int(operand)

    def execute_instruction(self, inst: Inst):
        # Increment pointer by default
        jump_offset: int = 1

        opcode, operands = inst
        match opcode:
            case Opcode.Copy:
                match operands:
                    case x, y:
                        self.reg[self.compute_reg_idx(y)] = self.resolve_operand(x)
                    case _:
                        raise ValueError(
                            f"Expected 2 operands for '{inst}', found '{operands}'"
                        )
            case Opcode.Incr:
                x = operands[0]
                self.reg[self.compute_reg_idx(x)] += 1
            case Opcode.Decr:
                x = operands[0]
                self.reg[self.compute_reg_idx(x)] -= 1
            case Opcode.Jump:
                match operands:
                    case x, y:
                        if self.resolve_operand(x) != 0:
                            jump_offset = self.resolve_operand(y)
                    case _:
                        raise ValueError(
                            f"Expected 2 operands for '{inst}', found '{operands}'"
                        )
            case _:
                raise ValueError(f"Unexpected instruction '{inst}'")

        self.ptr += jump_offset


tape: Tape = []
for line in Path("inputs/real").read_text().splitlines():
    opcode, *operands = line.split(" ")
    match operands:
        case (x,):
            tape.append((Opcode(opcode), (x,)))
        case (x, y):
            tape.append((Opcode(opcode), (x, y)))


program = Program(tape)
program.run()
print(f"Part 1: {program.reg[0]}")

program = Program(tape, [0, 0, 1, 0])
program.run()
print(f"Part 2: {program.reg[0]}")
