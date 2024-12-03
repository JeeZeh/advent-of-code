from advent.solution import Solution


class Day02(Solution):
    def direction(self, a, b):
        return max(min(b - a, 1), -1)

    def jump_is_safe(self, a, b, initial_direction):
        if self.direction(a, b) != initial_direction or not (1 <= abs(a - b) <= 3):
            return False

        return True

    def floor_is_safe(self, level: list[int], dampener=0):
        unsafe = 0
        initial_direction = self.direction(level[0], level[1])
        i = 0
        while i < len(level) - 1:
            a, b = level[i], level[i + 1]
            if not self.jump_is_safe(a, b, initial_direction):
                if dampener and unsafe < dampener:
                    try:
                        check_right = level[i + 2]
                        if self.jump_is_safe(a, check_right, initial_direction):
                            unsafe += 1
                            i += 1
                    except:
                        try:
                            check_left = level[i - 1]
                            if self.jump_is_safe(check_left, b, initial_direction):
                                unsafe += 1
                        except:
                            return False
                else:
                    return False
            i += 1
        return True

    def run(self, puzzle_input: str):
        levels: list[list[int]] = [
            list(map(int, line.split())) for line in puzzle_input.splitlines()
        ]

        # Part 1
        safe_levels = sum(self.floor_is_safe(level) for level in levels)

        safe_levels_damp = sum(self.floor_is_safe(level, 1) for level in levels)
        return safe_levels, safe_levels_damp
