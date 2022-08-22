# Courtesy of u/rhardih: https://www.reddit.com/r/adventofcode/comments/5hbygy/2016_day_9_solutions/dazentu/


def decompress(compressed: str, part_one=False):
    total = 0
    weights = [1] * len(compressed)

    i = 0
    while i < len(compressed):
        if compressed[i] == "(":
            # Scan ahead to the end of the decompression marker
            begin = i + 1
            while compressed[i] != ")":
                i += 1
            end = i

            # Apply the given weight to the next_chars
            # Weight represents the number of times to decompress the characters
            next_chars, weight = map(int, compressed[begin:end].split("x", maxsplit=1))
            for w in range(i + 1, i + next_chars + 1):
                weights[w] *= weight

            # If we're processing part one, we want to jump to the end of the decompression
            # so that we don't try to recursively decompress
            if part_one:
                i += next_chars
                total += (next_chars - 1) * weight

        # So long as we're not at the end of a compression marker, we can add the weight
        if compressed[i] != ")":
            total += weights[i]

        i += 1

    return total


print("Part 1:", decompress(open("real.txt").read(), part_one=True))
print("Part 2:", decompress(open("real.txt").read()))
