from dataclasses import dataclass
from enum import Enum
from typing import Self

from advent.solution import Solution


@dataclass
class Pos:
    x: int
    y: int

    def add(self, other: Self):
        return Pos(self.x + other.x, self.y + other.y)


class Direction(Enum):
    UP_LEFT = Pos(-1, -1)
    UP = Pos(0, -1)
    UP_RIGHT = Pos(1, -1)
    RIGHT = Pos(1, 0)
    DOWN_RIGHT = Pos(1, 1)
    DOWN = Pos(0, 1)
    DOWN_LEFT = Pos(-1, 1)
    LEFT = Pos(-1, 0)


@dataclass
class Grid:
    grid: list[list[str | None]]
    width: int
    height: int

    @staticmethod
    def from_string(string: str):
        grid_list = [list(line) for line in string.splitlines()]
        return Grid.from_list(grid_list)  # type: ignore

    @staticmethod
    def from_list(grid_list: list[list[str | None]]):
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

    def kernel_match(self, kernel: Self, ref: Pos):
        for k_y in range(kernel.height):
            for k_x in range(kernel.width):
                local = Pos(k_x, k_y)
                if (value := kernel.get(local)) and value != self.get(ref.add(local)):
                    return False
        return True

    def find_convolution_matches(self, kernel: Self):
        count = 0
        for ref_y in range(self.height - kernel.height + 1):
            for ref_x in range(self.width - kernel.width + 1):
                if self.kernel_match(kernel, ref=Pos(ref_x, ref_y)):
                    count += 1

        return count

    def rotate(self):
        return Grid(
            grid=list(zip(*self.grid[::-1], strict=False)),
            width=self.width,
            height=self.height,
        )


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
        xmas = 0
        for pos in grid.scan():
            if grid.get(pos) == word[0]:
                for direction in Direction:
                    xmas += self.search_word(
                        grid, pos.add(direction.value), word[1:], direction.value
                    )

        kernel = Grid.from_list(
            [
                ["M", None, "M"],
                [None, "A", None],
                ["S", None, "S"],
            ]
        )

        x_mas = 0
        for _rotations in range(4):
            x_mas += grid.find_convolution_matches(kernel)
            kernel = kernel.rotate()

        return xmas, x_mas


if __name__ == "__main__":
    print(Day04().run(Puzzle(day=4, year=2024).input_data))
