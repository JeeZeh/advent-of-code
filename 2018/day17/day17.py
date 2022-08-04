from collections import deque
from enum import Enum
from time import time
from tqdm import tqdm
from typing import Any, Deque, Iterable, NamedTuple, Optional


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

    def add(self, x, y):
        return Point(self.x + x, self.y + y)

    def down(self):
        return self.add(0, 1)

    def left(self):
        return self.add(-1, 0)

    def right(self):
        return self.add(1, 0)


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


class World:
    y_min: int
    y_max: int
    x_min: int
    x_max: int

    wall: set[Point] = set()
    flowing: set[Point] = set()
    still: set[Point] = set()
    fronts: Deque[Point] = deque((Point(500, 0),))

    def __init__(self, world_file: str = "input") -> None:
        for line in open(world_file):
            x, y = (p.split("=")[1] for p in sorted(line.split(", ")))

            if ".." in x:
                start, end = map(int, x.split(".."))
                for x_ in range(start, end + 1):
                    self.wall.add(Point(x_, int(y)))
            else:
                start, end = map(int, y.split(".."))
                for y_ in range(start, end + 1):
                    self.wall.add(Point(int(x), y_))

        self._calc_bounds()

    def _calc_bounds(self):
        points = list(self.wall)
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

    def can_flow_over(self, pos: Point):
        return pos in self.still or pos in self.wall

    def try_settle(self, pos: Point):
        # If water is flowing below, skip this tile
        checked = set((pos,))

        check = Point(*pos)
        while self.can_flow_over(check.down()) and (check := check.left()) not in self.wall:
            checked.add(check)
        left_wall = check in self.wall

        check = Point(*pos)
        while self.can_flow_over(check.down()) and (check := check.right()) not in self.wall:
            checked.add(check)
        right_wall = check in self.wall

        # Settling
        if left_wall and right_wall:
            self.still |= checked
            self.flowing.difference_update(checked)
            return True

        # Flowing outwards
        to_flow = checked.difference(self.flowing)
        if to_flow:
            self.flowing |= to_flow
            self.fronts.extend(to_flow)
            return True

        return False

    def can_fall(self, pos: Point):
        return pos not in self.flowing and pos not in self.still and pos not in self.wall and pos.y <= self.y_max

    def try_fall(self, pos: Point):
        check = Point(*pos).down()
        if not self.can_fall(check):
            return False
        else:
            self.fronts.append(pos)
            self.fronts.append(check)
            self.flowing.add(check)

        check = check.down()
        while self.can_fall(check):
            self.fronts.append(check)
            self.flowing.add(check)
            check = check.down()

        return True

    def simulate_water(self):
        if not self.fronts:
            return

        next_water = self.fronts.pop()
        if next_water in self.still:
            return True

        return self.try_fall(next_water) or self.try_settle(next_water)

    def print(self, focus=False, lowest=True):
        focus_y = None
        if focus:
            if lowest:
                focus_y = max(self.flowing, key=lambda p: p.y).y
            else:
                focus_y = (next_ := self.fronts.pop()).y
                self.fronts.append(next_)
        range_ = range(focus_y - 30, focus_y + 10) if focus else range(self.y_min, self.y_max + 1)
        rows = [""]
        for y in range_:
            row = []
            for x in range(self.x_min, self.x_max + 1):
                p = Point(x, y)
                tile = (
                    Tile.WALL
                    if p in self.wall
                    else Tile.WATER_M
                    if p in self.flowing
                    else Tile.WATER_S
                    if p in self.still
                    else Tile.EMPTY
                )
                row.append(tile)
            rows.append("".join(map(str, row)))
        print("\n".join(rows))


world = World("input")
while (changed := world.simulate_water()) is not None:
    # if changed:
    #     world.print(focus=True, lowest=False)
    pass

print("Part 1:", sum(1 for p in world.flowing | world.still if world.y_min <= p.y <= world.y_max))
print("Part 2:", sum(1 for p in world.still if world.y_min <= p.y <= world.y_max))
