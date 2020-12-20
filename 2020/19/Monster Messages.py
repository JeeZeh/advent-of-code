from itertools import combinations, islice
import sys
from pprint import pprint

rules, messages = open("real.txt").read().split("\n\n")

mem = {}

for rule in rules.splitlines():
    parts = rule.split(": ")
    num = parts[0]
    # Hard code recursion to 10-depth because why not
    if num == "8":
        parts[1] = " | ".join(" ".join(["42"] * i) for i in range(1, 11))
    elif num == "11":
        parts[1] = " | ".join(
            " ".join((["42"] * i)) + " " + " ".join((["31"] * i)) for i in range(1, 11)
        )
    ref = ""
    if '"' in parts[1]:
        ref = parts[1][1]
    else:
        refs = parts[1].split(" | ")
        ref = list(map(lambda x: tuple(x.split()), refs))

    mem[num] = ref


def generate_choices(rule, rec=0):
    solutions = []

    if isinstance(mem[rule], str):
        return mem[rule]
    else:
        for req in mem[rule]:
            c = []
            for r in req:
                if rule == r:
                    c.append(r)
                else:
                    c.append(generate_choices(r, rec))

            solutions.append(tuple(c))

    return solutions


def try_match(phrase, rule):
    if phrase == "":
        return ""
    if isinstance(rule, tuple):
        copy = phrase
        for r in rule:
            new_phrase = try_match(phrase, r)
            if phrase != "" and phrase == new_phrase:
                return copy
            if new_phrase == "" and r != rule[-1]:
                return copy
            phrase = new_phrase
        return phrase
    if isinstance(rule, list):
        copy = phrase
        for r in rule:
            new_phrase = try_match(phrase, r)
            if new_phrase != phrase:
                return new_phrase
        return copy
    if isinstance(rule, str):
        if phrase[0] == rule:
            return phrase[1:]
        return phrase


def is_valid(phrase, rule):
    return not try_match(phrase, rule)


rule = generate_choices("0")[0]
print(sum(is_valid(m, rule) for m in messages.splitlines()))
