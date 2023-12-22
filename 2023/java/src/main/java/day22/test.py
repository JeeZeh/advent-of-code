from collections import defaultdict, deque

FILENAME = "input.txt"


def parse_brick(brick):
    return [[int(num) for num in pos.split(",")] for pos in brick.split("~")]


def parse_input(filename):
    with open(filename, "r") as input_file:
        return [parse_brick(brick) for brick in input_file.read().split("\n")]


def create_bricks(data):
    return [
        {
            (x, y, z)
            for x in range(x1, x2 + 1)
            for y in range(y1, y2 + 1)
            for z in range(z1, z2 + 1)
        }
        for (x1, y1, z1), (x2, y2, z2) in data
    ]


def hit_bottom(brick):
    return any(pos[-1] == 0 for pos in brick)


def simulate_fall(bricks):
    jenga = {}
    graph = {i: set() for i in range(len(bricks))}
    for i, brick in enumerate(bricks):
        next_pos = {(x, y, z - 1) for x, y, z in brick}
        intersected = {jenga.get(pos) for pos in next_pos if pos in jenga}
        while not intersected and not hit_bottom(next_pos):
            brick = next_pos
            next_pos = {(x, y, z - 1) for x, y, z in brick}
            intersected = {jenga.get(pos) for pos in next_pos if pos in jenga}
        jenga |= {pos: i for pos in brick}
        for b in intersected:
            graph[b].add(i)
    return graph


def supported_bricks(graph):
    supported = defaultdict(set)
    for parent, children in graph.items():
        for child in children:
            supported[child].add(parent)
    return supported


def part_one(graph, supported):
    safe = set()
    for parent, children in graph.items():
        if not children or all(len(supported[child]) > 1 for child in children):
            safe.add(parent)
    return safe


def bfs(graph, supported, root):
    count = 0
    removed = set()
    queue = deque([root])
    while queue:
        current = queue.popleft()
        removed.add(current)
        for child in graph[current]:
            if len(supported[child] - removed) == 0:
                count += 1
                queue.append(child)
    return count


def part_two(graph, supported, unsafe):
    return sum(bfs(graph, supported, brick) for brick in unsafe)


def main():
    data = parse_input(FILENAME)
    data.sort(key=lambda brick: brick[0][2])

    bricks = create_bricks(data)
    graph = simulate_fall(bricks)
    print(graph)
    supported = supported_bricks(graph)
    safe = part_one(graph, supported)
    print(len(safe))
    unsafe = set(range(len(bricks))) - safe
    print(part_two(graph, supported, unsafe))


if __name__ == "__main__":
    main()
