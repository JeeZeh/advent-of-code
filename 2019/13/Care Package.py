from intcode import Intcode
from pprint import pprint

"""
This solution works not by reacting in time to the ball's position,
but by starting with an initial step, simulating until failure, and
adding the steps to prevent this from happening next run.

This version of the intcode computer will return the save state of 
the machine at the last inserted step so that we only need to calculate
steps and simulate from that point - save scumming.

P.S. - It's slow
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


def un_fail(ball_x, ball_y, pad, calculated):
    moves = []
    fail_point = ball_y.index(19) if 19 in ball_y else 0

    for p in ball_x[calculated - 1 : fail_point - 1]:
        if p > pad:
            pad += 1
            moves.append(1)
        elif p < pad:
            pad -= 1
            moves.append(-1)

    return moves


output = play([0, -1, 1], None)
while output[4] is not None:
    moves = un_fail(
        list(map(lambda x: x[0], output[1])),
        list(map(lambda x: x[1], output[1])),
        output[0][-1][0],
        output[4][4],
    )
    output = play(moves, output[4])

print(f"Final Score: {output[2][-1][-1]}")
