package aoc;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import lombok.AllArgsConstructor;

@AllArgsConstructor
public class TestCase {
  int opcode;
  int[] initialRegister;
  Operands input;
  int[] expectedRegister;

  public static TestCase fromString(String string) {
    String[] parts = string.split("\n");
    String before = parts[0].split(": ")[1].trim();
    String input = parts[1].trim();
    String after = parts[2].split(":  ")[1].trim();

    int[] beforeInts = Arrays.asList(before.substring(1, before.length() - 1).split(", ")).stream()
        .mapToInt(e -> Integer.parseInt(e)).toArray();
    int[] inputInts = Arrays.asList(input.split(" ")).stream().mapToInt(e -> Integer.parseInt(e)).toArray();
    int[] afterInts = Arrays.asList(after.substring(1, after.length() - 1).split(", ")).stream()
        .mapToInt(e -> Integer.parseInt(e)).toArray();

    return new TestCase(inputInts[0], beforeInts, new Operands(inputInts[1], inputInts[2], inputInts[3]),
        afterInts);
  }

  public List<String> findMatches() {
    CPU testCPU = new CPU();
    List<String> matchingOperations = new ArrayList<>();
    for (final String operation : testCPU.operations.keySet()) {
      testCPU.setRegisters(this.initialRegister);
      testCPU.applyOperation(operation, this.input);
      if (Arrays.equals(testCPU.reg, this.expectedRegister)) {
        matchingOperations.add(operation);
      }
    }
    return matchingOperations;
  }
}