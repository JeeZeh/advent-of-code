from intcode import Intcode
import os

tape = list(map(int, open("tape.txt").readline().split(",")))

code = Intcode()
running = code.init(tape)


def drill_down(x, y):
    depth = 0
    try:
        while True:
            running = code.init(tape)
            running.send(x)
            beam = running.send(y)
            if beam == 1:
                depth += 1
                y += 1
            else:
                break
    except StopIteration as si:
        pass
    return depth


def find_start():
    x_offset = 0
    y_offset = 0

    while True:
        row = []
        for x in range(x_offset, 1400):
            running = code.init(tape)
            running.send(x)
            beam = running.send(y_offset)
            if 1 not in row and beam == 1:
                x_offset = x
            if 1 in row and beam == 0:
                break
            row.append(beam)


        check_x = x_offset + len(row) - 101
        print(f"Checking: {check_x}, {y_offset}")
        width, height = verify(check_x, y_offset)
        
        if height < 100:
            print(f"Offsetting Y by: {max((100 - height)//2, 1)}\n")
            y_offset += min((100 - height)//2, 99)
        elif height == 100 and width == 100:
            print("Reached 100x100!")
            print(f"X: {check_x}\nY: {y_offset}")
            return (check_x, y_offset)
        else:
            print(f"Offsetting Y by: 1")
            y_offset += 1
        
    
       
        


def verify(x, y):
    length, height = 0, 0
    try:
        cx, cy = x, y
        running = code.init(tape)
        running.send(cx)
        beam = running.send(cy)
        # print(f"Starts with: {beam}")

        cx, cy = x, y
        running = code.init(tape)
        running.send(cx)
        beam = running.send(cy)
        # Check left
        while beam == 1:
            length += 1
            cx += 1
            running = code.init(tape)
            running.send(cx)
            beam = running.send(cy)

        # print(f"X: {length}")

        cx, cy = x, y
        running = code.init(tape)
        running.send(cx)
        beam = running.send(cy)
        # Check left
        while beam == 1:
            height += 1
            cy += 1
            running = code.init(tape)
            running.send(cx)
            beam = running.send(cy)

        # print(f"Y: {height}")
    except:
        pass

    return (length, height) 


x, y = find_start()

verify(x, y)

print((x * 10000) + y)

# x, y = find_start()

# for i in range(y-10, 1000):
#     row = []
#     for j in range(x-100, 1400):
#         running = code.init(tape)
#         running.send(j)
#         beam = running.send(i)
#         if x <= j and y <= i and beam == 1 :
#             beam = "O"
#         row.append("#" if beam == 1 else " " if beam != "O" else beam)
#         if not row and beam == 1:
#             x_start = x
#         if 1 in row and beam == 0:
#             break
#     print("".join(row))


# row = []
# for i, a in enumerate(areas):
#     if i % DIM == 0:
#         print("".join(row))
#         row = []

#     if a == 0:
#         row.append(".")
#     else:
#         row.append("#")

