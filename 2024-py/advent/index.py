from advent.solution import Solution
from advent.solutions.day01 import Day01
from advent.solutions.day02 import Day02
from advent.solutions.day03 import Day03
from advent.solutions.day04 import Day04
from advent.solutions.day05 import Day05
from advent.solutions.day06 import Day06
from advent.solutions.day07 import Day07

index: dict[str, type[Solution]] = {
    ### START SOLUTIONS ###
    "01": Day01,
    "02": Day02,
    "03": Day03,
    "04": Day04,
    "05": Day05,
    "06": Day06,
    "07": Day07,
    ### END SOLUTIONS ###
}
