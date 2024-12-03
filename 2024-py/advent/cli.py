import argparse
import logging
from pathlib import Path

import advent
from advent.index import index

log = logging.getLogger("cli")


def cli():
    parser = argparse.ArgumentParser()
    parser.add_argument("--new", type=int)
    args = parser.parse_args()

    if args.new:
        solutions = Path(advent.solutions.__path__[0])  # type: ignore
        day_name = f"day{args.new:02}"
        solution_file = (solutions / day_name).with_suffix(".py")
        if solution_file.exists():
            msg = f"{solution_file.name} already exists"
            raise FileExistsError(msg)

        template = solutions / "__template__"
        content = template.read_text().format(
            class_name=day_name.capitalize(), day=args.new, year="2024"
        )
        solution_file.write_text(content)

        # Update index
        index_file = Path(advent.__path__[0]) / "index.py"  # type: ignore
        index_lines = index_file.read_text().splitlines(keepends=True)

        # Add the solution to the imports (first empty line)
        import_line = next(i for i, line in enumerate(index_lines) if not line.strip())
        index_lines.insert(
            import_line,
            f"from advent.solutions.{day_name} import {day_name.capitalize()}\n",
        )

        # Add the solution to the index dictionary (insert before indicator)
        entry_line = next(
            i for i, line in enumerate(index_lines) if "### END SOLUTIONS ###" in line
        )
        index_lines.insert(
            entry_line, f'    "{args.new:02d}": {day_name.capitalize()},\n'
        )
        index_file.open(mode="w+").writelines(index_lines)


def solve(year: str, day: str, data: str):  # noqa: ARG001
    try:
        solution_ref = index[f"{day:02d}"]
        solution = solution_ref()
        return solution.run(data)
    except KeyError:
        log.exception("Could not find solution for day")
        return None
