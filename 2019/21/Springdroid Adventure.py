from intcode import Intcode, IntcodeStatus

spring_rom = list(map(int, open("tape.txt").readline().split(",")))


def part_one():
    droid = Intcode(spring_rom)

    if droid.run() == IntcodeStatus.WAITING:
        program = """NOT C J
        NOT A T
        AND D J
        OR T J
        WALK
        """

        droid.send(program)

    print("Part 1:", droid.drain_output(for_humans=True).splitlines()[-1])


def part_two():
    droid = Intcode(spring_rom)

    if droid.run() == IntcodeStatus.WAITING:
        program = """NOT B T
        NOT C J
        AND H J
        OR T J
        NOT A T
        OR T J
        AND D J
        RUN
        """

        droid.send(program)

    result = droid.drain_output(for_humans=True)

    if len(result.splitlines()) > 10:
        print(result)
    else:
        print("Part 2:", result.splitlines()[-1])


part_one()
part_two()
