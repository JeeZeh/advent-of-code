from math import sqrt


def get_gift_count(house: int):
    s = 0
    for i in range(1, int(sqrt(house)) + 1):
        if house % i == 0:
            s += i
            if (d := house // i) != i:
                s += d

    return s * 10


def get_gift_count_v2(house: int):
    s = 0
    for i in range(1, int(sqrt(house)) + 1):
        if house % i == 0:
            if house / i <= 50:
                s += i
            d = house // i
            if d != i and house / d <= 50:
                s += d

    return s * 11


def find_target(target):
    current = 1
    while get_gift_count(current) < target:
        current += 1

    return current


def find_target_v2(start, target):
    current = start
    while (c := get_gift_count_v2(current)) < target:
        if current % 10000 == 0:
            print(current, c)
        current += 1

    return current


target = 36_000_000
part_one = find_target(target)
print("Part 1:", find_target(target))
print("Part 2:", find_target_v2(target))
