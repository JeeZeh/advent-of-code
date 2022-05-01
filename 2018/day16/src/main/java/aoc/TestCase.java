package aoc;

import java.util.Arrays;

import lombok.RequiredArgsConstructor;

@RequiredArgsConstructor
public class TestCase {
    int opcode;
    int[] initialRegister;
    Operands input;
    int[] expectedRegister;

    public static TestCase fromString(String string) {
        String[] parts = string.split("\n");
        String before = parts[0].split(": ")[1];
        String input = parts[1];
        String after = parts[2].split(": ")[1];

        int[] beforeInts = Arrays.asList(before.substring(1, before.length() - 1).split(", ")).stream()
                .mapToInt(e -> Integer.parseInt(e)).toArray();
        int[] inputInts = Arrays.asList(input.split(" ")).stream().mapToInt(e -> Integer.parseInt(e)).toArray();
        int[] afterInts = Arrays.asList(after.substring(1, after.length() - 1).split(", ")).stream()
                .mapToInt(e -> Integer.parseInt(e)).toArray();

        return new TestCase(inputInts[0], beforeInts, new Operands(inputInts[1], inputInts[2], inputInts[3]),
                afterInts);
    }
}