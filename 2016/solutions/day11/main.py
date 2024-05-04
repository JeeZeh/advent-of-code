from queue import PriorityQueue
import re
from collections import deque
from copy import deepcopy
from dataclasses import dataclass, field
from itertools import chain, combinations
from typing import Any, Literal, NamedTuple, Optional


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
    generators = [c.element for c in as_components if c.kind == "G"]
    if not generators:
        return True

    return all(c.element in generators for c in as_components if c.kind == "M")


@dataclass
class Diagram:
    floors: list[list[int]]
    elevator: int

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


def parse_input(part) -> tuple[Diagram, list[Component]]:
    components = []
    floors = []

    input_ = "test.txt" if part == 0 else f"input{part}.txt"  

    for line in open(input_).read().splitlines():
        floor = parse_floor(line)
        floors.append([i for i in range(len(components), len(components) + len(floor))])
        components += floor

    return (Diagram(floors=floors, elevator=0), components)

def find_pair(state: Diagram, part: Component):
    for y, floor in enumerate(state.floors):
        for p in floor:
            if components[p].kind != part.kind and components[p].element == part.element:
                return y


def simplify_state(state: Diagram):
    """
    The simplified state of a diagram is a tuple containing the elevator position followed by
    a sorted list of tuples representing the pairs of generators and chips. Elements are not
    encoded since two pairs of elements in two positions are interchangeable.
    """
    
    if state is None:
        return None
    
    pairs = []
    elements = set()
    for y, floor in enumerate(state.floors):
        for c in floor:
            part = components[c]
            if part.element in elements:
                continue
            
            other = find_pair(state, part)
            pairs.append((tuple(sorted((y, other)))))
    
    pairs.sort()
    return (state.elevator, *pairs)

initial, components = parse_input(2)

@dataclass(order=True)
class PrioritizedItem:
    priority: int
    item: Diagram=field(compare=False)

def bfs(initial: Diagram):
    queue = PriorityQueue()
    queue.put(PrioritizedItem(0, initial))
    seen = set()
    seen.add(simplify_state(initial))

    while not queue.empty():
        item = queue.get()
        steps = item.priority
        state = item.item
        floor = state.floors[state.elevator]

        if len(state.floors[3]) == len(components):
            return steps

        # Try take two
        for direction in ("Up", "Down"):
            chain_ = chain(combinations(floor, 1), combinations(floor, 2)) if direction == "Up" else chain(combinations(floor, 2), combinations(floor, 1))
            
            for take in chain_:
                if (
                    next_state := state.try_take(direction, take)
                ) and (simple_state := simplify_state(next_state)) not in seen:
                    seen.add(simple_state)
                    queue.put(PrioritizedItem(steps + 1, next_state))

    return seen

print("Part 2:", bfs(initial))