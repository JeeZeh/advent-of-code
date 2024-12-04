from dataclasses import dataclass
from enum import Enum
from typing import Self

from aocd import get_puzzle

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

    @staticmethod
    def from_string(string: str):
        return Grid([list(line) for line in string.splitlines()])

    def get(self, pos: Pos):
        try:
            return self.grid[pos.y][pos.x]
        except:  # noqa: E722
            return None

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
    def search_word(self, grid: Grid, pos: Pos, need: list[str], direction: Direction):
        if grid.get(pos) == need[0]:
            if len(need) == 1:
                return 1

            return self.search_word(grid, pos.add(direction.value), need[1:], direction)

        return 0

    def run(self, puzzle_input: str):
        grid: Grid = Grid.from_string(puzzle_input)

        word = ["X", "M", "A", "S"]
        count = 0
        for pos in grid.scan():
            if grid.get(pos) == word[0]:
                for direction in Direction:
                    count += self.search_word(grid, pos.add(direction.value), word[1:], direction)

        return count, None


if __name__ == "__main__":
    print(Day04().run(get_puzzle(day=4, year=2024).input_data))
