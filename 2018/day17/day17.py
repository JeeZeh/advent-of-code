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

    def add(self, x, y):
        return Point(self.x + x, self.y + y)


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
        for line in open(world_file):
            x, y = (p.split("=")[1] for p in sorted(line.split(", ")))

            if ".." in x:
                start, end = map(int, x.split(".."))
                for x_ in range(start, end + 1):
                    self[Point(x_, int(y))] = Tile.WALL
            else:
                start, end = map(int, y.split(".."))
                for y_ in range(start, end + 1):
                    self[Point(int(x), y_)] = Tile.WALL

        # Add spring and initial flow
        first_water = self.spring.add(0, 1)
        self[self.spring] = Tile.SPRING
        self[first_water] = Tile.WATER_M

        self._calc_bounds()

    def __getitem__(self, p: Point) -> Optional[Tile]:
        if p.x < self.x_min or p.x > self.x_max or p.y < self.y_min or p.y > self.y_max:
            return None

        if p not in self.grid:
            return Tile.EMPTY

        return self.grid[p]

    def __setitem__(self, p: Point, t: Tile) -> None:
        self.grid[p] = t

    def try_set(self, p: Point, t: Tile) -> bool:
        if self.x_min <= p.x <= self.x_max and self.y_min <= p.y <= self.y_max:
            self.grid[p] = t
            return True

        return False

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

    def is_bounded_lateral(self, starting: Point, step: int) -> tuple[bool, list[Point], list[Point]]:
        empty = []
        flowing = []

        check = starting.add(step, 0)
        while (self[check] == Tile.EMPTY or self[check] == Tile.WATER_M) and self[check.add(0, 1)] in [
            Tile.WALL,
            Tile.WATER_S,
        ]:
            if self[check] == Tile.EMPTY:
                empty.append(check)
            else:
                flowing.append(check)

            check = check.add(step, 0)

        if self[check] == Tile.WALL:
            return True, empty, flowing
        elif self[check] == Tile.EMPTY:
            # We're floating so we need to add the water further out so it falls
            empty.append(check)

        return False, empty, flowing

    def try_bound_flow(self, pos: Point) -> bool:
        left_bounded, left_empty, left_flowing = self.is_bounded_lateral(pos, -1)
        right_bounded, right_empty, right_flowing = self.is_bounded_lateral(pos, 1)

        empty = left_empty + right_empty
        flowing = left_flowing + right_flowing

        if left_bounded and right_bounded:
            for tile_pos in [pos] + empty + flowing:
                self[tile_pos] = Tile.WATER_S

            return True
        # if not left_checked and not right_checked:
        #     return False

        for tile_pos in empty:
            self[tile_pos] = Tile.WATER_M

        return True

    def try_flow(self, pos: Point) -> bool:
        # If water is flowing below, skip this tile
        down = pos.add(0, 1)
        if self[down] == Tile.WATER_M:
            return False

        # Try flow down and move to next simulation step
        if self[down] == Tile.EMPTY:
            return self.try_set(down, Tile.WATER_M)
        if self[down] is None:
            return False

        if self[pos.add(-1, 0)] == Tile.WATER_M and self[pos.add(1, 0)] == Tile.WATER_M:
            return False

        # Down is blocked by still or clay, try right and left
        return self.try_bound_flow(pos)

    def simulate_water(self) -> bool:
        changed = False
        to_flow = []
        for k, v in self.grid.items():
            if v != Tile.WATER_M:
                continue
            
            down = self[k.add(0, 1)]
            if down == Tile.EMPTY:
                to_flow.append(k)
                continue
            elif down == Tile.WATER_M:
                continue
            
            left, right = self[k.add(-1, 0)], self[k.add(1, 0)]
            if left == Tile.EMPTY or right == Tile.EMPTY:
                to_flow.append(k)
                continue
            
            if left == Tile.WALL and right == Tile.WALL:
                to_flow.append(k)
                continue

        for pos in to_flow:
            changed |= self.try_flow(pos)

        return changed

    def find_water(self) -> list[(Point, Tile)]:
        return [(k, v) for k, v in self.grid.items() if v == Tile.WATER_M or v == Tile.WATER_S]

    def print(self):
        rows = []
        for y in range(0, 40):
            row = []
            for x in range(self.x_min, self.x_max + 1):
                row.append(self[Point(x, y)])
            rows.append("".join(map(str, row)))
        print("\n".join(rows))


world = World("input")

while world.simulate_water():
    # print()
    # world.print()
    # input()
    pass

# world.print()
print("Part 1:", len(world.find_water()))

# world.simulate_water()
# world.print()

# world.simulate_water()
# world.print()

# TODO: Consider using BFS or something, still getting wrong answer