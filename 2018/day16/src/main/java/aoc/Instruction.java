package aoc;

import java.util.Arrays;

import lombok.RequiredArgsConstructor;

@RequiredArgsConstructor
public class Instruction {
    final int opcode;
    final Operands operands;

    static Instruction fromString(String string) {
        final int[] parsed = Arrays.asList(string.split(" ")).stream().mapToInt(s -> Integer.parseInt(s)).toArray();
        return new Instruction(parsed[0], Operands.fromArray(Arrays.copyOfRange(parsed, 1, 4)));
    }
}