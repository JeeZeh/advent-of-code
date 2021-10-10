card_contents = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1,
}

sues = []

for line in open("input.txt").read().splitlines():
    parts = [tuple(l.split(": ")) for l in line[line.index(": ") + 2 :].split(", ")]
    sues.append({k: int(v) for k, v in parts})

# print(sues)


def check_sue(sue_parts: dict):
    for part, count in sue_parts.items():
        if card_contents[part] != count:
            return False
    return True


def check_sue_v2(sue_parts: dict):
    for part, count in sue_parts.items():
        if part in ["cats", "trees"]:
            if card_contents[part] >= count:
                return False
        elif part in ["pomeranians", "goldfish"]:
            if card_contents[part] <= count:
                return False
        elif card_contents[part] != count:
            return False
    return True


for i, sue in enumerate(sues):
    if check_sue(sue):
        print("Part 1:", f"Found Sue #{i+1}")

for i, sue in enumerate(sues):
    if check_sue_v2(sue):
        print("Part 2:", f"Found Sue #{i+1}")
