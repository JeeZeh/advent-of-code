from copy import deepcopy
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
            yield guard, facing
            step = guard.add(DIRECTIONS[facing].value)
            match floor.get(step):
                case Tile.SPACE:
                    guard = step
                case Tile.BLOCK:
                    facing = (facing + 1) % len(DIRECTIONS)
                case None:
                    return

    def loops(self, floor: Grid[Tile], guard: Pos, facing: int):
        turtle_generator = self.patrol(floor, guard, facing)
        hare_generator = self.patrol(floor, guard, facing)

        turtle = next(turtle_generator)
        hare = next(hare_generator)
        next(hare_generator)
        try:
            while True:
                if hare == turtle:
                    return True
                next(hare_generator)
                hare = next(hare_generator)
                turtle = next(turtle_generator)
        except StopIteration:
            return False

    def run(self, puzzle_input: str):
        floor: Grid[Tile] = Grid.from_string(puzzle_input, mapper=Tile.from_char)
        guard = self.find_guard(puzzle_input)

        start = guard.add(Direction.UP.value)
        walk: list[tuple[Pos, int]] = list(self.patrol(floor, guard, DIRECTIONS.index(Direction.UP)))
        seen: set[tuple[Pos, int]] = {(start, 0)}
        looping_blocks: list[Pos] = []
        for walking, facing in walk:
            # Place block
            place_at = walking.add(DIRECTIONS[facing].value)
            if floor.get(place_at) == Tile.SPACE and (place_at, facing) not in seen:
                new_floor = deepcopy(floor)
                new_floor.grid[place_at.y][place_at.x] = Tile.BLOCK
                if self.loops(new_floor, walking, facing):
                    looping_blocks.append(place_at)
            seen.add((place_at, facing))

        return len(set(pos for pos, _ in walk)), len(set(looping_blocks))


if __name__ == "__main__":
    print(Day06().run(get_puzzle(day=6, year=2024).examples[0].input_data))
