passwords = [map(str.strip, line.split(":")) for line in open("input.txt").readlines()]


def valid(mn, mx, c, p):
    return mn <= p.count(c) <= mx


def valid_2(p1, p2, c, p):
    p1 -= 1
    p2 -= 1

    return (p[p1] == c) ^ (p[p2] == c)


v1_count = 0
v2_count = 0

for rule, password in passwords:
    min_, max_ = map(int, rule[:-2].split("-"))
    char = rule[-1]

    if valid(min_, max_, char, password):
        v1_count += 1

    if valid_2(min_, max_, char, password):
        v2_count += 1

print(v1_count)
print(v2_count)

codes = [line.split() for line in open("input.txt").readlines()]
print(sum([eval(c[0].replace("-", "<=c[2].count(c[1][0])<=")) for c in codes]))
