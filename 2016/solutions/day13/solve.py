from dataclasses import dataclass
from queue import PriorityQueue
from typing import Callable, NamedTuple, Self


class Pos(NamedTuple):
    x: int
    y: int

    def add(self, other: Self):
        return Pos(self.x + other.x, self.y + other.y)

    def __eq__(self, value):
        return self.x == value.x and self.y == value.y


class Maze:
    grid: dict[Pos, bool]
    seed: int

    def __init__(self, designer_num: int):
        self.grid = {}
        self.seed = designer_num

    def visualize(self, location: Pos, dest: Pos):
        rows: list[str] = []
        extent = dest.add(Pos(2, 2))
        for y in range(extent.y):
            row = []
            for x in range(extent.x):
                p = Pos(x, y)
                if location == p:
                    c = "O"
                elif dest == p:
                    c = "X"
                else:
                    c = "#" if self.is_wall(p) else "."
                row.append(c)
            rows.append("".join(row))
        print("\n".join(rows))

    def is_wall(self, p: Pos):
        if p.x < 0 or p.y < 0:
            return True

        if p in self.grid:
            return self.grid[p]

        token = self.seed + (p.x**2 + 3 * p.x + 2 * p.x * p.y + p.y + p.y**2)

        wall = False
        while token > 0:
            if token & 1:
                wall = not wall
            token >>= 1

        self.grid[p] = wall
        return wall


DIRECTIONS = [
    Pos(0, -1),  # Up
    Pos(1, 0),  # Right
    Pos(0, 1),  # Down
    Pos(-1, 0),  # Left
]


def solve(input_: int):
    maze = Maze(input_)
    start = Pos(1, 1)
    end = Pos(7, 4) if input_ == 10 else Pos(31, 39)

    maze.visualize(start, end)

    part_1 = bfs(maze, start, lambda state: state.loc == end)[0].steps
    part_2 = len(bfs(maze, start, lambda state: state.steps == 50)[1])
    return part_1, part_2


@dataclass(order=True)
class State:
    steps: int
    loc: Pos


def bfs(
    maze: Maze,
    initial: Pos,
    end_state: Callable[[State], bool],
) -> tuple[State, dict[Pos, int]]:
    shortest_paths: dict[Pos, int] = {}
    queue = PriorityQueue[State]()
    queue.put(State(0, initial))
    shortest_paths[initial] = 0

    while not queue.empty():
        item = queue.get()
        steps = item.steps
        loc = item.loc

        if end_state(item):
            return (item, shortest_paths)

        # Try take two
        for d in DIRECTIONS:
            next_pos = loc.add(d)
            if not maze.is_wall(next_pos) and steps + 1 < shortest_paths.get(
                next_pos, 0xFFFF
            ):
                shortest_paths[next_pos] = steps + 1
                queue.put(State(steps + 1, next_pos))

    raise RuntimeError("Failed to reach destination")


p1, p2 = solve(int(open("input/real").readline().strip()))
print("Part 1: ", p1)
print("Part 1: ", p2)
