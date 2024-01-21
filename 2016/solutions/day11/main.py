import re
from typing import NamedTuple


class Component(NamedTuple):
    kind: str
    is_chip: bool


def parse_floor(line: str) -> list[Component]:
    matches = re.findall(r"(?<=a\s)[^,|\.]+", line)

    items = []
    for match in matches:
        if "microchip" in match:
            items.append(Component(match.split("-")[0], True))
        else:
            items.append(Component(match.split(" ")[0], False))

    return items


lines = open("test.txt").read().splitlines()
floors = [parse_floor(floor) for floor in lines]
floors.reverse()

print(floors)
