from collections import defaultdict
from dataclasses import dataclass, field
from typing import Union
from enum import Enum

from black import out


class IntcodeStatus(Enum):
    STOPPED = "STOPPED"
    WAITING = "WAITING_FOR_INPUT"


@dataclass
class IntcodeState:
    tape: list[int] = None
    mem: dict[int] = None
    ops: dict[int] = None
    ptr: int = 0
    rel: int = 0
    waiting: bool = False
    input_buffer: list[int] = field(default_factory=list)
    output_buffer: list[int] = field(default_factory=list)

    def save(self):
        return IntcodeState(
            self.tape[:],
            self.ops.copy(),
            self.mem.copy(),
            self.ptr,
            self.rel,
            self.waiting,
            self.input_buffer[:],
            self.output_buffer[:],
        )

    def load(self, state):
        self.tape = state.tape[:]
        self.ops = state.ops.copy()
        self.mem = state.mem.copy()
        self.ptr = state.ptr
        self.rel = state.rel
        self.waiting = state.waiting
        self.input_buffer = state.input_buffer[:]
        self.output_buffer = state.output_buffer[:]


class Intcode:
    operand_count: dict[int, int] = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1}

    def __init__(self, tape, input_buffer=None):
        """
        Init Intcode computer with memory from input text

        Returns initialised Intcode generator
        """

        self.state = IntcodeState(tape=tape, mem=defaultdict(int), ptr=0, rel=0)

        for i, data in enumerate(self.state.tape):
            self.state.mem[i] = data

        self.state.ops = self.state.mem.copy()

        if input_buffer:
            self.state.input_buffer = input_buffer

    def get_pos(self, mode, param, param_idx):
        """
        Get a value given a position based on mode
        """

        if mode == 0:
            return param[param_idx]
        elif mode == 1:
            return self.state.ptr + param_idx + 1
        elif mode == 2:
            return self.state.rel + param[param_idx]

    def write(self, mode, param, param_idx, value):
        """
        Write to memory
        """

        self.state.ops[self.get_pos(mode, param, param_idx)] = value
        self.state.ptr += len(param) + 1

    def read(self, mode, param, param_idx):
        """
        Read from memory
        """

        output = self.state.ops[self.get_pos(mode, param, param_idx)]
        self.state.ptr += len(param) + 1
        return output

    def send(self, input: str):
        if self.state.input_buffer:
            raise ValueError("Input buffer not empty, not sending!")

        self.state.input_buffer = list(map(ord, input))

    @staticmethod
    def decode(char):
        try:
            return chr(char)
        except ValueError:
            return str(char)

    def drain_output(self, for_humans=True) -> Union[str, list]:
        if for_humans:
            output = "".join(map(self.decode, self.state.output_buffer))
        else:
            output = self.state.output_buffer[:]

        self.state.output_buffer = []
        return output

    def repl(self):
        state = self.run()

        while state != IntcodeStatus.STOPPED:
            if state == IntcodeStatus.WAITING:
                print(self.drain_output(for_humans=True))
                self.send(input() + "\n")
                state = self.run()
                
        print(self.drain_output(for_humans=True))


    def run(self) -> IntcodeStatus:
        """
        Intcode CPU
        """

        while self.state.ops[self.state.ptr] != 99:
            op = f"{self.state.ops[self.state.ptr]:05}"
            code = int(op[-2:])
            operand_count = self.operand_count[code]
            modes = list(map(int, list(op[:-2][::-1][:operand_count])))
            params = [self.state.ops[self.state.ptr + i] for i in range(1, operand_count + 1)]
            operand_values = [self.state.ops[self.get_pos(modes[i], params, i)] for i in range(operand_count)]

            if code == 1:
                self.write(modes[-1], params, operand_count - 1, operand_values[0] + operand_values[1])
            elif code == 2:
                self.write(modes[-1], params, operand_count - 1, operand_values[0] * operand_values[1])
            elif code == 3:  # Input
                if self.state.input_buffer:
                    i = self.state.input_buffer.pop(0)
                    self.write(modes[-1], params, operand_count - 1, i)
                    self.state.waiting = False
                else:
                    self.state.waiting = True
                    break
            elif code == 4:
                rd = self.read(modes[-1], params, operand_count - 1)
                self.state.output_buffer.append(rd)
            elif code == 5:
                if operand_values[0] != 0:
                    self.state.ptr = operand_values[1]
                else:
                    self.state.ptr += operand_count + 1
            elif code == 6:
                if operand_values[0] == 0:
                    self.state.ptr = operand_values[1]
                else:
                    self.state.ptr += operand_count + 1
            elif code == 7:
                self.write(modes[-1], params, operand_count - 1, int(operand_values[0] < operand_values[1]))
            elif code == 8:
                self.write(modes[-1], params, operand_count - 1, int(operand_values[0] == operand_values[1]))
            elif code == 9:
                self.state.rel += operand_values[0]
                self.state.ptr += 2

        if self.state.ops[self.state.ptr] == 99:
            return IntcodeStatus.STOPPED
        elif self.state.waiting:
            return IntcodeStatus.WAITING
