from collections.abc import Callable
from dataclasses import dataclass
from typing import Self

from advent.pos import Pos


@dataclass
class Grid[T]:
    grid: list[list[T]]
    width: int
    height: int

    @staticmethod
    def from_string(string: str, mapper: Callable[[str], T] = lambda c: str(c)):
        grid_list = [[mapper(c) for c in line] for line in string.splitlines()]
        return Grid[T].from_list(grid_list)

    @staticmethod
    def from_list(grid_list: list[list[T]]):
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

    def __str__(self):
        return "\n".join("".join(str(g) for g in line) for line in self.grid)
