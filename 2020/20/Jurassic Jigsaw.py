import enum
from math import sqrt
import numpy as np
from math import prod
from itertools import product
import numpy as np

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
        self.tile = self.tile[1:-1, 1:-1]
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


def crop_and_merge_image(image, size):
    new_image = image.copy()

    for pos, tile in image.items():
        new_image[pos] = tile.strip_borders()

    merged = np.concatenate(
        tuple(
            (np.concatenate(tuple(new_image[(x, y)].tile for x in range(size)), axis=1))
            for y in range(size)
        ),
        axis=0,
    )

    return merged


def is_wildcard(array):
    def func(a):
        return a == 0

    return np.vectorize(func)(array)


def find_monster_indexes(slice_range, monster_pattern):
    y_slice, x_slice = slice_range

    y_offset = y_slice[0]
    x_offset = x_slice[0]

    positions = set()
    for y, row in enumerate(monster_pattern):
        for x, m in enumerate(row):
            if m == 1:
                positions.add((y + y_offset, x + x_offset))

    return positions


# Thought about doing a sort-of convolution, found this: https://stackoverflow.com/a/52095255
# Changed some parts to count 0 as a wildcard, and also to collect the indices that were matched
def match_pattern(input_array, pattern, wildcard_function=is_wildcard):
    pattern_shape = pattern.shape
    input_shape = input_array.shape

    is_wildcard = wildcard_function(pattern)  # This gets a boolean N-dim array

    if len(pattern_shape) != len(input_shape):
        raise ValueError("Input array and pattern must have the same dimension")

    shape_difference = [i_s - p_s for i_s, p_s in zip(input_shape, pattern_shape)]

    if any((diff < -1 for diff in shape_difference)):
        raise ValueError("Input array cannot be smaller than pattern in any dimension")

    dimension_iterators = [range(0, s_diff + 1) for s_diff in shape_difference]

    matches = set()

    # This loop will iterate over every possible "window" given the shape of the pattern
    for start_indexes in product(*dimension_iterators):
        indices = [
            (start_i, start_i + p_s)
            for start_i, p_s in zip(start_indexes, pattern_shape)
        ]
        range_indexes = tuple(map(lambda x: slice(*x), indices))
        input_match_candidate = input_array[range_indexes]

        # This checks that for the current "window" - the candidate - every element is equal
        #  to the pattern OR the element in the pattern is a wildcard
        if np.all(np.logical_or(is_wildcard, (input_match_candidate == pattern))):
            matches |= find_monster_indexes(indices, pattern)

    return matches


### Part 1 ###
def part_1(image, size):
    corners = [
        image[(0, 0)],
        image[(0, size - 1)],
        image[(size - 1, 0)],
        image[(size - 1, size - 1)],
    ]

    print(prod(int(c.id_) for c in corners))


monster = """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"""


def part_2(image, size):
    monster_pattern = np.array(
        [[1 if c == "#" else 0 for c in line] for line in monster.splitlines()],
        dtype=int,
    )
    search_space = Tile(0, crop_and_merge_image(image, size))

    search_space.generate_transforms()

    matches = [
        (match_pattern(ss.tile, monster_pattern), ss.tile)
        for ss in search_space.generate_transforms()
    ]
    monsters, space = list(filter(lambda x: x[0], matches))[0]
    m_count = len(monsters)
    
    
    print(space.sum() - m_count)


part_1(image, size)
part_2(image, size)
