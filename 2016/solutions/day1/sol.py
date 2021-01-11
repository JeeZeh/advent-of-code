import math

Point = tuple[int, int]

def rotate(p: Point, angle: int):
    angle = math.radians(angle)
    px, py = p
    qx = math.cos(angle) * px - math.sin(angle) * py
    qy = math.sin(angle) * px + math.cos(angle) * py
    return round(qx), round(qy)


def solve():
    history = set()
    heading: Point = (0, 1)
    position: Point = (0, 0)
    bunny_hq: Point = None

    instructions = open("real.txt").read().split(", ")

    for ins in instructions:
        rotation_dir = 1 if ins[0] == "R" else -1
        steps = int(ins[1:])

        heading = rotate(heading, 90 * rotation_dir)

        for _ in range(steps):
            position = position[0] + heading[0], position[1] + heading[1]

            if not bunny_hq and position in history:
                bunny_hq = position

            history.add(position)

    print(f"Distance from start: {abs(position[0]) + abs(position[1])}")

    if bunny_hq:
        print(f"Distance from Easter Bunny HQ: {abs(bunny_hq[0]) + abs(bunny_hq[1])}")


if __name__ == "__main__":
    solve()