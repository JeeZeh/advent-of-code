from dataclasses import dataclass
from math import prod
from typing import List, Tuple


@dataclass
class Properties:
    capacity: int
    durability: int
    flavour: int
    texture: int
    calories: int


def parse(line: str) -> Properties:
    parts = line.replace(",", "").split()

    return Properties(
        int(parts[2]),
        int(parts[4]),
        int(parts[6]),
        int(parts[8]),
        int(parts[10]),
    )


def get_score(*args: List[Tuple[int, Properties]]) -> Tuple[int, int]:
    capacity = []
    durability = []
    flavour = []
    texture = []
    calories = []

    for amount, ingredient in args:
        capacity.append(amount * ingredient.capacity)
        durability.append(amount * ingredient.durability)
        flavour.append(amount * ingredient.flavour)
        texture.append(amount * ingredient.texture)
        calories.append(amount * ingredient.calories)

    return (
        prod(
            map(
                lambda x: max(0, x), map(sum, (capacity, durability, flavour, texture))
            ),
        ),
        sum(calories),
    )


def constrained_values(n):
    """
    This is horrible. I know how to constrain the max for each range, 
    but I've no idea how to constrain the start value.
    """
    for a in range(0, n + 1):
        for b in range(0, n - a + 1):
            for c in range(0, n - a - b + 1):
                for d in range(0, n - a - b - c + 1):
                    if a + b + c + d == n:
                        yield (a, b, c, d)


ingredients = list(map(parse, open("input.txt")))
variations = [get_score(*zip(v, ingredients)) for v in constrained_values(100)]

print("Part 1:", max(variations)[0])
print("Part 2:", max(filter(lambda v: v[1] == 500, variations))[0])
