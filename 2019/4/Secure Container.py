from collections import Counter

x, MAX = 356261, 846303
passwords = []

def fix_x(x):
    x = [int(d) for d in str(x)]
    for i in range(len(x) - 1):
        if int(x[i]) > int(x[i + 1]):
            x[i + 1] = x[i]
    return int("".join(map(str, x)))

while x <= MAX:
    x = fix_x(x)
    if not x > MAX and 2 in Counter([int(d) for d in str(x)]).values():
        passwords.append(x)
    x += 1

print(len(passwords))
