from enum import Enum

from advent.grid import Grid
from advent.pos import Pos
from advent.solution import Solution


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
        xmas = 0
        for pos in grid.scan():
            if grid.get(pos) == word[0]:
                for direction in Direction:
                    xmas += self.search_word(grid, pos.add(direction.value), word[1:], direction.value)

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
