numbers = list(map(int, open("input.txt").readlines()))
numset = set(numbers)


def sumset(s, n):
    if s - n in numset:
        return n, s - n


def get_subsum(s):
    for n in numbers:
        if sol := sumset(s, n):
            return sol[0] * sol[1]


print(get_subsum(2020))

for n in numbers:
    if sol := get_subsum(2020 - n):
        print(sol * n)
        break


# sumset = {sum(c): (c[0], c[1]) for c in combinations(numbers, r=2)}
# print(sumset[2020][0] * sumset[2020][1])
