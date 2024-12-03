from advent.solution import Solution
from advent.solutions.day01 import Day01
from advent.solutions.day02 import Day02
from advent.solutions.day03 import Day03

index: dict[str, type[Solution]] = {
    ### START SOLUTIONS ###
    "01": Day01,
    "02": Day02,
    "03": Day03,
    ### END SOLUTIONS ###
}
