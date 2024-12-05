from collections import defaultdict
from dataclasses import dataclass
from functools import cmp_to_key
from typing import Self

from aocd import get_puzzle

from advent.solution import Solution


class Update(list[int]):
    def sort_by(self, rules: dict[int, list[int]]) -> Self:
        def comparator(a: int, b: int):
            if a == b:
                return 0
            if b in rules[a]:
                return -1

            return 1

        return Update(sorted(self, key=cmp_to_key(comparator)))  # type: ignore


@dataclass
class State:
    updates: list[Update]
    rules: dict[int, list[int]]

    @staticmethod
    def from_string(text: str):
        a, b = text.split("\n\n")
        precede: dict[int, list[int]] = defaultdict(list)
        for line in a.splitlines():
            first, second = tuple(map(int, line.split("|")))
            precede[first].append(second)

        updates = [
            Update([int(c) for c in update.split(",")]) for update in b.splitlines()
        ]

        return State(updates, precede)


class Day05(Solution):
    def run(self, puzzle_input: str):
        state = State.from_string(puzzle_input)

        valid_updates: list[Update] = []
        fixed_updates: list[Update] = []
        for before in state.updates:
            if (after := before.sort_by(state.rules)) != before:
                fixed_updates.append(after)
            else:
                valid_updates.append(before)

        return (
            sum(u[len(u) // 2] for u in valid_updates),
            sum(u[len(u) // 2] for u in fixed_updates),
        )


if __name__ == "__main__":
    print(Day05().run(get_puzzle(day=5, year=2024).input_data))
