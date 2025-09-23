import re
from collections import defaultdict
from hashlib import md5

# Regex for detecting same character 3 times in a row
TRIPLE_RE = re.compile(r"(.)\1\1")
# For detecting same character 5 times in a row
QUINTUPLE_RE = re.compile(r"(.)\1\1\1\1")


def solve(input_: str):
    keygen_1 = generate_key(input_)
    keygen_2 = generate_key(input_, stretch=True)

    part_1 = sorted([next(keygen_1) for _ in range(67)])[63]
    part_2 = sorted([next(keygen_2) for _ in range(67)])[63]
    return part_1, part_2


def generate_key(salt: str, stretch: bool = False):
    last_triple_index: dict[str, list[int]] = defaultdict(list)
    round = 0
    while True:
        hash = md5(f"{salt}{round}".encode()).hexdigest()
        if stretch:
            for _ in range(2016):
                hash = md5(hash.encode()).hexdigest()
        if match := QUINTUPLE_RE.search(hash):
            quint_char = match.group(1)
            if last_round := last_triple_index[quint_char]:
                for r in last_round:
                    if 0 < round - r <= 1000:
                        yield r

            last_triple_index[match.group(1)].append(round)
        elif match := TRIPLE_RE.search(hash):
            last_triple_index[match.group(1)].append(round)
        round += 1


p1, p2 = solve(open("input/real").readline().strip())
print("Part 1: ", p1)
print("Part 2: ", p2)
