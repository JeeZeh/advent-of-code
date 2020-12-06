from itertools import combinations

buckets, MAX = [int(l) for l in open("input.txt")], 150

combs = (combinations(buckets, i) for i in range(len(buckets)))

success = [a for c in combs for a in c if sum(a) == MAX]

print(len(success))
print(sum(map(lambda c: len(c) == len(success[0]), success)))
