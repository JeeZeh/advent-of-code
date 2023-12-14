#!/usr/bin/env python3

import pathlib
import sys

from collections import defaultdict
from itertools import combinations

sys.path.append(str(pathlib.Path(__file__).resolve().parents[3] / "lib" / "python"))


class Reflection:
    def __init__(self, pattern: str) -> None:
        self.rows = [tuple(c for c in line.strip()) for line in pattern.splitlines()]
        self.cols = [
            tuple(line[x] for line in self.rows) for x in range(len(self.rows[0]))
        ]

    def line_of_reflection(self, smudge: bool = False) -> tuple[int, bool]:
        for view, is_horizontal in ((self.rows, True), (self.cols, False)):
            matching = defaultdict(set)
            off_by_one = defaultdict(set)

            for i, j in combinations(range(len(view)), 2):
                if view[i] == view[j]:
                    matching[i].add(j)
                elif sum(1 for a, b in zip(view[i], view[j]) if a != b) == 1:
                    off_by_one[i].add(j)

            for sep in range(1, len(view)):
                smudged = False
                for lo, hi in zip(reversed(range(0, sep)), range(sep, len(view))):
                    if hi not in matching[lo]:
                        if not smudge or smudged or hi not in off_by_one[lo]:
                            break
                        smudged = True
                else:
                    if not smudge or smudged:
                        return sep, is_horizontal

        assert False


def score(line: int, is_horizontal: bool) -> int:
    return line * (100 if is_horizontal else 1)


def run() -> None:
    with open("input.txt") as f:
        input = f.read()

    reflections = [Reflection(pattern) for pattern in input.split("\n\n")]

    note_sum = 0
    smudge_sum = 0

    for i, r in enumerate(reflections):
        line, is_horizontal = r.line_of_reflection()
        print(i, score(line, is_horizontal))
        note_sum += score(line, is_horizontal)

        line, is_horizontal = r.line_of_reflection(True)
        smudge_sum += score(line, is_horizontal)

    print(f"Sum of note values: {note_sum}")
    # print(f"Sum of smudge values: {smudge_sum}")


if __name__ == "__main__":
    run()
    sys.exit(0)
