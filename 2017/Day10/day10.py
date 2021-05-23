from collections import deque

hash_len = 256


def get_strip(string, start, n):
    strip = []
    start %= hash_len

    while n != 0:
        strip.append(string[start])
        n -= 1
        start = (start + 1) % hash_len

    return reversed(strip)


def write_strip(string, start, strip):
    for i, num in enumerate(strip):
        string[(start + i) % hash_len] = num

    return string


def part_one():
    lengths = deque(map(int, open("Day10/input").read().split(",")))

    string = list(range(hash_len))

    pos = 0
    skip_size = 0
    while lengths:
        length = lengths.popleft()

        string = write_strip(string, pos, get_strip(string, pos, length))

        pos = (pos + length + skip_size) % hash_len
        skip_size += 1

    print(string[0] * string[1])


def part_two():
    lengths = list(map(ord, open("Day10/input").read())) + [17, 31, 73, 47, 23]
    # lengths = [49, 44, 50, 44, 51, 17, 31, 73, 47, 23]
    string = list(range(hash_len))
    pos = 0
    skip_size = -1
    for _ in range(64):
        for length in lengths:
            skip_size += 1
            string = write_strip(string, pos, get_strip(string, pos, length))

            pos = (pos + length + skip_size) % hash_len

    dense = []
    for start in range(0, hash_len, 16):
        xord = 0
        for t in string[start : start + 16]:
            xord ^= t

        dense.append(xord)

    print("".join(map('{:02x}'.format, dense)))


part_one()
part_two()