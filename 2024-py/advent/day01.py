import sys


# This is actually 2017-01
def circular_sum(digits: list[int], sum_distance=1):
    total = 0
    list_size = len(digits)
    for i in range(list_size):
        if digits[i] == digits[(i + sum_distance) % list_size]:
            total += digits[i]

    return total


digits = [int(c) for c in sys.stdin.readline()]

print(f"Part 1: {circular_sum(digits)}")
print(f"Part 2: {circular_sum(digits, sum_distance=len(digits) // 2)}")
