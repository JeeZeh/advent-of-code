package aoc;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.Map.Entry;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import lombok.AllArgsConstructor;
import lombok.Data;

@AllArgsConstructor
class TestResult {
  int opcode;
  List<String> matching;
}

/**
 * Hello world!
 *
 */
public class Solution {
  public static void main(String[] args) throws IOException {
    String input = getResourceFileAsString("aoc/input");
    String[] parts = input.split("(\\r?\\n){4}");
    String tests = parts[0];
    String program = parts[1];

    List<TestResult> mappings = findInstructions(tests);
    System.out.println(String.format("Part 1: %d", partOne(mappings)));
  }

  static long partOne(List<TestResult> testResults) {
    return testResults.stream().filter(r -> r.matching.size() >= 3).count();
  }

  static List<TestResult> findInstructions(String tests) {
    CPU testCPU = new CPU();
    List<TestResult> matches = new ArrayList<>();
    List<TestCase> testCases = Stream.of(tests.split("(\\r?\\n){2}")).map(TestCase::fromString).toList();

    for (final TestCase test : testCases) {
      matches.add(new TestResult(test.opcode, test.findMatches()));
    }

    return matches;
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
