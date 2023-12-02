package day01;

import java.io.IOException;
import java.util.List;
import java.util.Map;
import lib.Input;
public class Solution {
  static Map<String, Integer> numberMap = Map.of(
      "one", 1,
      "two", 2,
      "three", 3,
      "four", 4,
      "five", 5,
      "six", 6,
      "seven", 7,
      "eight", 8,
      "nine", 9
  );

  public static void main(String[] args) throws IOException {
    List<String> input = Input.lines("day01/input.txt").toList();
    System.out.println(partOne(input));
    System.out.println(partTwo(Input.lines("day01/input.txt").toList()));
  }

  public static int partOne(List<String> lines) {
    return lines.stream().map(str -> {
      var digits = str.replaceAll("[^0-9]", "");
      var numAsString = "" + digits.charAt(0) + digits.charAt(digits.length() - 1);
      return Integer.parseInt(numAsString);
    }).mapToInt(Integer::intValue).sum();
  }

  public static int partTwo(List<String> lines) {
    return lines.stream().map(str -> {
      int first = findFirstNumber(str);
      int last = findLastNumber(str);
      return Integer.parseInt("" + first + last);
    }).mapToInt(Integer::intValue).sum();
  }

  public static int findLastNumber(String line) {
    for (int c = line.length() - 1; c >= 0; c--) {
      for (final String n : numberMap.keySet()) {
        if (Character.isDigit(line.charAt(c))) {
          return Character.getNumericValue(line.charAt(c));
        }
        if (line.startsWith(n, c)) {
          return numberMap.get(n);
        }
      }
    }
    return -1;
  }

  public static int findFirstNumber(String line) {
    for (int c = 0; c < line.length(); c++) {
      for (final String n : numberMap.keySet()) {
        if (Character.isDigit(line.charAt(c))) {
          return Character.getNumericValue(line.charAt(c));
        }
        if (line.startsWith(n, c)) {
          return numberMap.get(n);
        }
      }
    }
    return -1;
  }
}
