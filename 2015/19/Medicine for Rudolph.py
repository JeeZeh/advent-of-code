from enum import unique
from os import replace
from typing import List, Match, Set, Tuple
from dataclasses import dataclass
import re


@dataclass
class Replacement:
    match: str
    produce: str


def read_input(test=False) -> Tuple[List[Replacement], str]:
    filename = "test.txt" if test else "input.txt"
    replacements = []

    with open(filename) as in_file:
        _replacements, molecule = in_file.read().split("\n\n")

        for r in _replacements.splitlines():
            replacements.append(Replacement(*r.split(" => ")))

    return replacements, molecule


replacements, molecule = read_input()


def unique_replacements(replacement: Replacement, molecule: str) -> Set[str]:
    molecules = set()
    for match in re.finditer(replacement.match, molecule):
        start, end = match.span()
        molecules.add(molecule[:start] + replacement.produce + molecule[end:])

    return molecules


unique_molecules = set()
for r in replacements:
    unique_molecules |= unique_replacements(r, molecule)

print("Part 1:", len(unique_molecules))


def largest_replacement(replacements, molecule) -> Tuple[Replacement, Match]:
    searches = ((r, re.search(r.produce, molecule)) for r in replacements)
    searches = [m for m in searches if m[1] is not None]
    return (None, None) if not searches else max(searches, key=lambda m: len(m[0].produce))

# Start with the end molecule and try to 'dissolve' it down to 'e'
# rather than trying to generate the molecule by starting from 'e'
reduction = molecule
steps = 0
while reduction != "e":
    replacement, match = largest_replacement(replacements, reduction)

    if match is None:
        continue
    start, end = match.span()
    reduction = reduction[:start] + replacement.match + reduction[end:]
    steps += 1
    
print("Part 2:", steps)