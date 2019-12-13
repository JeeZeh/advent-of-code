from intcode import Intcode
from pprint import pprint

"""
This solution works not by reacting in time to the ball's position,
but by starting with an initial step, simulating until failure, and
adding the steps to prevent this from happening next run.

This version of the intcode computer will return the save state of 
the machine at the last inserted step so that we only need to calculate
steps and simulate from that point - save scumming.
"""


def play(moves, save):
    mem = Intcode().init(moves, save)
    x = mem[0]
    d = [[x[i], x[i + 1], x[i + 2]] for i in range(0, len(x), 3)]
    balls = [t[:2] for t in d if t[2] == 4]
    pad = [t[:2] for t in d if t[2] == 3]
    score = [t for t in d if t[:2] == [-1, 0]]
    blocks = len([t[:2] for t in d if t[2] == 2])
    return (pad, balls, score, blocks, mem[1])


def un_fail(ball, pads, calculated):
    moves = []
    pad = pads[-1][0]
    fail_point = list(map(lambda x: x[1], ball)).index(19) - 2

    for p in ball[calculated - 1 : fail_point + 1]:
        if p[0] > pad:
            pad += 1
            moves.append(1)
        elif p[0] < pad:
            pad -= 1
            moves.append(-1)

    return moves


output = play([0, -1, 1], None)
while output[4] is not None:
    moves = un_fail(output[1], output[0], output[4][4])
    output = play(moves, output[4])

print(f"Final Score: {output[2][-1][-1]}")
