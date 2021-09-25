from collections import defaultdict
from dataclasses import dataclass
from typing import Tuple


@dataclass
class Reindeer:
    speed: int
    stamina: int
    rest: int

    def get_travelled(self, seconds: int) -> int:
        # Don't step through, calculate the num of full run/rest cycles
        # and then check the remaining time to see how long it can run.
        # Total time spent running * speed is the distance at a given time.
        cycle_length = self.stamina + self.rest
        cycles = seconds // cycle_length

        moving_time = cycles * self.stamina
        moving_time += min(seconds - (cycle_length * cycles), self.stamina)

        return moving_time * self.speed


def parse(line: str) -> Tuple[str, Reindeer]:
    parts = line.split()
    name = parts[0]
    speed = int(parts[3])
    stamina = int(parts[6])
    rest = int(parts[13])

    return name, Reindeer(speed, stamina, rest)


squad = {k: v for k, v in map(parse, open("input.txt"))}


def race(squad: dict[str, Reindeer], time: int):
    return list(map(lambda r: (r[0], r[1].get_travelled(time)), squad.items()))


def king_of_the_hill(squad: dict[str, Reindeer], time: int):
    score = defaultdict(int)

    for i in range(1, time + 1):
        positions = race(squad, i)
        lead = max(positions, key=lambda r: r[1])[1]

        for p in positions:
            if p[1] == lead:
                score[p[0]] += 1

    return score


race_length = 2503

print(
    "Part 1: ",
    max(race(squad, race_length), key=lambda r: r[1]),
)
print(
    "Part 2: ",
    max(king_of_the_hill(squad, race_length).items(), key=lambda kv: kv[1]),
)
