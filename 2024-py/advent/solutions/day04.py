from dataclasses import dataclass
from enum import Enum
from typing import Self

from aocd.models import Puzzle

from advent.solution import Solution


@dataclass
class Pos:
    x: int
    y: int

    def add(self, other: Self):
        return Pos(self.x + other.x, self.y + other.y)


@dataclass
class Grid:
    grid: list[list[str]]
    width: int
    height: int

    @staticmethod
    def from_string(string: str):
        grid_list = [list(line) for line in string.splitlines()]
        if not grid_list:
            raise ValueError("Grid empty")

        height = len(grid_list)
        expected_width = len(grid_list[0])

        # Check is square
        for row in grid_list:
            if not row or len(row) != expected_width:
                raise ValueError("Grid is not square")

        return Grid(grid=grid_list, width=expected_width, height=height)

    def get(self, pos: Pos, *, wrap: bool = False):
        if wrap:
            try:
                return self.grid[pos.y][pos.x]
            except KeyError:
                return None
        else:
            if not 0 <= pos.y < self.height or not 0 <= pos.x < self.width:
                return None

            return self.grid[pos.y][pos.x]

    def rows(self):
        return self.grid.__iter__()

    def scan(self):
        for y in range(len(self.grid)):
            for x in range(len(self.grid[y])):
                yield Pos(x, y)


class Direction(Enum):
    UP_LEFT = Pos(-1, -1)
    UP = Pos(0, -1)
    UP_RIGHT = Pos(1, -1)
    RIGHT = Pos(1, 0)
    DOWN_RIGHT = Pos(1, 1)
    DOWN = Pos(0, 1)
    DOWN_LEFT = Pos(-1, 1)
    LEFT = Pos(-1, 0)


class Day04(Solution):
    def search_word(self, grid: Grid, pos: Pos, need: list[str], direction: Pos) -> int:
        check = pos
        while need:
            if grid.get(check) == need[0]:
                if len(need) == 1:
                    return 1

                need = need[1:]
                check = check.add(direction)
            else:
                break

        return 0

    def run(self, puzzle_input: str):
        grid: Grid = Grid.from_string(puzzle_input)

        word = ["X", "M", "A", "S"]
        count = 0
        for pos in grid.scan():
            if grid.get(pos) == word[0]:
                for direction in Direction:
                    count += self.search_word(
                        grid, pos.add(direction.value), word[1:], direction.value
                    )

        return count, None


if __name__ == "__main__":
    print(Day04().run(Puzzle(day=4, year=2024).input_data))
