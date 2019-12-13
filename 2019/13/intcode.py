from collections import defaultdict


class Intcode:

    p = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1}
    o = None
    mem = None
    ops = None
    ptr = 0
    r = 0
    moves = []

    def init(self, moves, save):
        """
        Init Intcode computer with memory from input text

        Returns initialised Intcode generator
        """

        self.p = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1}
        if save is not None:
            self.mem = save[0]
            self.ptr = save[1]
            self.r = save[2]
            self.moves = save[5]
        else:
            self.o = list(map(int, open("input.txt").readline().split(",")))
            self.mem = defaultdict(int)
            for i in range(len(self.o)):
                self.mem[i] = self.o[i]
            self.ptr = 0
            self.r = 0

        self.moves += moves

        self.ops = self.mem
        x = self.run(save)
        # x.send(None)

        return x

    def get_pos(self, mode, param, param_idx):
        """
        Get a value given a position based on mode
        """

        if mode == 0:
            return param[param_idx]
        elif mode == 1:
            return self.ptr + param_idx + 1
        elif mode == 2:
            return self.r + param[param_idx]

    def write(self, mode, param, param_idx, value):
        """
        Write to memory
        """

        self.ops[self.get_pos(mode, param, param_idx)] = value
        self.ptr += len(param) + 1

    def read(self, mode, param, param_idx):
        """
        Read from memory
        """

        output = self.ops[self.get_pos(mode, param, param_idx)]
        self.ptr += len(param) + 1
        return output

    def run(self, save):
        """
        Intcode CPU
        """

        if save is None:
            self.ops[0] = 2
            output = []
            ins = 0
        else:
            self.ops = save[0]
            self.ptr = save[1]
            self.r = save[2]
            output = save[3]
            ins = save[4]
        save = None

        while self.ops[self.ptr] != 99:
            op = f"{self.ops[self.ptr]:05}"
            code = int(op[-2:])
            e = self.p[code]
            modes = list(map(int, list(op[:-2][::-1][:e])))
            params = [self.ops[self.ptr + i] for i in range(1, e + 1)]
            data = [self.ops[self.get_pos(modes[i], params, i)] for i in range(e)]

            if code == 1:
                self.write(modes[-1], params, e - 1, data[0] + data[1])
            elif code == 2:
                self.write(modes[-1], params, e - 1, data[0] * data[1])
            elif code == 3:
                if len(self.moves) > ins:
                    move = self.moves[ins]
                    ins += 1
                    self.write(modes[-1], params, e - 1, move)
                    if len(self.moves) == ins:
                        save = (
                            self.ops.copy(),
                            self.ptr,
                            self.r,
                            output.copy(),
                            ins,
                            self.moves.copy(),
                        )
                else:
                    move = 0
                    ins += 1
                    self.write(modes[-1], params, e - 1, move)
            elif code == 4:
                rd = self.read(modes[-1], params, e - 1)
                output.append(rd)
            elif code == 5:
                if data[0] != 0:
                    self.ptr = data[1]
                else:
                    self.ptr += e + 1
            elif code == 6:
                if data[0] == 0:
                    self.ptr = data[1]
                else:
                    self.ptr += e + 1
            elif code == 7:
                self.write(modes[-1], params, e - 1, int(data[0] < data[1]))
            elif code == 8:
                self.write(modes[-1], params, e - 1, int(data[0] == data[1]))
            elif code == 9:
                self.r += data[0]
                self.ptr += 2

        return (output, save)
