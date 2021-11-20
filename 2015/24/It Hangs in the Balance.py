import cProfile
from cgitb import small
from itertools import combinations, groupby, permutations
from math import prod


weights = set(map(int, open("input.txt")))
# weights = set(range(1, 6)) | set(range(7, 12))


def part_one():
    numbers = len(weights)
    sum_numbers = sum(weights)
    amounts = [
        (a, b, c)
        for a in range(1, numbers)
        for b in range(1, numbers)
        for c in range(1, numbers)
        if a + b + c == len(weights)
    ]
    smallest_a = len(weights)
    lowest_entanglement = None

    for a, b, _ in amounts:
        if a > smallest_a:
            continue
        for group_a in combinations(weights, a):
            if a > smallest_a:
                continue

            sum_a = sum(group_a)
            if sum_a * 3 != sum_numbers:
                continue

            if lowest_entanglement is not None:
                entanglement = prod(group_a)
                if entanglement > lowest_entanglement:
                    continue

            for group_b in combinations(set(weights) - set(group_a), b):
                # print(a, b, c)
                if sum_a == sum(group_b):
                    if a < smallest_a:
                        smallest_a = a

                    if lowest_entanglement is None:
                        lowest_entanglement = prod(group_a)
                        break
                    elif entanglement < lowest_entanglement:
                        lowest_entanglement = entanglement
                        break

    print("Part 1:", lowest_entanglement)


def part_two():
    numbers = len(weights)
    sum_numbers = sum(weights)
    amounts = [
        (a, b, c, d)
        for a in range(1, numbers)
        for b in range(1, numbers)
        for c in range(1, numbers)
        for d in range(1, numbers)
        if a + b + c + d == len(weights)
    ]
    smallest_a = len(weights)
    lowest_entanglement = None

    for a, b, c, _ in amounts:
        if a > smallest_a:
            continue
        for group_a in combinations(weights, a):
            if a > smallest_a:
                continue

            sum_a = sum(group_a)
            if sum_a * 4 != sum_numbers:
                continue

            if lowest_entanglement is not None:
                entanglement = prod(group_a)
                if entanglement > lowest_entanglement:
                    continue

            for group_b in combinations(set(weights) - set(group_a), b):
                if sum_a != sum(group_b):
                    continue
                if lowest_entanglement is not None and entanglement >= lowest_entanglement:
                    break
                for group_c in combinations((set(weights) - set(group_a)) - set(group_b), c):
                    if sum_a == sum(group_c):
                        if a < smallest_a:
                            smallest_a = a
                        if lowest_entanglement is None:
                            lowest_entanglement = prod(group_a)
                            entanglement = lowest_entanglement
                            break
                        elif entanglement < lowest_entanglement:
                            lowest_entanglement = entanglement
                            break

    print("Part 2:", lowest_entanglement)


part_one()
part_two()
