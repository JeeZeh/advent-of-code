package aoc;

import lombok.Data;

@Data
public class Operands {
    final int a;
    final int b;
    final int c;

    public static Operands fromArray(int[] array) {
        return new Operands(array[0], array[1], array[2]);
    }
}