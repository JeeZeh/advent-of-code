from collections import Counter

from advent.solution import Solution


class Day01(Solution):
    def parse_lists(self, lists: str):
        list_a: list[int] = []
        list_b: list[int] = []

        for line in lists.splitlines():
            a, b = line.split("   ")
            list_a.append(int(a))
            list_b.append(int(b))

        return list_a, list_b

    def run(self, puzzle_input: str):
        a, b = self.parse_lists(puzzle_input)

        # Part 1
        a.sort()
        b.sort()
        difference: int = sum(abs(num_a - b[i]) for i, num_a in enumerate(a))

        # Part 2
        nums_in_b = Counter(b)
        similarity = sum(num * nums_in_b.get(num, 0) for num in a)

        return difference, similarity
