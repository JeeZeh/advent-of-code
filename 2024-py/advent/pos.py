from dataclasses import dataclass
from enum import Enum
from typing import Self


@dataclass
class Pos:
    x: int
    y: int

    def add(self, other: Self):
        return Pos(self.x + other.x, self.y + other.y)

    def __hash__(self) -> int:
        return (self.x, self.y).__hash__()


class Direction(Enum):
    UP_LEFT = Pos(-1, -1)
    UP = Pos(0, -1)
    UP_RIGHT = Pos(1, -1)
    RIGHT = Pos(1, 0)
    DOWN_RIGHT = Pos(1, 1)
    DOWN = Pos(0, 1)
    DOWN_LEFT = Pos(-1, 1)
    LEFT = Pos(-1, 0)
