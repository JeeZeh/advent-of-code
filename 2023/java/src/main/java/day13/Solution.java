package day13;

import java.io.IOException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import org.apache.commons.lang3.ArrayUtils;
import lib.Input;

/**
 * This solution is a re-implementation of Eoin Davey's solution in Python.
 * <p>
 * I didn't want to work through today's (not a fan of combinatorial problems), so I chose the
 * challenge of understanding and converting an existing Python solution to Java.
 */
public class Solution {

  public record Pattern(boolean[][] )

  public static void main(String[] args) throws IOException {
    List<Schema> originalSchemas = Input.lines("day12/input.txt").map(Schema::fromLine).toList();
    long partOne = originalSchemas.stream().mapToLong(schema -> schema.count(0, 0)).sum();
    long partTwo = originalSchemas.stream().mapToLong(schema -> schema.unfold().count(0, 0)).sum();

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }
}
