package day12;

import java.io.IOException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.IntStream;
import lib.Input;

/**
 * This solution is a re-implementation of Eoin Davey's solution in Python.
 *
 * I didn't want to work through today's (not a fan of combinatorial problems), so I chose the
 * challenge of understanding and converting an existing Python solution to Java.
 */
public class Solution {

  public record FitState(int from, int groupSize) {
  }


  public record CountState(int from, int groupIdx) {
  }



  public record Schema(char[] pattern, int[] groups, Map<CountState, Long> countCache,
      Map<FitState, Boolean> fitCache) {
    public long count(int position, int groupIdx) {
      if (groupIdx >= groups.length) {
        return IntStream.range(position, pattern.length).allMatch(this::spaceOrDamaged) ? 1 : 0;
      }
      if (position >= pattern.length) {
        return 0;
      }

      var countState = new CountState(position, groupIdx);
      if (countCache.containsKey(countState)) {
        return countCache.get(countState);
      }

      long count = 0;
      if (fits(position, groups[groupIdx])) {
        count += count(position + groups[groupIdx] + 1, groupIdx + 1);
      }
      if (spaceOrDamaged(position)) {
        count += count(position + 1, groupIdx);
      }
      countCache.put(countState, count);
      return count;
    }

    boolean fits(int from, int splitSize) {
      var fitState = new FitState(from, splitSize);
      var fitResult = fitCache.get(fitState);
      if (fitResult != null) {
        return fitResult;
      }
      int to = from + splitSize;
      if (to > pattern.length) {
        return false;
      }

      boolean canPlace = IntStream.range(from, to).allMatch(this::springOrDamaged);
      boolean placeAfter = to == pattern.length || spaceOrDamaged(to);
      fitResult = canPlace && placeAfter;
      fitCache.put(fitState, fitResult);
      return fitResult;
    }

    boolean spaceOrDamaged(int position) {
      return pattern[position] == '.' || pattern[position] == '?';
    }

    boolean springOrDamaged(int position) {
      return pattern[position] == '#' || pattern[position] == '?';
    }

    public Schema unfold() {
      // TODO
      return this;
    }

    static Schema fromLine(String line) {
      var parts = line.split(" ");
      return new Schema(
          parts[0].toCharArray(),
          Arrays.stream(parts[1].split(",")).mapToInt(Integer::parseInt).toArray(),
          new HashMap<>(),
          new HashMap<>());
    }
  }

  public static void main(String[] args) throws IOException {
    List<Schema> originalSchemas = Input.lines("day12/input.txt").map(Schema::fromLine).toList();
    long partOne = originalSchemas.stream().mapToLong(schema -> schema.count(0, 0)).sum();
    long partTwo = 0;

    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo }" );
  }
}
