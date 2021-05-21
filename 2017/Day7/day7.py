from collections import defaultdict
from distutils.command.install_egg_info import install_egg_info
from email.policy import default
from multiprocessing.context import get_spawning_popen


def parse_tower(t) -> tuple[str, int, list[str]]:
    parts = t.split(" -> ")
    name, weight = parts[0].split()
    weight = int(weight[1:-1])

    supporting = []
    if len(parts) == 2:
        supporting = parts[1].split(", ")

    return name, weight, supporting


towers = {
    tower: (weight, supporting)
    for tower, weight, supporting in map(
        parse_tower,
        open("input").read().splitlines(),
    )
}


def find_imbalance(node):
    offenders = []
    weight, supporting = towers.get(node)

    if not supporting:
        return (weight, offenders)

    subs = defaultdict(list)
    supporting_weights = [(node, *find_imbalance(node)) for node in supporting]
    for _node, _weight, _offenders in supporting_weights:
        subs[_weight].append(_node)
        offenders += _offenders

    if len(subs) == 2:
        offender, correct = sorted(list(subs.items()), key=lambda x: len(x[1]))
        offenders.append((sum(x[1] for x in supporting_weights), offender, correct))

    return weight + sum(x[1] for x in supporting_weights), offenders


def part_one():
    inverse = {t: None for t in towers}

    for tower, meta in towers.items():
        weight, supports = meta

        for s in supports:
            inverse[s] = tower

    bottom_tower = [k for k, v in inverse.items() if v is None][0]

    print(f"Bottom tower: {bottom_tower}")

    return bottom_tower


def part_two():
    towers = {
        tower: (weight, supporting)
        for tower, weight, supporting in map(
            parse_tower,
            open("input").read().splitlines(),
        )
    }

    bottom_tower = part_one()

    imbalances = find_imbalance(bottom_tower)[1]
    _, bad, good = sorted(imbalances)[0]
    bad_name = bad[1][0]
    offset = good[0] - bad[0]
    bad_weight = towers[bad_name][0]

    print(
        f"Tower {bad_name} needs weight {bad_weight + offset} ({offset}) in order to be balanced"
    )

    return bad_weight + offset


part_two()
