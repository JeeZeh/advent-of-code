from dataclasses import dataclass
from typing import Self


@dataclass
class Pos:
    x: int
    y: int

    def add(self, other: Self):
        return Pos(self.x + other.x, self.y + other.y)
