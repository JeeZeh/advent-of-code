from math import prod
from collections import defaultdict

document = open("real.txt").read().split("\n\n")


def parse_rules(rules):
    lines = [rule.split(":") for rule in rules.splitlines()]
    rules = []
    for line in lines:
        name = line[0]
        r1, r2 = line[1].split(" or ")
        r1 = tuple(map(int, r1.split("-")))
        r2 = tuple(map(int, r2.split("-")))
        ruleset = set(range(r1[0], r1[1] + 1)) | set(range(r2[0], r2[1] + 1))
        rules.append((name, ruleset))
    return rules


yours = list(map(int, document[1].splitlines()[1].split(",")))
nearby = [list(map(int, x.split(","))) for x in document[2].splitlines()[1:]]
rules = parse_rules(document[0])


def get_invalid(ticket):
    return [n for n in ticket if not any (n in ruleset for _, ruleset in rules)]


def get_rule_columns(tickets):
    columns = defaultdict(list)
    for i in range(len(tickets[0])):
        for name, ruleset in rules:
            if all(ticket[i] in ruleset for ticket in tickets):
                columns[name].append(i)

    taken_positions = set()
    filtered_columns = {}
    while len(columns) != len(taken_positions):
        for c, v in columns.items():
            if len(v) == 1:
                filtered_columns[c] = v[0]
                taken_positions.add(v[0])

        columns = {
            c: [x for x in v if x not in taken_positions] for c, v in columns.items()
        }

    return filtered_columns


tickets = [(ticket, get_invalid(ticket)) for ticket in nearby]
leftover = [ticket[0] for ticket in tickets if not ticket[1]]

print(sum(num for ticket in tickets for num in ticket[1] if ticket[1]))
print(prod(yours[v] for k, v in get_rule_columns(leftover).items() if k[:2] == "de"))
