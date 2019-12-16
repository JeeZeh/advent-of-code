import operator
from intcode import Intcode
from copy import deepcopy
from os import system, name
from time import sleep


p = {1: (0, 1), 2: (0, -1), 3: (-1, 0), 4: (1, 0)}
explored = {(0, 0): "+"}
loc = None
order = []
DRAW_RATE = 0.008


def clear():
    if name == "nt":
        system("cls")
    else:
        system("clear")


def print_ship(pos, trace):
    clear()
    if len(explored) > 0:
        offsetx = min(explored.keys())[0]
        offsety = min(explored.keys(), key=lambda x: x[1])[1]
        maxx = max(explored.keys())[0] + abs(offsetx)
        maxy = max(explored.keys(), key=lambda x: x[1])[1] + abs(offsety)
    else:
        offsetx = 0
        offsety = 0
        maxx = 0
        maxy = 0
    for y in range(maxy, -1, -1):
        row = []
        for x in range(0, maxx + 1):
            pos = (x + offsetx, y + offsety)
            if pos in trace:
                row.append("@")
            elif pos in explored:
                row.append(explored[pos])
            else:
                row.append(" ")

        print("".join(row))


def find_tank(d, pos, data, trace):
    global order, loc

    # Is the requested direction already explored?
    old_pos = pos
    pos = tuple(map(operator.add, pos, p[d]))
    if pos in explored:
        return

    # Load a copy of the bot
    comp = Intcode()
    bot = comp.load(data)

    # Send it in the requested direction
    r = bot.send(d)
    if r == 0:
        explored[pos] = "."
        pos = old_pos
    elif r == 1:
        explored[pos] = " "
        trace.append(pos)
    elif r == 2:
        explored[pos] = "X"
        order = trace
        loc = old_pos
        trace.append(pos)
    for x in p.keys():
        find_tank(x, pos, comp.save(), trace.copy())


def get_oxygen():
    a = Intcode()
    a.init()
    find_tank(2, (0, 0), a.save(), [])
    for i in range(len(order)):
        print_ship(order[i], order[:i])
        print("Steps:", i + 1)
        print("Position:", order[i])
        sleep(DRAW_RATE)


def fill():
    open_area = {k: v for k, v in explored.items() if v == " " or v == "X"}
    open_area[loc] = "O"
    filled = [loc]
    edges = [loc]
    mins = 1

    while len(filled) - 1 < len(open_area) - 1:
        new_edges = []
        for f in edges:
            for d in p.keys():
                spread = tuple(map(operator.add, f, p[d]))
                if spread in open_area and spread not in filled:
                    filled.append(spread)
                    new_edges.append(spread)
        mins += 1
        edges = new_edges
        print_ship(loc, filled)
        print("Mins:", mins)
        sleep(DRAW_RATE)

    return mins


get_oxygen()
sleep(2)
fill()
