inputs = open("input.txt").readlines()

vowels = "aeiou"


def valid(s):
    if any(pair in s for pair in ("ab", "cd", "pq", "xy")):
        return False
    if not sum(s.count(c) for c in vowels) > 2:
        return False
    return any(c * 2 in s for c in set(s))


def valid_2(s):
    pairs = set(s[i : i + 2] for i in range(len(s) - 1))
    if not any(s.count(p) > 1 for p in pairs):
        return False
    return any(s[x+2] == c for c in set(s[:-2]) for x in (i for i, l in enumerate(s[:-2]) if l == c))

print(sum(valid(s) for s in inputs))

print(sum(valid_2(s) for s in inputs))