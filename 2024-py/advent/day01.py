import sys
from collections import Counter

list_a = []
list_b = []

for line in sys.stdin.readlines():
    a, b = line.split("   ")
    list_a.append(int(a))
    list_b.append(int(b))

list_a.sort(), list_b.sort()

total = sum(abs(num_a - list_b[i]) for i, num_a in enumerate(list_a))

print(f"Part 1: {total}")

nums_in_b = Counter(list_b)
similarity = sum(num * nums_in_b.get(num, 0) for num in list_a)

print(f"Part 2: {similarity}")
