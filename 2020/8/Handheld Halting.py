from copy import deepcopy


class GameBoy:
    def __init__(self, bootcode) -> None:
        self.rom = bootcode
        self.reset()

    def reset(self):
        self.pointer = 0
        self.accumulator = 0
        self.states = []
        self.exit = None

    def run(self):
        while self.exit is None:
            yield self.rom[self.pointer]
            self.execute(*self.rom[self.pointer])

            if self.pointer in self.states:
                self.exit = 1
            elif self.pointer == len(self.rom) - 1:
                self.exit = 0
            elif self.pointer >= len(self.rom):
                self.exit = 2

    def execute(self, instruction, *args):
        self.states.append(self.pointer)
        getattr(self, f"op_{instruction}")(*args)

    def simulate(self):
        for _ in self.run():
            pass
        return self

    def op_nop(self, *args):
        self.pointer += 1

    def op_acc(self, i):
        self.accumulator += i
        self.pointer += 1

    def op_jmp(self, i):
        self.pointer += i


### Solution ###

bootcode = list(map(lambda x: (x[0], int(x[1])), map(str.split, open("bootcode.txt"))))
gameboy = GameBoy(bootcode)

### Part 1 ###
print(gameboy.simulate().accumulator)

### Part 2 ###
gameboy.reset()
for instruction, inputs in gameboy.run():
    if instruction == "jmp":
        modded_gameboy = deepcopy(gameboy)
        modded_gameboy.rom[modded_gameboy.pointer] = ("nop", inputs)

        if modded_gameboy.simulate().exit == 0:
            print(modded_gameboy.accumulator)
            break
