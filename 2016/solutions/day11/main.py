from queue import PriorityQueue
import re
from collections import deque
from copy import deepcopy
from dataclasses import dataclass
from itertools import chain, combinations
from typing import Literal, NamedTuple, Optional


class Component(NamedTuple):
    element: str
    kind: Literal["M", "G"]


components = []


def parse_floor(line: str) -> list[Component]:
    matches = re.findall(r"(?<=a\s)[^\s]+", line)

    items = []
    for match in matches:
        if "compatible" in match:
            items.append(Component(match.split("-")[0], "M"))
        else:
            items.append(Component(match, "G"))

    return items


def is_stable(parts: list[int]):
    as_components = [components[p] for p in parts]
    generators = [c.kind for c in as_components if c.kind == "G"]
    if not generators:
        return True

    return all(c for c in as_components if c.kind == "M" and c.kind in generators)


@dataclass
class Diagram:
    floors: list[list[int]]
    elevator: int

    def __lt__(self, other):
        self.steps

    def try_take(self, direction: Literal["Up", "Down"], to_take: tuple[int]):
        target_floor = self.elevator - 1 if direction == "Down" else self.elevator + 1

        if not 0 <= target_floor <= 3:
            return None

        new_diagram = deepcopy(self)
        new_diagram.floors[target_floor] += to_take
        new_diagram.floors[self.elevator] = [
            el for el in new_diagram.floors[self.elevator] if el not in to_take
        ]

        if not all(
            map(
                is_stable,
                (new_diagram.floors[target_floor], new_diagram.floors[self.elevator]),
            )
        ):
            return None

        new_diagram.elevator = target_floor
        return new_diagram

    def __hash__(self) -> int:
        return ",".join(
            map(
                str,
                ("|".join(".".join(map(str, self.floors))), self.elevator),
            )
        ).__hash__()


def parse_input(test=False) -> tuple[Diagram, list[Component]]:
    components = []
    floors = []

    for line in open("test.txt" if test else "input.txt").read().splitlines():
        floor = parse_floor(line)
        floors.append([i for i in range(len(components), len(components) + len(floor))])
        components += floor

    return (Diagram(floors=floors, elevator=0), components)


initial, components = parse_input(True)


def bfs(initial: Diagram):
    queue = PriorityQueue()
    seen = set((0, initial))

    while queue.not_empty:
        steps, state = queue.get()
        floor = state.floors[state.elevator]

        if len(state.floors[3]) == len(components):
            return steps

        # Try take two
        for take in chain(combinations(floor, 2), combinations(floor, 1)):
            for direction in ("Up", "Down"):
                if (
                    next_state := state.try_take(direction, take)
                ) and next_state not in seen:
                    seen.add((steps + 1, next_state))
                    queue.append((steps + 1, next_state))

    return seen


print(bfs(initial))
