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


def get_score(*args: List[Tuple[int, Properties]]):
    capacity = []
    durability = []
    flavour = []
    texture = []

    for amount, ingredient in args:
        capacity.append(amount * ingredient.capacity)
        durability.append(amount * ingredient.durability)
        flavour.append(amount * ingredient.flavour)
        texture.append(amount * ingredient.texture)
        
    print(capacity, durability, flavour, texture)

    return prod(
        map(lambda x: max(0, x), map(sum, (capacity, durability, flavour, texture))),
    )



butterscotch = Properties(-1, -2, 6, 3, 8)
cinnamon = Properties(2, 3, -2, -1, 3)


print(get_score((44, butterscotch)))

# for i in range(100, -1, -1):
#     print(get_score((i, butterscotch)))