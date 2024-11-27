import io
import runpy
import sys


def plugin(year, day, data: str):
    mod_name = "advent.day{:02d}".format(day)
    sys.modules.pop(mod_name, None)

    old_stdin, old_stdout = sys.stdin, sys.stdout
    sys.stdin, sys.stdout = io.StringIO(data), io.StringIO()
    try:
        runpy.run_module(mod_name, run_name="__main__")
    finally:
        out = sys.stdout
        sys.stdin, sys.stdout = old_stdin, old_stdout

    lines = [x for x in out.getvalue().splitlines() if x]
    if not lines:
        return None, None

    part_a = part_b = None
    for line in lines:
        if line.startswith("Part 1"):
            part_a = line.split(": ")[-1]
        elif line.startswith("Part 2"):
            part_b = line.split(": ")[-1]

    return part_a, part_b
