from collections import defaultdict
from pprint import pprint


def parse_bag(bag):
    parts = bag.split(" bags contain ")
    parent = parts[0]
    contents = []
    for content in map(str.split, parts[1].split(",")):
        number = content.pop(0)
        colour = content[:-1]

        if number != "no":
            contents.append((int(number), " ".join(colour)))

    return parent, contents


bags = list(map(parse_bag, map(str.strip, open("input.txt").readlines())))

nest = {parent: contains for parent, contains in bags}


def inside_out(bags):
    out = defaultdict(list)
    for parent, contents in bags.items():
        for content in contents:
            out[content[1]].append((content[0], parent))

    return out


def get_hierarchy(inv_nest, colour):
    hierarchy = []
    if colour not in inv_nest:
        return hierarchy

    for bag in inv_nest[colour]:
        hierarchy.append(bag[1])
        hierarchy.extend(get_hierarchy(inv_nest, bag[1]))

    return hierarchy


def get_nesting(nest, colour):
    required = 1
    if colour not in nest:
        return required

    for bag in nest[colour]:
        required += bag[0] * get_nesting(nest, bag[1])

    return required


inverted_nest = inside_out(nest)

pprint(len(set(get_hierarchy(inverted_nest, "shiny gold"))))
pprint(get_nesting(nest, "shiny gold") - 1)