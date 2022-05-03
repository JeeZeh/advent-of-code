package aoc;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Solution {
  public static void main(String[] args) throws IOException {
    final String input = getResourceFileAsString("aoc/input");
    final String[] parts = input.split("(\\r?\\n){4}");
    final String programInputString = parts[1];

    final List<TestResult> testInputResults = runTestInputs(parts[0]);
    System.out.println(String.format("Part 1: %d", partOne(testInputResults)));
    System.out.println(String.format("Part 2: %d", partTwo(testInputResults, programInputString)));
  }

  /**
   * Number of test cases which matched 3 or more operations from the given
   * opcode.
   */
  static long partOne(List<TestResult> results) {
    return results.stream()
        .filter(r -> r.matching.size() >= 3)
        .count();
  }

  /**
   * The value in register 0 after deducing all opcode operations and running the
   * program input against the VM.
   */
  static long partTwo(List<TestResult> testResults, String programInput) {
    final Map<Integer, String> opcodeTranslation = deduceOpcodes(testResults);

    final VirtualMachine vm = new VirtualMachine();
    programInput.lines()
        .map(Instruction::fromString)
        .forEach(i -> vm.applyOperation(opcodeTranslation.get(i.opcode), i.operands));

    return vm.reg[0];
  }

  /**
   * Very simple hunting algorithm.
   * 
   * Until we've found all 16 opcodes and their corresponding operation:
   * 1. Find any test case which only matches a single operation after filtering
   * out any matching operations that we already know about
   * 2. Assign the operation to that test case's opcode
   */
  static Map<Integer, String> deduceOpcodes(List<TestResult> testResults) {
    final Set<String> known = new HashSet<>();
    final Map<Integer, String> mappedOpcodes = new HashMap<>();

    while (mappedOpcodes.size() < 16) {
      for (final TestResult result : testResults) {
        final List<String> toConsider = result.matching.stream().filter(r -> !known.contains(r)).toList();
        if (toConsider.size() == 1) {
          final String operation = toConsider.get(0);
          mappedOpcodes.put(result.opcode, operation);
          known.add(operation);

          // If we found a matching code, let's go back to the beginning and scan again
          // until we find another match
          break;
        }
      }
    }

    return mappedOpcodes;
  }

  static List<TestResult> runTestInputs(String testInputString) {
    return Stream.of(testInputString.split("(\\r?\\n){2}"))
        .map(TestCase::fromString)
        .map(TestCase::run)
        .toList();
  }

  /**
   * Reads given resource file as a string.
   *
   * @param fileName path to the resource file
   * @return the file's contents
   * @throws IOException if read fails for any reason
   */
  static String getResourceFileAsString(String fileName) throws IOException {
    ClassLoader classLoader = ClassLoader.getSystemClassLoader();
    try (InputStream is = classLoader.getResourceAsStream(fileName)) {
      if (is == null)
        return null;
      try (InputStreamReader isr = new InputStreamReader(is, StandardCharsets.UTF_8);
          BufferedReader reader = new BufferedReader(isr)) {
        return reader.lines().collect(Collectors.joining(System.lineSeparator()));
      }
    }
  }
}
