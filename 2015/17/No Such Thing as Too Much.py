from itertools import combinations
from collections import Counter

buckets, MAX = [int(l) for l in open("input.txt")], 150
combs = [combinations(buckets, i) for i in range(len(buckets))]

success = [a for c in combs for a in c if sum(a) == MAX]

print(len(success))
print(len(list(filter(lambda c: len(c) == len(success[0]), success))))
