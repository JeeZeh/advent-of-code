from collections import defaultdict
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Dict, Iterable, NamedTuple, Optional


class Tile(Enum):
    WATER_S = (0,)
    WATER_M = (1,)
    WALL = (2,)
    EMPTY = 3
    SPRING = 4

    def __str__(self) -> str:
        if self == self.WALL:
            return "#"
        if self == self.WATER_M:
            return "|"
        if self == self.WATER_S:
            return "~"
        if self == self.EMPTY:
            return "."
        if self == self.SPRING:
            return "+"


class Point(NamedTuple):
    x: int
    y: int


class CompType(Enum):
    MIN = -1
    MAX = 1


def tuple_comp(ts: Iterable[tuple[Any]], comp_type: CompType):
    vals = [el for el in ts[0]]

    for t in ts:
        for i, el in enumerate(t):
            if comp_type == CompType.MIN:
                vals[i] = el if el < vals[i] else vals[i]
            else:
                vals[i] = el if el > vals[i] else vals[i]


@dataclass
class World:
    grid: dict[Point, Tile]
    x_min: int
    x_max: int
    y_min: int
    y_max: int

    spring = Point(500, 0)

    def __init__(self, world_file: str = "input") -> None:
        self.grid = {}
        for line in open("input"):
            x, y = (p.split("=")[1] for p in sorted(line.split(", ")))

            if ".." in x:
                start, end = map(int, x.split(".."))
                for x_ in range(start, end + 1):
                    self.grid[Point(x_, int(y))] = Tile.WALL
            else:
                start, end = map(int, y.split(".."))
                for y_ in range(start, end + 1):
                    self.grid[Point(int(x), y_)] = Tile.WALL

        self.grid[self.spring] = Tile.SPRING
        self._calc_bounds()

    def __getitem__(self, p: Point) -> Optional[Tile]:
        if p.x < self.x_min or p.x > self.x_max or p.y < self.y_min or p.y > self.y_max:
            return None

        if p not in self.grid:
            self.grid[p] = Tile.EMPTY

        return self.grid[p]

    def _calc_bounds(self):
        points = list(self.grid.keys())
        self.x_min, self.y_min = points[0]
        self.x_max, self.y_max = points[0]

        for point in points:
            x, y = point
            if x < self.x_min:
                self.x_min = x
            if x > self.x_max:
                self.x_max = x
            if y < self.y_min:
                self.y_min = y
            if y > self.y_max:
                self.y_max = y

        # self.x_min -= 1
        # self.y_min -= 1
        # self.x_max += 1
        # self.y_max += 1
        
    def simulate_water(self):
        pass

    def print(self):
        rows = []
        for y in range(self.y_min, self.y_max + 1):
            row = []
            for x in range(self.x_min, self.x_max + 1):
                row.append(self[Point(x, y)])

            rows.append("".join(map(str, row)))

        print("\n".join(rows))


world = World()

world.print()
