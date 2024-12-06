from enum import Enum

from aocd import get_puzzle

from advent.grid import Grid
from advent.pos import Direction, Pos
from advent.solution import Solution


class Tile(Enum):
    SPACE = "."
    BLOCK = "#"

    @staticmethod
    def from_char(c: str):
        match c:
            case Tile.SPACE.value:
                return Tile.SPACE
            case Tile.BLOCK.value:
                return Tile.BLOCK
            case "^":
                return Tile.SPACE
            case _:
                raise ValueError(f"Unknown value: {c}")

    def __str__(self: Enum) -> str:
        return self.value


DIRECTIONS = [
    Direction.UP,
    Direction.RIGHT,
    Direction.DOWN,
    Direction.LEFT,
]


class Day06(Solution):
    def find_guard(self, puzzle_input: str):
        for y, line in enumerate(puzzle_input.splitlines()):
            for x, c in enumerate(line):
                if c == "^":
                    return Pos(x, y)

        raise ValueError("No guard found")

    def patrol(self, floor: Grid[Tile], guard: Pos, facing: int):
        guard = guard
        while True:
            yield guard
            step = guard.add(DIRECTIONS[facing % len(DIRECTIONS)].value)
            match floor.get(step):
                case Tile.SPACE:
                    guard = step
                case Tile.BLOCK:
                    facing += 1
                case None:
                    return

    def run(self, puzzle_input: str):
        floor: Grid[Tile] = Grid.from_string(puzzle_input, mapper=Tile.from_char)
        guard = self.find_guard(puzzle_input)

        walk = list(self.patrol(floor, guard, DIRECTIONS.index(Direction.UP)))

        return len(set(walk)), None


if __name__ == "__main__":
    print(Day06().run(get_puzzle(day=6, year=2024).input_data))
