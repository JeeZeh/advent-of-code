from math import prod

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


def knot_hash(string: str, partial=False):
    if partial:
        lengths = list(map(int, string.split(",")))
    else:
        lengths = list(map(ord, string)) + [17, 31, 73, 47, 23]

    output = list(range(hash_len))
    pos = 0
    skip_size = 0
    for _ in range(64 if not partial else 1):
        for length in lengths:
            output = write_strip(output, pos, get_strip(output, pos, length))

            pos = (pos + length + skip_size) % hash_len
            skip_size += 1

    if partial:
        return output

    dense = []
    for start in range(0, hash_len, 16):
        xord = 0
        for t in output[start : start + 16]:
            xord ^= t

        dense.append(xord)

    return "".join(map("{:02x}".format, dense))


def part_one():
    print(prod(hash(open("input").read(), partial=True)[:2]))


def part_two():
    print(hash(open("input").read()))

if __name__ == "__main__":
    part_one()
    part_two()