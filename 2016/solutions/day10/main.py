from collections import defaultdict

monitoring: tuple[int, int] = None


class Holds:
    slot_a: int = None
    slot_b: int = None

    def can_give(self):
        return self.slot_a is None or self.slot_b is None

    def give(self, x: int):
        if not self.can_give():
            return False

        if self.slot_a is None:
            self.slot_a = x
        else:
            self.slot_b = x

        return True


class Output(Holds):
    def can_give(self):
        return self.slot_a is None

    def give(self, x: int):
        if not self.can_give():
            return False

        self.slot_a = x
        return True


class Bot(Holds):
    low: Holds
    high: Holds

    def process(self) -> bool:
        if self.slot_a is not None and self.slot_b is not None:
            self.low.give(min(self.slot_a, self.slot_b))
            self.high.give(max(self.slot_a, self.slot_b))

            self.slot_a = None
            self.slot_b = None

            return True

        return False


def read_rules(filename: str):
    hardcoded: dict[int, int] = {}
    bots: dict[int, Bot] = defaultdict(Bot)
    outputs: dict[int, Output] = defaultdict(Output)

    for line in map(str.strip, open(filename)):
        if line.startswith("value"):
            value, bot = map(int, line[6:].split(" goes to bot "))
            hardcoded[value] = bot
        else:
            low_parts, high_parts = map(str.split, line.split(" and "))
            bot_source, low_dest, high_dest = map(int, (low_parts[1], low_parts[-1], high_parts[-1]))
            low_is_bot, high_is_bot = low_parts[-2] == "bot", high_parts[-2] == "bot"

            bot = bots[bot_source]
            bot.low = bots[low_dest] if low_is_bot else outputs[low_dest]
            bot.high = bots[high_dest] if high_is_bot else outputs[high_dest]

    return hardcoded, bots, outputs


def main(filename: str, look_for_comparison: tuple[int, int]):
    global monitoring
    monitoring = look_for_comparison

    hardcoded, bots, outputs = read_rules(filename)
    for value, destination in hardcoded.items():
        dest_bot = bots[destination]
        if dest_bot.can_give():
            dest_bot.give(value)
        else:
            dest_bot.process()

    active = True
    while active:
        active = False
        for id_, bot in bots.items():
            if bot.slot_a in monitoring and bot.slot_b in monitoring:
                print(f"Part 1: {id_}")
            active |= bot.process()

    mult_result = outputs[0].slot_a * outputs[1].slot_a * outputs[2].slot_a
    print(f"Part 2: {mult_result}")


if __name__ == "__main__":
    main("input.txt", (61, 17))
