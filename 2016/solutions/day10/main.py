from collections import defaultdict


class Holds:
    slot_a: int = None
    slot_b: int = None

    def give(self, x: int):
        if not self.slot_a:
            slot_a = x
        elif not self.slot_b:
            self.slot_b = x
        else:
            raise ValueError(f"Tried to give {x} but {self} already holds 2 values")


class Output(Holds):
    def give(self, x: int):
        if not self.slot_a:
            self.slot_a = x
        else:
            raise ValueError("Output already holds a value")


class Bot(Holds):
    low: Holds
    high: Holds


def read_rules():
    hardcoded: dict[int, int] = {}
    bots: dict[int, Bot] = defaultdict(Bot)
    outputs: dict[int, Output] = defaultdict(Output)

    for line in map(str.strip, open("test.txt")):
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


def main():
    hardcoded, bots, outputs = read_rules()
    
    print(hardcoded, bots, outputs)


if __name__ == "__main__":
    main()
