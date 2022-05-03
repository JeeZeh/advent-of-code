package aoc;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import lombok.RequiredArgsConstructor;

/**
 * Java, please add tuples...
 */
@RequiredArgsConstructor
class TestResult {
  final int opcode;
  final List<String> matching;
}

@RequiredArgsConstructor
public class TestCase {
  final int opcode;
  final int[] initialRegister;
  final Operands input;
  final int[] expectedRegister;

  public static TestCase fromString(String string) {
    final String[] parts = string.split("\n");
    final String before = parts[0].split(": ")[1].trim();
    final String input = parts[1].trim();
    final String after = parts[2].split(":  ")[1].trim();

    final int[] beforeInts = Arrays.asList(before.substring(1, before.length() - 1).split(", ")).stream()
        .mapToInt(e -> Integer.parseInt(e)).toArray();
    final int[] inputInts = Arrays.asList(input.split(" ")).stream().mapToInt(e -> Integer.parseInt(e)).toArray();
    final int[] afterInts = Arrays.asList(after.substring(1, after.length() - 1).split(", ")).stream()
        .mapToInt(e -> Integer.parseInt(e)).toArray();

    return new TestCase(
        inputInts[0],
        beforeInts,
        new Operands(inputInts[1], inputInts[2], inputInts[3]),
        afterInts);
  }

  /**
   * Running a test case involves setting up a virtual machine, setting its
   * registers to the {@code initialRegister} state, applying all known operations
   * in the VM, and collecting any operations that produce a matching
   * {@code expectedRegister} state.
   */
  public TestResult run() {
    final VirtualMachine testVm = new VirtualMachine();
    final List<String> matchingOperations = new ArrayList<>();
    for (final String operation : testVm.operations.keySet()) {
      testVm.setRegisters(this.initialRegister);
      testVm.applyOperation(operation, this.input);
      if (Arrays.equals(testVm.reg, this.expectedRegister)) {
        matchingOperations.add(operation);
      }
    }
    return new TestResult(this.opcode, matchingOperations);
  }
}