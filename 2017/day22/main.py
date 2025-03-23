from dataclasses import dataclass
from enum import Enum
from typing import NamedTuple


class Pos(NamedTuple):
    x: int
    y: int

    def __lt__(self, value):
        return self.x < value.x or (self.x == value.x and self.y < value.y)

    def __gt__(self, value):
        return self.x > value.x or (self.x == value.x and self.y > value.y)

    def __add__(self, value):
        return Pos(self.x + value.x, self.y + value.y)

    def __sub__(self, value):
        return Pos(self.x - value.x, self.y - value.y)

    def __eq__(self, value):
        return self.x == value.x and self.y == value.y


class Direction(Enum):
    UP = Pos(0, -1)
    DOWN = Pos(0, 1)
    LEFT = Pos(-1, 0)
    RIGHT = Pos(1, 0)

    def turn_left(self):
        return {
            Direction.UP: Direction.LEFT,
            Direction.LEFT: Direction.DOWN,
            Direction.DOWN: Direction.RIGHT,
            Direction.RIGHT: Direction.UP,
        }[self]

    def turn_right(self):
        return {
            Direction.UP: Direction.RIGHT,
            Direction.RIGHT: Direction.DOWN,
            Direction.DOWN: Direction.LEFT,
            Direction.LEFT: Direction.UP,
        }[self]

    def turn_around(self):
        return {
            Direction.UP: Direction.DOWN,
            Direction.DOWN: Direction.UP,
            Direction.LEFT: Direction.RIGHT,
            Direction.RIGHT: Direction.LEFT,
        }[self]


class InfiniteGrid(dict):
    min_x = 0
    min_y = 0
    max_x = 0
    max_y = 0

    def __update_extent(self, pos: Pos):
        self.min_x = min(self.min_x, pos.x)
        self.min_y = min(self.min_y, pos.y)
        self.max_x = max(self.max_x, pos.x)
        self.max_y = max(self.max_y, pos.y)

    def get_middle(self):
        return Pos(
            ((self.max_x - self.min_x) + 1) // 2,
            ((self.max_y - self.min_y) + 1) // 2,
        )

    def __setitem__(self, name, value):
        match name:
            case Pos(_, _) as p:
                self.__update_extent(p)
                dict.__setitem__(self, p, value)
            case _:
                raise ValueError("Only Pos can be used as key")

    def display(self, padding: int = 0, map_fn=None):
        for y in range(self.min_y - padding, self.max_y + padding + 1):
            for x in range(self.min_x - padding, self.max_x + padding + 1):
                if map_fn:
                    print(map_fn(Pos(x, y), self.get(Pos(x, y), None)), end="")
                else:
                    print(self.get(Pos(x, y), "."), end="")
            print()


@dataclass
class Carrier:
    pos: Pos
    direction: Direction


class NodeState(Enum):
    CLEAN = 0
    WEAKENED = 1
    INFECTED = 2
    FLAGGED = 3

    def __str__(self):
        return {
            NodeState.CLEAN: ".",
            NodeState.WEAKENED: "W",
            NodeState.INFECTED: "#",
            NodeState.FLAGGED: "F",
        }[self]


@dataclass
class Cluster:
    grid: InfiniteGrid
    carrier: Carrier

    @staticmethod
    def from_input(input_file):
        grid = InfiniteGrid()

        with open(input_file) as f:
            for y, line in enumerate(f.readlines()):
                for x, char in enumerate(line.strip()):
                    if char == "#":
                        grid[Pos(x, y)] = NodeState.INFECTED

        carrier = Carrier(grid.get_middle(), Direction.UP)
        return Cluster(grid, carrier)

    def __draw_tile(self, pos, value):
        tile = str(value or NodeState.CLEAN)

        return f"\033[34;1;4m{tile}\033[0m " if pos == self.carrier.pos else f"{tile} "

    def display(self):
        self.grid.display(2, map_fn=self.__draw_tile)
        print("Carrier at", self.carrier.pos, "facing", self.carrier.direction)

    def burst(self):
        new_state: NodeState
        match self.grid.get(self.carrier.pos, NodeState.CLEAN):
            case NodeState.CLEAN:
                self.carrier.direction = self.carrier.direction.turn_left()
                new_state = NodeState.WEAKENED
            case NodeState.WEAKENED:
                new_state = NodeState.INFECTED
            case NodeState.INFECTED:
                self.carrier.direction = self.carrier.direction.turn_right()
                new_state = NodeState.FLAGGED
            case NodeState.FLAGGED:
                self.carrier.direction = self.carrier.direction.turn_around()
                new_state = NodeState.CLEAN
            case _:
                raise ValueError("Invalid state")

        self.grid[self.carrier.pos] = new_state

        self.carrier.pos += self.carrier.direction.value

        return new_state == NodeState.INFECTED


def main():
    cluster = Cluster.from_input("input/real")
    cluster.display()

    caused_infection = 0
    for _ in range(10_000_000):
        caused_infection += cluster.burst()

    print("Caused infections:", caused_infection)


if __name__ == "__main__":
    main()
