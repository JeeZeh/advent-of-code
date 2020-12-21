from math import sqrt
import numpy as np
from math import prod

from numpy.lib.type_check import imag

tiles_raw = open("real.txt").read().split("\n\n")
w, h = 10, 10


class Tile:
    def __init__(self, id_, tile) -> None:
        self.id_ = id_
        self.tile = tile
        self.set_border()

    def set_border(self):
        top = "".join(map(str, self.tile[0, :]))
        right = "".join(map(str, self.tile[:, w - 1]))
        bottom = "".join(map(str, self.tile[h - 1, :]))
        left = "".join(map(str, self.tile[:, 0]))
        self.border = (top, right, bottom, left)

    def rotate(self, k):
        self.tile = np.rot90(self.tile, k=k)
        self.set_border()
        return self

    def flip(self, a):
        if a is not None:
            self.tile = np.flip(self.tile, a)
        self.set_border()
        return self

    def can_stitch(self, b, side):
        side_a = self.border[side]
        side_b = b.border[(side + 2) % 4]

        return side_a == side_b

    def generate_transforms(self):
        transforms = []

        for f in [None, 0]:
            for r in [0, 1, 2, 3]:
                transforms.append(Tile(self.id_, self.tile).rotate(r).flip(f))

        return transforms
    
    def strip_borders(self):
        self.tile = self.tile[1:-1,1:-1]
        return self


def parse_tiles(raw):
    tiles = {}

    for tile in raw:
        lines = tile.splitlines()
        tile_id = lines[0].split()[1][:-1]

        np_tile = np.ndarray((w, h), dtype=int)
        for j, r in enumerate(lines[1:]):
            for i, c in enumerate(r):
                np_tile[j][i] = 1 if c == "#" else 0

        tiles[tile_id] = Tile(tile_id, np_tile)

    return tiles


tiles = parse_tiles(tiles_raw)


def get_surrounding(grid, x, y) -> dict:
    dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)]
    surr = {}
    for i, d in enumerate(dirs):
        dx = x + d[0]
        dy = y + d[1]
        if tile := grid.get((dx, dy)):
            surr[i] = tile

    return surr


def can_be_placed(to_be_placed: Tile, grid, x, y):
    surrounding_tiles = get_surrounding(grid, x, y)

    if surrounding_tiles:
        return all(
            to_be_placed.can_stitch(neighbour, side)
            for side, neighbour in surrounding_tiles.items()
        )
    else:
        return True


# Shamelessly stolen from busdriverbuddha: https://github.com/busdriverbuddha/aoc2020_solutions/blob/main/Day%2020.py
# I had all the pieces but forgot how to implement backtracking :(
def stitch_image(tiles):
    GRID_SIZE = int(sqrt(len(tiles)))

    path = [(i, j) for i in range(GRID_SIZE) for j in range(GRID_SIZE)]
    variations = {t_id: t.generate_transforms() for t_id, t in tiles.items()}
    grid = {}
    unused_ids = list(tiles.keys())
    step = 0

    starting_problem = {
        "step": step,
        "grid": grid,
        "unused_ids": unused_ids,
    }

    stack = [starting_problem]

    while stack:
        p = stack.pop()
        step = p["step"]
        grid = p["grid"]
        unused_ids = p["unused_ids"]

        if step == len(path):  # success
            return p["grid"], GRID_SIZE

        x, y = path[step]

        for t_id in unused_ids:
            for var in variations[t_id]:
                if not can_be_placed(var, grid, x, y):
                    continue

                _grid = grid.copy()
                _grid[(x, y)] = var
                _unused_ids = unused_ids.copy()
                _unused_ids.remove(t_id)
                stack.append(
                    {"grid": _grid, "unused_ids": _unused_ids, "step": step + 1}
                )

image, size = stitch_image(tiles)


### Part 1 ###
def part_1(image, size):
    corners = [
        image[(0, 0)],
        image[(0, size - 1)],
        image[(size - 1, 0)],
        image[(size - 1, size - 1)],
    ]
    
    print(prod(int(c.id_) for c in corners))

part_1(image, size)