from collections import defaultdict
from dataclasses import dataclass

from aocd import get_puzzle

from advent.solution import Solution

Rule = tuple[int, int]
Update = list[int]


@dataclass
class State:
    rules: list[Rule]
    updates: list[Update]
    must_precede: dict[int, list[int]]
    must_follow: dict[int, list[int]]

    @staticmethod
    def from_string(text: str):
        a, b = text.split("\n\n")
        rules: list[tuple[int, int]] = [
            tuple(map(int, line.split("|"))) for line in a.splitlines()  # type: ignore
        ]
        updates = [
            [int(p) for p in update.strip().split(",")] for update in b.splitlines()
        ]

        precede: dict[int, list[int]] = defaultdict(list)
        follow: dict[int, list[int]] = defaultdict(list)
        for first, second in rules:
            precede[first].append(second)
            follow[second].append(first)

        return State(rules, updates, precede, follow)


def check_update(update: Update, state: State):
    for i, num in enumerate(update[:-1]):
        before, after = update[:i], update[i + 1 :]
        if any(b in state.must_precede[num] for b in before) or any(
            a in state.must_follow[num] for a in after
        ):
            return False

    return True


class Day05(Solution):

    def run(self, puzzle_input: str):
        state = State.from_string(puzzle_input)
        valid_updates = sum(
            update[len(update) // 2]
            for update in state.updates
            if check_update(update, state)
        )

        return valid_updates, None


if __name__ == "__main__":
    print(Day05().run(get_puzzle(day=5, year=2024).examples[0].input_data))
