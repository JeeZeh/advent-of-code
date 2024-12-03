from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Generator
from dataclasses import dataclass
from re import Match, compile
from typing import Any

from aocd import get_puzzle

from advent.solution import Solution

match_dont = compile(r"don't\(\)")


class Flag(ABC):
    @staticmethod
    @abstractmethod
    def find_all(text: str) -> Generator[tuple[Match[str], Flag], Any]:
        raise NotImplementedError


class Do(Flag):
    __pattern = compile(r"do\(\)")

    @staticmethod
    def find_all(text: str):
        for match in Do.__pattern.finditer(text):
            yield match, Do()


class Dont(Flag):
    __pattern = compile(r"don't\(\)")

    @staticmethod
    def find_all(text: str):
        for match in Dont.__pattern.finditer(text):
            yield match, Dont()


class Instruction(ABC):
    @abstractmethod
    def run(self) -> int:
        raise NotImplementedError

    @staticmethod
    def find_all(text: str) -> Generator[tuple[Match[str], Instruction], Any]:
        raise NotImplementedError


@dataclass
class Mult(Instruction):
    __pattern = compile(r"mul\((\d{1,3}),(\d{1,3})\)")
    a: int
    b: int

    @staticmethod
    def find_all(text: str):
        for match in Mult.__pattern.finditer(text):
            yield (match, Mult(*(int(group) for group in match.groups())))

    def run(self) -> int:
        return self.a * self.b


Instruction = Mult


class Day03(Solution):
    def parse_instructions_full(self, inst_text: str) -> list[Instruction | Flag]:
        instructions: list[tuple[Match[str], Instruction | Flag]] = []
        for instruction in (Mult, Do, Dont):
            if found := list(instruction.find_all(inst_text)):
                instructions += list(found)

        def sort_instructions(match: tuple[Match[str], Instruction]) -> int:
            return match[0].start()

        return [inst[1] for inst in sorted(instructions, key=sort_instructions)]  # type: ignore

    def run(self, puzzle_input: str):
        instructions = self.parse_instructions_full(puzzle_input)

        run_all = sum(
            inst.run() for inst in instructions if isinstance(inst, Instruction)
        )

        with_flags: list[Instruction] = []
        last: Flag = Do()
        for op in instructions:
            if isinstance(last, Do) and isinstance(op, Instruction):
                with_flags.append(op)
            if isinstance(op, Flag):
                last = op

        return run_all, sum(inst.run() for inst in with_flags)


if __name__ == "__main__":
    Day03().run(get_puzzle(day=3, year=2024).input_data)
