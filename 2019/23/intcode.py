from collections import defaultdict
from enum import Enum

class IntcodeState(Enum):
    STOPPED: 0
    WAITING_FOR_INPUT: 1
    WAITING_FOR_OUTPUT: 2
class Intcode:

    operand_count = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1}
    o = None
    mem = None
    ops = None
    ptr = 0
    r = 0
    waiting = False
    sending = False
    input_buffer = []
    output_buffer = []

    

    def init(self, tape, queue=None):
        """
        Init Intcode computer with memory from input text

        Returns initialised Intcode generator
        """

        self.o = tape
        self.mem = defaultdict(int)
        self.ptr = 0
        self.r = 0

        for i, data in enumerate(self.o):
            self.mem[i] = data

        self.ops = self.mem

        if queue:
            self.input_buffer = queue

        x = self.run()
        x.send(None)

        return x

    def save(self):
         return (self.o.copy(), self.ops.copy(), self.ptr, self.r, self.input_buffer,self.output_buffer)
        
    def is_waiting(self):
        return self.waiting
    
    def is_sending(self):
        return self.sending
    
    def load(self, data):
        self.o = data[0]
        self.ops = data[1]
        self.ptr = data[2]
        self.r = data[3]
        self.input_buffer = data[4]
        self.output_buffer = data[5]

        x = self.run()
        x.send(None)
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

    def run(self):
        """
        Intcode CPU
        """

        while self.ops[self.ptr] != 99 or not self.is_waiting or not self.output_buffer:
            op = f"{self.ops[self.ptr]:05}"
            code = int(op[-2:])
            e = self.operand_count[code]
            modes = list(map(int, list(op[:-2][::-1][:e])))
            params = [self.ops[self.ptr + i] for i in range(1, e + 1)]
            data = [self.ops[self.get_pos(modes[i], params, i)] for i in range(e)]

            if code == 1:
                self.write(modes[-1], params, e - 1, data[0] + data[1])
            elif code == 2:
                self.write(modes[-1], params, e - 1, data[0] * data[1])
            elif code == 3:
                if self.input_buffer:
                    i = self.input_buffer.pop()
                    self.write(modes[-1], params, e - 1, i)
                    self.waiting = False
                else:
                    self.is_waiting = True
            elif code == 4:
                self.sending = True
                rd = self.read(modes[-1], params, e - 1)
                self.output_buffer.append(rd)
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
                
        if self.ops[self.ptr] != 99:
            return IntcodeState.STOPPED
        if self.is_waiting:
            return IntcodeState.WAITING_FOR_INPUT
        if self.output_buffer:
            return IntcodeState.WAITING_FOR_OUTPUT
