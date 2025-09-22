import re
from copy import deepcopy
from dataclasses import dataclass, field
from itertools import chain, combinations
from queue import PriorityQueue
from typing import Literal, NamedTuple


class Component(NamedTuple):
    element: str
    kind: Literal["M", "G"]


def parse_floor(line: str) -> list[Component]:
    matches = re.findall(r"(?<=a\s)[^\s]+", line)

    items = []
    for match in matches:
        if "compatible" in match:
            items.append(Component(match.split("-")[0], "M"))
        else:
            items.append(Component(match, "G"))

    return items


def is_stable(parts: list[int], components: list[Component]) -> bool:
    as_components = [components[p] for p in parts]
    generators = [c.element for c in as_components if c.kind == "G"]
    if not generators:
        return True

    return all(c.element in generators for c in as_components if c.kind == "M")


@dataclass
class Diagram:
    floors: list[list[int]]
    elevator: int

    def try_take(
        self,
        direction: Literal["Up", "Down"],
        to_take: tuple[int],
        components: list[Component],
    ):
        target_floor = self.elevator - 1 if direction == "Down" else self.elevator + 1

        if not 0 <= target_floor <= 3:
            return None

        new_diagram = deepcopy(self)
        new_diagram.floors[target_floor] += to_take
        new_diagram.floors[self.elevator] = [
            el for el in new_diagram.floors[self.elevator] if el not in to_take
        ]

        if not (
            is_stable(new_diagram.floors[target_floor], components)
            and is_stable(new_diagram.floors[self.elevator], components)
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
    components: list[Component] = []
    floors = []

    input_ = "test.txt" if part == 0 else "input.txt"

    for floor_n, line in enumerate(open(input_).read().splitlines()):
        floor = parse_floor(line)
        if floor_n == 0 and part == 2:
            floor += [
                Component("elerium", "G"),
                Component("elerium", "M"),
                Component("dilithium", "G"),
                Component("dilithium", "M"),
            ]
        floors.append([i for i in range(len(components), len(components) + len(floor))])
        components += floor

    return (Diagram(floors=floors, elevator=0), components)


def find_pair(state: Diagram, components: list[Component], part: Component):
    for y, floor in enumerate(state.floors):
        for p in floor:
            if (
                components[p].kind != part.kind
                and components[p].element == part.element
            ):
                return y


def simplify_state(state: Diagram, components: list[Component]):
    """
    The simplified state of a diagram is a tuple containing the elevator position followed by
    a sorted list of tuples representing the positions of each element's generator and chip.
    If an element is missing a pair, use -1 for the missing part.
    """
    if state is None:
        return None

    # Map element -> (generator_floor, microchip_floor)
    element_floors = {}
    for y, floor in enumerate(state.floors):
        for c in floor:
            part = components[c]
            if part.element not in element_floors:
                element_floors[part.element] = [-1, -1]
            idx = 0 if part.kind == "G" else 1
            element_floors[part.element][idx] = y

    # Sort by element name to ensure canonical form
    pairs = [tuple(floors) for _, floors in sorted(element_floors.items())]
    return (state.elevator, *sorted(pairs))


@dataclass(order=True)
class PrioritizedItem:
    priority: int
    item: Diagram = field(compare=False)


def bfs(initial: Diagram, components: list[Component]) -> int | None:
    queue = PriorityQueue[PrioritizedItem]()
    queue.put(PrioritizedItem(0, initial))
    seen = set()
    seen.add(simplify_state(initial, components))

    while not queue.empty():
        item = queue.get()
        steps = item.priority
        state = item.item
        floor = state.floors[state.elevator]

        if len(state.floors[3]) == len(components):
            return steps

        # Try take two
        for direction in ("Up", "Down"):
            chain_ = (
                chain(combinations(floor, 1), combinations(floor, 2))
                if direction == "Up"
                else chain(combinations(floor, 2), combinations(floor, 1))
            )

            for take in chain_:
                if (next_state := state.try_take(direction, take, components)) and (
                    simple_state := simplify_state(next_state, components)
                ) not in seen:
                    seen.add(simple_state)
                    queue.put(PrioritizedItem(steps + 1, next_state))

    return None


# P1 is broken with the BFS implementation.
print("Part 1:", bfs(*parse_input(1)))
print("Part 2:", bfs(*parse_input(2)))
