from advent.solution import Solution


class Day02(Solution):
    def direction(self, a: int, b: int):
        return max(min(b - a, 1), -1)

    def floor_is_safe(self, level: list[int]):
        initial_direction = self.direction(level[0], level[1])
        i = 0
        for i in range(1, len(level)):
            a, b = level[i - 1], level[i]
            if self.direction(a, b) != initial_direction or not (1 <= abs(a - b) <= 3):
                return False

        return True

    def test_floor(self, level: list[int], *, with_removal: bool = False):
        if self.floor_is_safe(level):
            return True

        if with_removal:
            for remove_index in range(len(level)):
                copy = [*level]
                copy.pop(remove_index)
                if self.floor_is_safe(copy):
                    return True

        return False

    def run(self, puzzle_input: str):
        levels: list[list[int]] = [
            list(map(int, line.split())) for line in puzzle_input.splitlines()
        ]

        # Part 1
        safe_levels = sum(
            self.test_floor(level, with_removal=False) for level in levels
        )

        # Part 2
        safe_levels_damp = sum(
            self.test_floor(level, with_removal=True) for level in levels
        )
        return safe_levels, safe_levels_damp
