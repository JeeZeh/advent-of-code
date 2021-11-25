from operator import add
from collections import defaultdict, namedtuple

p = {"right": (1, 0), "left": (-1, 0), "down": (0, -1), "up": (0, 1)}
inner_level_entry = {"right": 0, "left": 4, "down": 0, "up": 4}
outer_level_entry = {"right": (3, 2), "left": (1, 2), "down": (2, 3), "up": (2, 1)}
inner_boundaries = {(3, 2), (1, 2), (2, 3), (2, 1)}
outer_boundaries = (
    {(x, 0) for x in range(5)} | {(x, 4) for x in range(5)} | {(0, y) for y in range(5)} | {(4, y) for y in range(5)}
)

Tile = namedtuple("Tile", ["pos", "depth"])
grid: dict[Tile, bool] = defaultdict(bool)
seen_states = set()

test = """....#
#..#.
#.?##
..#..
#...."""
for y, line in enumerate(open("input.txt").read().splitlines()):
    for x, v in enumerate(line):
        grid[Tile(pos=(x, y), depth=0)] = True if v == "#" else False

WIDTH, HEIGHT = (4, 4)


def print_grid(depth):
    for y in range(HEIGHT + 1):
        row_tiles = [Tile((x, y), depth) for x in range(WIDTH + 1)]
        print("".join("#" if t in grid and grid[t] else "." for t in row_tiles))


def get_neighbour_tiles_in_direction(tile: Tile, direction: str) -> list[bool]:
    new_pos = tuple(map(add, tile.pos, p[direction]))
    neighbours = []
    # Moving to inner grid
    if new_pos == (2, 2):
        if direction in ["left", "right"]:
            neighbours = [Tile((inner_level_entry[direction], y), tile.depth - 1) for y in range(5)]
        else:
            neighbours = [Tile((x, inner_level_entry[direction]), tile.depth - 1) for x in range(5)]
    # Moving to outer grid
    elif min(new_pos) < 0 or max(new_pos) > 4:
        neighbours = [Tile(outer_level_entry[direction], tile.depth + 1)]
    else:
        neighbours = [Tile(new_pos, tile.depth)]

    return neighbours


def get_surrounding(tile: Tile, no_expand=False) -> list[bool]:
    neighbours = []
    for direction in p:
        neighbours += get_neighbour_tiles_in_direction(tile, direction)

    if no_expand:
        return [grid[n] for n in neighbours if n in grid]
    else:
        return [grid[n] for n in neighbours]


# def expand_bounds():
#     max_depth = max(grid.keys(), key=lambda tile: tile.depth)
#     keys_in_max_depth = [k for k in grid if k.depth == max_depth]

#     # If our current max grid has any bugs on the perimeter, we need to expand
#     if any(grid[Tile(pos, max_depth)] for pos in outer_boundaries):
#         grid[]


def tick():
    begin_state = grid.copy()

    updates = {}

    for pos, bug in begin_state.items():
        if pos.pos == (2, 2):
            continue
        adjs = get_surrounding(pos)
        if not bug and sum(adjs) in [1, 2]:
            updates[pos] = True
        elif bug and not sum(adjs) == 1:
            updates[pos] = False

    # By looking around the grid, we've automatically expanded it inwards and
    # outwards. However, these newly evaluated positions should have actually been evaluated
    # as part of the first loop above.
    new_positions = set(grid.keys()) - set(begin_state.keys())

    for pos in new_positions:
        adjs = get_surrounding(pos, no_expand=True)
        if pos.pos == (2, 2):
            continue
        if sum(adjs) in [1, 2]:
            updates[pos] = True

    grid.update(updates)


def get_biodiversity(grid):
    bugs = [(k[1] * 5) + k[0] for k, v in grid.items() if v]
    return [pow(2, b) for b in bugs]


for t in range(200):
    # print_grid(0)
    tick()

print("Part 2:", sum(grid.values()))

# while True:
#     tick()
#     hashed_bugs = tuple(sorted(grid.items()))
#     if hashed_bugs in seen_states:
#         print("SEEN!")
#         print_grid()
#         levels = get_biodiversity(grid)
#         print(levels)
#         print(sum(levels))
#         break
#     else:
#         seen_states.add(hashed_bugs)
