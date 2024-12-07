from dataclasses import dataclass
from enum import Enum
from itertools import product

from aocd import get_puzzle

from advent.solution import Solution


class Operation(Enum):
    ADD = (0,)
    MULT = (1,)
    CONCAT = (2,)


@dataclass
class Equation:
    calibration: int
    operands: tuple[int, ...]

    def __hash__(self) -> int:
        return (self.calibration, self.operands).__hash__()

    def evaluate(self, operations: list[Operation] | tuple[Operation, ...]):
        value = self.operands[0]
        for i in range(1, len(self.operands)):
            match operations[i - 1]:
                case Operation.ADD:
                    value += self.operands[i]
                case Operation.MULT:
                    value *= self.operands[i]
                case Operation.CONCAT:
                    value = int(f"{value}{self.operands[i]}")

            if value > self.calibration:
                return False

            # Idea: greedy-add remaining operands to see if the sum is too large?
        return value == self.calibration


class Day07(Solution):
    def parse(self, puzzle_input: str):
        equations: list[Equation] = []
        for line in puzzle_input.splitlines():
            calibration, operands = line.split(": ")
            equations.append(Equation(int(calibration), tuple(map(int, operands.split()))))

        return equations

    def validate_equations(self, equations: set[Equation], operations: tuple[Operation, ...]):
        for equation in equations:
            for combination in product(operations, repeat=len(equation.operands) - 1):
                if equation.evaluate(combination):
                    yield equation
                    break

    def run(self, puzzle_input: str):
        equations = set(self.parse(puzzle_input))

        # Idea: lazy evaluate and bail once equation becomes too large
        # Idea: dynamic programming to memoize when operand combinations have already been tried?
        valid_with_add_mult = set(self.validate_equations(equations, (Operation.ADD, Operation.MULT)))
        part_one = sum(eq.calibration for eq in valid_with_add_mult)
        valid_with_all = set(self.validate_equations(equations - valid_with_add_mult, tuple(Operation)))
        part_two = part_one + sum(eq.calibration for eq in valid_with_all)

        return part_one, part_two


if __name__ == "__main__":
    print(Day07().run(get_puzzle(day=7, year=2024).examples[0].input_data))
