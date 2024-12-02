from advent.solution import Solution


class Day02(Solution):
    def direction(self, a, b):
        return max(min(b - a, 1), -1)

    def is_safe(self, level: list[int]):
        initial_direction = self.direction(level[0], level[1])
        for i in range(1, len(level)):
            if self.direction(level[i - 1], level[i]) != initial_direction or not (
                1 <= abs(level[i - 1] - level[i]) <= 3
            ):
                return False

        return True

    def run(self, puzzle_input: str):
        levels: list[list[int]] = [
            list(map(int, line.split())) for line in puzzle_input.splitlines()
        ]

        # Part 1
        safe_levels = sum(self.is_safe(level) for level in levels)

        return safe_levels, None
